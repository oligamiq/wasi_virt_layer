pub(super) fn core(wasm_name: impl AsRef<str>) -> String {
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
