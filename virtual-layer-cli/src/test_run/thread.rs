use std::io::Write as _;

pub fn gen_threads_run(
    wasm_name: impl AsRef<str>,
    mem_size: Vec<(u64, u64)>,
    out_dir: impl AsRef<str>,
) {
    let wasm_name = wasm_name.as_ref();

    let mut memories = String::new();
    for (i, (init, max)) in mem_size.iter().enumerate() {
        let i = if i > 0 {
            memories.push_str(",\n        ");
            i.to_string()
        } else {
            "".to_string()
        };
        memories.push_str(&format!(
            "memory{i}: new WebAssembly.Memory({{initial:{init}, maximum:{max}, shared:true}})"
        ));
    }

    [
        ("common.ts", common_ts()),
        ("inst.ts", custom_instantiate_ts()),
        ("test_run.ts", test_run_ts()),
        ("thread_spawn.ts", thread_spawn_ts()),
        ("tsconfig.json", tsconfig_json()),
        ("package.json", package_json()),
        ("worker_background_worker.ts", worker_background_worker_ts()),
        ("worker.ts", worker_ts()),
    ]
    .iter()
    .for_each(|(name, content)| {
        let out_dir = out_dir.as_ref();
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{out_dir}/{name}"))
            .expect("Failed to create file")
            .write_all(content.as_bytes())
            .expect("Failed to write file");
    });
}

fn common_ts() -> &'static str {
    r#"
import { Worker, isMainThread, parentPort } from "node:worker_threads";

class WorkerWrapper {
	worker: Worker;
	onmessage?: (event: unknown) => void;
	constructor(path: string) {
		this.worker = new Worker(path);
		this.worker.on("message", (event) => {
			this.onmessage?.(event);
		});
	}
	postMessage(msg: unknown) {
        console.log("WorkerWrapper posting message:", msg);
		this.worker.postMessage({
			data: msg,
		});
	}
	terminate() {
		return this.worker.terminate();
	}
}

const set_fake_worker = () => {
	if (isMainThread) {
		throw new Error("not main thread");
	}

	globalThis.postMessage = (msg: unknown) => {
        console.log("Posting message to worker:", msg);
		parentPort.postMessage({
			data: msg,
		});
	};
	parentPort.on("message", (event) => {
		// biome-ignore lint/suspicious/noExplicitAny: <explanation>
		(globalThis as any).onmessage?.(event);
	});

	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	(globalThis as any).Worker = WorkerWrapper;
};

export { set_fake_worker };
"#
    .trim_start()
}

fn custom_instantiate_ts() -> &'static str {
    r#"
import { type ImportObject, instantiate } from "./threads_vfs.js";

function snakeToCamel(snakeCaseString) {
	return snakeCaseString
		.toLowerCase()
		.replace(/_([a-z])/g, (match, letter) => letter.toUpperCase());
}

export const custom_instantiate = async (
	wasm_module: WebAssembly.Module,
	wasiImport: {
		[key: string]: (...args: unknown[]) => unknown;
	},
	wasiThreadImport: {
		"thread-spawn": (start_arg: number) => number;
	},
	memory: WebAssembly.Memory,
): Promise<WebAssembly.Instance> => {
	const imports = {};
	for (const key in wasiImport) {
		const inner_key = `${snakeToCamel(key)}Import`;
		imports[inner_key] = (...args) => {
			// console.log(`[WASI ${inner_key}]`, ...args);
			const ret = wasiImport[key](...args);
			// console.log(`[WASI ${inner_key}] ret`, ret);
			return ret;
		};
	}

	const threadSpawnImports = {
		threadSpawnImport: (start_arg) => {
			console.log("[WASI thread-spawn] start_arg", start_arg);
			const tid = wasiThreadImport["thread-spawn"](start_arg);
			console.log("[WASI thread-spawn] tid", tid);
			return tid;
		},
	};

	let inst: WebAssembly.Instance | undefined = undefined;

    console.log("custom_instantiate starting");

	const root = await instantiate(
		(_path) => {
			return wasm_module;
		}, // instantiate has default function if undefined
		{
			"wasip1-vfs:host/virtual-file-system-wasip1-core": {
				Wasip1: imports,
			},
			"wasip1-vfs:host/virtual-file-system-wasip1-threads-import": {
				Wasip1Threads: threadSpawnImports,
			},
		} as ImportObject,
		async (module, imports) => {
			console.log("WebAssembly Module:", module);
			imports.env = {
				memory,
			};
			console.log("WebAssembly Imports:", imports);

			inst = await WebAssembly.instantiate(module, imports);
			return inst;
		},
	);

	if (inst === undefined) {
		throw new Error("inst is not an instance");
	}
	inst = inst as WebAssembly.Instance;

    let tid = 0;

	const fake = {
		exports: {
			memory: inst.exports.memory as WebAssembly.Memory,
			_start: () => {
				// init only
				root.start();
				console.log("[WASI main]");
				root.init();
				console.log("[WASI main] done.");
			},
            wasi_thread_start: (arg) => {
                tid += 1;
                console.log("[WASI wasi_thread_start] tid", tid, "arg", arg);
                root.virtualFileSystemWasip1ThreadsExport.wasiThreadStart(tid, arg);
            }
		},
	};

    console.log("custom_instantiate done");

	return fake;
};
"#
    .trim_start()
}

fn test_run_ts() -> &'static str {
    r#"
// npx ts-node test_run.ts

import { Worker } from "node:worker_threads";
import {
	ConsoleStdout,
	File,
	OpenFile,
	PreopenDirectory,
	WASI,
} from "@bjorn3/browser_wasi_shim";
import { WASIFarm } from "@oligami/browser_wasi_shim-threads";

const farm = new WASIFarm(
	new OpenFile(new File([])), // stdin
	ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${msg}`)),
	ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${msg}`)),
	[],
);

const worker = new Worker("./worker.ts");

worker.postMessage({
	data: {
		wasi_ref: farm.get_ref(),
	},
});
"#
    .trim_start()
}

fn thread_spawn_ts() -> &'static str {
    r#"
import { thread_spawn_on_worker } from "@oligami/browser_wasi_shim-threads";
import { set_fake_worker } from "./common.ts";
import { custom_instantiate } from "./inst.ts";

set_fake_worker();

console.log("Thread spawn worker started");

globalThis.onmessage = (event) => {
	console.log("Thread spawn event received:", event.data);
	thread_spawn_on_worker(
		event.data,
		async (
			thread_spawn_wasm: WebAssembly.Module,
			imports: {
				env: { memory: WebAssembly.Memory };
				wasi: { "thread-spawn": (start_arg: number) => number };
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
				wasi_snapshot_preview1: { [key: string]: (...args: any[]) => unknown };
			},
		) => {
			return custom_instantiate(
				thread_spawn_wasm,
				imports.wasi_snapshot_preview1,
				imports.wasi,
				imports.env.memory,
			);
		},
	);
};
"#
    .trim_start()
}

fn tsconfig_json() -> &'static str {
    r#"
{
	"compilerOptions": {
		"target": "ESNext",
		"module": "NodeNext",
		"moduleResolution": "nodenext",
		"esModuleInterop": true,
		"skipLibCheck": true,
		"forceConsistentCasingInFileNames": true,
		"noEmit": true,
		"allowImportingTsExtensions": true
	},
	"include": ["**/*.ts"],
	"exclude": ["node_modules"]
}
"#
    .trim_start()
}

fn package_json() -> &'static str {
    r#"
{
	"scripts": {
		"test": "echo \"Error: no test specified\" && exit 1",
		"run": "ts-node test_run.ts"
	},
	"type": "module",
	"dependencies": {
		"@bjorn3/browser_wasi_shim": "^0.4.2",
		"@oligami/browser_wasi_shim-threads": "^0.1.5"
	},
    "devDependencies": {
        "ts-node": "^10.9.2"
    }
}
"#
    .trim_start()
}

fn worker_background_worker_ts() -> &'static str {
    r#"
// @ts-ignore
import run from "./node_modules/@oligami/browser_wasi_shim-threads/dist/worker_background_worker.min.js";

import { set_fake_worker } from "./common.ts";

set_fake_worker();

run();
"#
    .trim_start()
}

fn worker_ts() -> &'static str {
    r#"
import { readFileSync } from "node:fs";
import { WASIFarmAnimal } from "@oligami/browser_wasi_shim-threads";
import { set_fake_worker } from "./common.ts";
import { custom_instantiate } from "./inst.ts";

set_fake_worker();

globalThis.onmessage = async (message) => {
	const { wasi_ref } = message.data;

	// console.log("wasi_ref:", wasi_ref);

	const wasm_path = "./threads_vfs.core.wasm";
	const wasm = await WebAssembly.compile(
		readFileSync(wasm_path) as BufferSource,
	);

	const args = ["bin", "arg1", "arg2"];
	const env = ["FOO=bar"];

	// console.log("WASM:", wasm);

	const wasi = new WASIFarmAnimal(
		wasi_ref,
		args, // args
		env, // env
		{
			can_thread_spawn: true,
			thread_spawn_worker_url: "./thread_spawn.ts",
			thread_spawn_wasm: wasm,
			worker_background_worker_url: "./worker_background_worker.ts",
		},
	);

	// console.log("WASI:", wasi);

	await wasi.wait_worker_background_worker();

	// console.log("WASI2:", wasi);

	const root = await custom_instantiate(
		wasm,
		wasi.wasiImport,
		wasi.wasiThreadImport,
		wasi.get_share_memory(),
	);

	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	wasi.start(root as any);
};
"#
    .trim_start()
}
