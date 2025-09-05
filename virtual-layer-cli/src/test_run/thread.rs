pub fn gen_threads_run(wasm_name: impl AsRef<str>, mem_size: (u64, u64)) -> String {
    let wasm_name = wasm_name.as_ref();
    let (init, max) = mem_size;

    let core = super::common::core(wasm_name);

    format!(
        r#"
{core}

// @ts-ignore
const root = await instantiate(undefined, {{
	"wasip1-vfs:host/virtual-file-system-wasip1-core": {{
        Wasip1: imports,
    }},
    "wasip1-vfs:host/virtual-file-system-wasip1-threads-import": {{
        Wasip1Threads: {{
        }},
    }},
}}, async (module, imports) => {{
    console.log("WebAssembly Module:", module);
    imports.env = {{
        memory: new WebAssembly.Memory({{initial:{init}, maximum:{max}, shared:true}})
    }};
    console.log("WebAssembly Imports:", imports);
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
        }}
    }},
}});
"#
    )
    .trim_start()
    .to_owned()
}
