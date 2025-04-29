pub const TEST_RUN: &str = r#"
// deno run --allow-read dist/test_run.ts

import { ConsoleStdout, File, OpenFile, PreopenDirectory, WASI } from "@bjorn3/browser_wasi_shim";

import { instantiate } from "./example_vfs.js";

const args = ["bin", "arg1", "arg2"];
const env = ["FOO=bar"];
const fds = [
	new OpenFile(new File([])), // stdin
	ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${msg}`)),
	ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${msg}`)),
	new PreopenDirectory(".", []),
];
const wasi = new WASI(args, env, fds);

let inst: WebAssembly.Instance;

function snakeToCamel(snakeCaseString) {
    return snakeCaseString.toLowerCase().replace(/_([a-z])/g, (match, letter) => letter.toUpperCase());
}

const imports = {};
for (const key in wasi.wasiImport) {
    const inner_key = `${snakeToCamel(key)}Import`;
    imports[inner_key] = (...args) => {
        // console.log(`[WASI ${inner_key}]`, ...args);
        const ret = wasi.wasiImport[key](...args);
        // console.log(`[WASI ${inner_key}] ret`, ret);
        return ret;
    }
}
console.log(imports);
const root = await instantiate(undefined, {
	wasip1: {
        default: imports
    },
}, async (module, imports) => {
    inst = await WebAssembly.instantiate(module, imports);
    return inst;
});

wasi.start({
    ...inst,
    exports: {
        ...inst.exports,
        _start: () => {
            // init only
            console.log("[WASI main]");
            inst.exports.main();
            console.log("[WASI root.world()]");
            root.world();
            console.log('[WASI root.addEnv("RUST_BACKTRACE=1")');
            root.addEnv("RUST_BACKTRACE=1");
            console.log('[WASI root.getEnvs()');
            console.log(root.getEnvs());
            console.log("[WASI main]");
            console.log("rust have virtual env layer so envs are no changed");
            inst.exports.main();
        }
    },
});

"#;
