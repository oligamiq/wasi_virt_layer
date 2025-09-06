// deno run --allow-read --allow-env dist/test_run.ts

import { ConsoleStdout, File, OpenFile, PreopenDirectory, WASI } from "@bjorn3/browser_wasi_shim";

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

function snakeToCamel(snakeCaseString) {
    return snakeCaseString.toLowerCase().replace(/_([a-z])/g, (match, letter) => letter.toUpperCase());
}

const core_imports = {};
for (const key in wasi.wasiImport) {
    const inner_key = `[static]wasip1.${key.replaceAll('_', '-')}-import`;
    core_imports[inner_key] = (...args) => {
        // console.log(`[WASI ${inner_key}]`, ...args);
        const ret = wasi.wasiImport[key](...args);
        // console.log(`[WASI ${inner_key}] ret`, ret);
        return ret;
    }
}
console.log(core_imports);

const wasm_path = "target/wasm32-wasip1-threads/release/threads_vfs.opt.adjusted.opt.wasm";

const _fs = await import('node:fs/promises');
const wasm = await WebAssembly.compile(await _fs.readFile(wasm_path));

console.log("WebAssembly Module:", wasm);

const imports = {
    env: {
        memory: new WebAssembly.Memory({initial:17, maximum:17, shared:true})
    },
    "wasip1-vfs": {
        "__wasip1_vfs_test_threads_memory_copy_to": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_copy_to", ...args);
        },
        "__wasip1_vfs_test_threads_memory_copy_from": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_copy_to", ...args);
        },
        "__wasip1_vfs_test_threads_memory_director": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_director", ...args);
        },
        "__wasip1_vfs_test_threads__start": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads__start", ...args);
        },
        "__wasip1_vfs_test_threads_memory_trap": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads_memory_trap", ...args);
        },
        "__wasip1_vfs_test_threads_wasi_thread_start": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads_wasi_thread_start", ...args);
        },
        "__wasip1_vfs_test_threads_reset": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads_reset", ...args);
        },
        "__wasip1_vfs_test_threads___main_void": (...args) => {
            console.log("[wasip1_vfs] __wasip1_vfs_test_threads___main_void", ...args);
        }
    },
    "wasip1-vfs:host/virtual-file-system-wasip1-core": {
        ...core_imports,
    },
    "wasip1-vfs:host/virtual-file-system-wasip1-threads-import": {
        "[static]wasip1-threads.thread-spawn-import": (...args) => {
            console.log("[wasip1-threads] thread-spawn-import", ...args);
        }
    },
    "wasi_snapshot_preview1": core_imports,
    wasi: {

    }
};
console.log("WebAssembly Imports:", imports);
inst = await WebAssembly.instantiate(wasm, imports);

if (inst === undefined) {
    throw new Error("inst is not an instance");
}
inst = inst as WebAssembly.Instance;

wasi.start({
    exports: {
        memory: inst.exports.memory as WebAssembly.Memory,
        _start: () => {
            inst.exports.main();
        }
    },
});
