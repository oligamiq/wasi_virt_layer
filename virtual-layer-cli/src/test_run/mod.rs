use std::io::Write as _;

pub mod thread;

pub fn gen_test_run(wasm_name: impl AsRef<str>, out_dir: impl AsRef<str>) {
    let wasm_name = wasm_name.as_ref();

    let core = core(wasm_name);

    let code_base = format!(
        r#"
{core}

// @ts-ignore
const root = await instantiate(undefined, {{
	"wasip1-vfs:host/virtual-file-system-wasip1-core": {{
        Wasip1: imports,
    }}
}}, async (module, imports) => {{
    inst = await WebAssembly.instantiate(module, imports);
    return inst;
}});

if (inst === undefined) {{
    throw new Error("inst is not an instance");
}}
inst = inst as WebAssembly.Instance;

wasi.start({{
    exports: {{
        memory: inst.exports.memory as WebAssembly.Memory,
        _start: () => {{
            // init only
            console.log("[WASI main]");
            root.main();
            console.log("[WASI main] done.");
        }}
    }},
}});
"#
    );

    let code = code_base.trim_start();

    let out_dir = out_dir.as_ref();
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{out_dir}/test_run.ts"))
        .expect("Failed to create file")
        .write_all(code.as_bytes())
        .expect("Failed to write file");
}

fn core(wasm_name: impl AsRef<str>) -> String {
    let wasm_name = wasm_name.as_ref();

    format!(r#"
// deno run --allow-read --allow-env dist/test_run.ts

import {{ ConsoleStdout, File, OpenFile, PreopenDirectory, WASI }} from "@bjorn3/browser_wasi_shim";

import {{ instantiate }} from "./{wasm_name}.js";

const args = ["bin", "arg1", "arg2"];
const env = ["FOO=bar"];
const fds = [
	new OpenFile(new File([])), // stdin
	ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${{msg}}`)),
	ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${{msg}}`)),
	new PreopenDirectory(".", new Map()),
];
const wasi = new WASI(args, env, fds);

let inst: WebAssembly.Instance | undefined = undefined;

function snakeToCamel(snakeCaseString) {{
    return snakeCaseString.toLowerCase().replace(/_([a-z])/g, (match, letter) => letter.toUpperCase());
}}

const imports = {{}};
for (const key in wasi.wasiImport) {{
    const inner_key = `${{snakeToCamel(key)}}Import`;
    imports[inner_key] = (...args) => {{
        // console.log(`[WASI ${{inner_key}}]`, ...args);
        const ret = wasi.wasiImport[key](...args);
        // console.log(`[WASI ${{inner_key}}] ret`, ret);
        return ret;
    }}
}}
console.log(imports);
"#
    )
    .trim_start()
    .to_owned()
}
