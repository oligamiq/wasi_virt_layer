// deno run --allow-read --allow-env dist/test_run.ts

import { ConsoleStdout, File, OpenFile, PreopenDirectory, WASI } from "@bjorn3/browser_wasi_shim";

const wasm_path = "./target/wasm32-wasip1-threads/release/threads_vfs.wasm";

const args = ["bin", "arg1", "arg2"];
const env = ["FOO=bar"];
const fds = [
	new OpenFile(new File([])), // stdin
	ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${msg}`)),
	ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${msg}`)),
	new PreopenDirectory(".", new Map()),
];
const wasi = new WASI(args, env, fds);

let inst: WebAssembly.Instance | undefined = undefined;

function underscore_to_hyphen(snakeCaseString) {
    return snakeCaseString.toLowerCase().replace(/_([a-z])/g, (match, letter) => `-${letter}`);
}

const core_imports = {};
for (const key in wasi.wasiImport) {
    const inner_key = `[static]wasip1.${underscore_to_hyphen(key)}-import`;
    core_imports[inner_key] = (...args) => {
        // console.log(`[WASI ${inner_key}]`, ...args);
        const ret = wasi.wasiImport[key](...args);
        // console.log(`[WASI ${inner_key}] ret`, ret);
        return ret;
    }
}
console.log(core_imports);

import { readFileSync } from "node:fs";

const module = await WebAssembly.compile(readFileSync(wasm_path));
console.log("WebAssembly Module:", module);
const imports = {};
imports.env = {
    memory: new WebAssembly.Memory({initial:34, maximum:16384, shared:true})
};
imports["wasip1-vfs"] = {
    __wasip1_vfs_test_threads_memory_copy_to: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_copy_to", ...args);
    },
    __wasip1_vfs_test_threads_memory_copy_from: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_copy_to", ...args);
    },
    __wasip1_vfs_test_threads_memory_director: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_director", ...args);
    },
    __wasip1_vfs_self_wasi_thread_start: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_self_wasi_thread_start", ...args);
    },
    __wasip1_vfs_test_threads__start: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads__start", ...args);
    },
    __wasip1_vfs_test_threads_memory_trap: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_trap", ...args);
    },
    __wasip1_vfs_test_threads_wasi_thread_start: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads_wasi_thread_start", ...args);
    },
    __wasip1_vfs_test_threads_reset: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads_reset", ...args);
    },
    __wasip1_vfs_test_threads___main_void: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_test_threads___main_void", ...args);
    },
    __wasip1_vfs_wasi_thread_start_entry: (...args) => {
        console.log("[wasip1_vfs] __wasip1_vfs_wasi_thread_start_entry", ...args);
    }
}
imports["wasip1-vfs:host/virtual-file-system-wasip1-core"] = {
    ...core_imports,
}
imports["wasip1-vfs:host/virtual-file-system-wasip1-threads-import"] = {
    "[static]wasip1-threads.thread-spawn-import": (...args) => {
        console.log("[wasip1-threads] thread-spawn-import", ...args);
    }
};
imports["wasi_snapshot_preview1"] = {
    ...wasi.wasiImport
};
imports["wasi"] = {
    "thread-spawn": (...args) => {
        console.log("[wasi] thread-spawn", ...args);
    }
}
// console.log("WebAssembly Imports:", imports);
inst = await WebAssembly.instantiate(module, imports);

if (inst === undefined) {
    throw new Error("inst is not an instance");
}
inst = inst as WebAssembly.Instance;

wasi.start({
    exports: {
        memory: inst.exports.memory as WebAssembly.Memory,
        _start: () => {
            // init only
            inst.exports._start();
            console.log("[WASI main]");
            inst.exports.init();
            // console.log("[WASI init] done.");
            // main
            // inst.exports.main();
            console.log("[WASI main] done.");
        }
    },
});
