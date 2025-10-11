use std::io::Write as _;

pub fn gen_threads_run(
    wasm_name: impl AsRef<str>,
    mem_size: Box<[(u64, u64)]>,
    out_dir: impl AsRef<str>,
) {
    let wasm_name = wasm_name.as_ref();

    [
        ("common.ts", common_ts()),
        ("inst.ts", &custom_instantiate_ts(wasm_name)),
        ("test_run.ts", test_run_ts()),
        ("thread_spawn.ts", thread_spawn_ts()),
        ("tsconfig.json", tsconfig_json()),
        ("package.json", package_json()),
        ("worker_background_worker.ts", worker_background_worker_ts()),
        ("worker.ts", &worker_ts(wasm_name, &mem_size)),
        ("vite.config.ts", vite_config_ts()),
        ("index.html", index_html()),
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
// biome-ignore lint/suspicious/noExplicitAny: <explanation>
let _worker: any = null;
const set_fake_worker = async () => {
	if (
		typeof process !== "undefined" &&
		process.versions &&
		process.versions.node
	) {
		_worker = _worker || (await import("node:worker_threads"));
		const { Worker, isMainThread, parentPort } = _worker;

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
				this.worker.postMessage({
					data: msg,
				});
			}
			terminate() {
				return this.worker.terminate();
			}
		}

		if (isMainThread) {
			throw new Error("not main thread");
		}

		globalThis.postMessage = (msg: unknown) => {
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
	}
};

export { set_fake_worker };
"#
    .trim()
}

fn custom_instantiate_ts(wasm_name: &str) -> String {
    let pre = format!(
        r#"
import {{ type ImportObject, instantiate }} from "./{wasm_name}.js";
"#
    );

    let post = r#"
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
	memory: {
		[key: string]: WebAssembly.Memory;
	},
): Promise<WebAssembly.Instance> => {
	const imports = {};
	for (const key in wasiImport) {
		const inner_key = `${snakeToCamel(key)}Import`;
		imports[inner_key] = wasiImport[key];
	}

	const threadSpawnImports = {
		threadSpawnImport: (start_arg) => {
			const tid = wasiThreadImport["thread-spawn"](start_arg);
			return tid;
		},
	};

	let inst: WebAssembly.Instance | undefined = undefined;

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
			imports.env = {
				...memory,
			};

			inst = await WebAssembly.instantiate(module, imports);
			return inst;
		},
	);

	if (inst === undefined) {
		throw new Error("inst is not an instance");
	}
	inst = inst as WebAssembly.Instance;

	const fake = {
		exports: {
			memory: inst.exports.memory as WebAssembly.Memory,
			_start: () => {
				// init only
                root.start();
				console.log("[WASI main]");
				root.main();
				console.log("[WASI main] done.");
			},
			wasi_thread_start: (tid, arg) => {
				console.log("[WASI wasi_thread_start] tid", tid, "arg", arg);
				root.virtualFileSystemWasip1ThreadsExport.wasiThreadStart(tid, arg);
			},
		},
	};

	return fake;
};
"#;

    format!("{}{}", pre.trim(), post.trim())
}

fn test_run_ts() -> &'static str {
    r#"
// npx ts-node test_run.ts

import { ConsoleStdout, Fd, File, OpenFile } from "@bjorn3/browser_wasi_shim";
import { WASIFarm, wait_async_polyfill } from "@oligami/browser_wasi_shim-threads";

const isNode =
	typeof process !== "undefined" && process.versions && process.versions.node;

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
let _worker: any = null;

let farm: WASIFarm;
if (!isNode) {
    await import("@xterm/xterm/css/xterm.css");
    const { FitAddon } = await import("xterm-addon-fit");
    const { Terminal } = await import("@xterm/xterm");

    wait_async_polyfill();

	const term = new Terminal({
		convertEol: true,
	});
	const terminalElement = document.getElementById("terminal");

	if (!terminalElement) {
		throw new Error("No terminal element found");
	}

	term.open(terminalElement);

	const fitAddon = new FitAddon();
	term.loadAddon(fitAddon);
	fitAddon.fit();

	class XtermStdio extends Fd {
		term: Terminal;

		constructor(term: Terminal) {
			super();
			this.term = term;
		}
		fd_write(data: Uint8Array) /*: {ret: number, nwritten: number}*/ {
			const str = new TextDecoder().decode(data);
			this.term.write(str);
			console.log(str);
			return { ret: 0, nwritten: data.byteLength };
		}
	}

	farm = new WASIFarm(
		new XtermStdio(term),
		new XtermStdio(term),
		new XtermStdio(term),
		[],
	);

	const worker = new Worker("./worker.ts", { type: "module" });

	worker.postMessage({
		wasi_ref: farm.get_ref(),
	});
} else {
	_worker = _worker || (await import("node:worker_threads"));

	farm = new WASIFarm(
		new OpenFile(new File([])), // stdin
		ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${msg}`)),
		ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${msg}`)),
		[],
	);

	const worker = new _worker.Worker("./worker.ts");

	worker.postMessage({
		data: {
			wasi_ref: farm.get_ref(),
		},
	});
}
"#
    .trim()
}

fn thread_spawn_ts() -> &'static str {
    r#"
import { thread_spawn_on_worker } from "@oligami/browser_wasi_shim-threads";
import { set_fake_worker } from "./common.ts";
import { custom_instantiate } from "./inst.ts";

await set_fake_worker();

globalThis.onmessage = (event) => {
	thread_spawn_on_worker(
		event.data,
		async (
			thread_spawn_wasm: WebAssembly.Module,
			imports: {
				env: {
					[key: string]: WebAssembly.Memory;
				};
				wasi: { "thread-spawn": (start_arg: number) => number };
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
				wasi_snapshot_preview1: { [key: string]: (...args: any[]) => unknown };
			},
		) => {
			return custom_instantiate(
				thread_spawn_wasm,
				imports.wasi_snapshot_preview1,
				imports.wasi,
				imports.env,
			);
		},
	);
};
"#
    .trim()
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
    .trim()
}

fn package_json() -> &'static str {
    r#"
{
	"scripts": {
		"run": "ts-node test_run.ts",
		"dev": "vite"
	},
	"type": "module",
	"dependencies": {
		"@bjorn3/browser_wasi_shim": "^0.4.2",
		"@oligami/browser_wasi_shim-threads": "^0.2.2",
		"@xterm/xterm": "^5.5",
		"xterm-addon-fit": "^0.8.0"
	},
	"devDependencies": {
		"ts-node": "^10.9.2",
		"vite": "^7.1.7"
	}
}
"#
    .trim()
}

fn worker_background_worker_ts() -> &'static str {
    r#"
import { wait_async_polyfill } from "@oligami/browser_wasi_shim-threads";
// @ts-ignore
import run from "./node_modules/@oligami/browser_wasi_shim-threads/dist/worker_background_worker.min.js";

import { set_fake_worker } from "./common.ts";

await set_fake_worker();

wait_async_polyfill();

run();
"#
    .trim()
}

fn worker_ts(wasm_name: &str, mem_size: &[(u64, u64)]) -> String {
    let mut memories = String::new();
    for (i, (init, max)) in mem_size.iter().enumerate() {
        let i = if i > 0 {
            memories.push_str("\n");
            i.to_string()
        } else {
            "".to_string()
        };
        memories.push_str(&{
            let str = format!(
                r#"
                memory{i}: new WebAssembly.Memory({{
                    initial:{init},
                    maximum:{max},
                    shared:true,
                }}),"#
            );
            format!("                {}", str.trim())
        });
    }

    format!(
        r#"
import {{ WASIFarmAnimal }} from "@oligami/browser_wasi_shim-threads";
import {{ set_fake_worker }} from "./common.ts";
import {{ custom_instantiate }} from "./inst.ts";

await set_fake_worker();

const isNode =
    typeof process !== "undefined" && process.versions && process.versions.node;
// biome-ignore lint/suspicious/noExplicitAny: <explanation>
let _fs: any = null;
async function fetchCompile(url) {{
    if (isNode) {{
        _fs = _fs || (await import("node:fs/promises"));
        return WebAssembly.compile(await _fs.readFile(url));
    }}
    return fetch(url).then(WebAssembly.compileStreaming);
}}

globalThis.onmessage = async (message) => {{
	const {{ wasi_ref }} = message.data;

	const wasm_path = "./{wasm_name}.core.wasm";
	const wasm = await fetchCompile(wasm_path);

	const args = ["bin", "arg1", "arg2"];
	const env = ["FOO=bar"];

	const wasi = new WASIFarmAnimal(
		wasi_ref,
		args, // args
		env, // env
		{{
			can_thread_spawn: true,
			thread_spawn_worker_url: "./thread_spawn.ts",
			thread_spawn_wasm: wasm,
			worker_background_worker_url: "./worker_background_worker.ts",
            share_memory: {{
{memories}
            }},
        }},
	);

	await wasi.wait_worker_background_worker();

	const root = await custom_instantiate(
		wasm,
		wasi.wasiImport,
		wasi.wasiThreadImport,
		wasi.get_share_memory(),
	);

	// biome-ignore lint/suspicious/noExplicitAny: <explanation>
	wasi.start(root as any);

    process?.exit(0);
}};
"#
    )
    .trim()
    .to_string()
}

fn vite_config_ts() -> &'static str {
    r#"
import { defineConfig } from "vite";

export default defineConfig({
	server: {
		headers: {
			"Cross-Origin-Embedder-Policy": "require-corp",
			"Cross-Origin-Opener-Policy": "same-origin",
		},
	},
});
    "#
    .trim()
}

fn index_html() -> &'static str {
    r#"
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">

<div id="terminal"></div>

<script type="module" src="./test_run.ts"></script>
</body>
</html>
    "#
    .trim()
}
