use std::io::Write as _;

pub mod common;
pub mod thread;

pub fn gen_test_run(wasm_name: impl AsRef<str>, out_dir: impl AsRef<str>) {
    let wasm_name = wasm_name.as_ref();

    let core = common::core(wasm_name);

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
            root.start();
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
