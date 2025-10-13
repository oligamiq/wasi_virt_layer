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

    let html = html(wasm_name);
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{out_dir}/test_run.html"))
        .expect("Failed to create file")
        .write_all(html.as_bytes())
        .expect("Failed to write file");
}

fn core(wasm_name: impl AsRef<str>) -> String {
    let wasm_name = wasm_name.as_ref();

    const PRE: &str = r#"
// deno run --allow-read --allow-env dist/test_run.ts

import { ConsoleStdout, File, OpenFile, PreopenDirectory, WASI } from "@bjorn3/browser_wasi_shim";
    "#;

    let middle = format!(r#"import {{ instantiate }} from "./{wasm_name}.js";"#);

    const END: &str = r#"
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
"#;

    PRE.trim_start().to_owned() + &middle + END
}

fn html(wasm_name: impl AsRef<str>) -> String {
    let wasm_name = wasm_name.as_ref();

    const PRE: &str = r#"
    <!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>WASI on the Web (non-threads, worker-run)</title>
  <style>
    body { font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif; margin: 0; background: #0b1020; color: #eef2ff; }
    header { padding: 16px 20px; border-bottom: 1px solid #20263a; background: #0b1020; position: sticky; top: 0; }
    h1 { margin: 0; font-size: 18px; letter-spacing: .3px; }
    main { padding: 20px; display: grid; gap: 16px; }
    .row { display: flex; gap: 12px; align-items: center; flex-wrap: wrap; }
    button { padding: 10px 14px; border-radius: 10px; border: 0; background: #4c6fff; color: white; font-weight: 600; cursor: pointer; }
    button:disabled { opacity: .6; cursor: not-allowed; }
    .badge { padding: 4px 8px; border-radius: 999px; background: #17203a; color: #9fb2ff; font-size: 12px; }
    .console { background: #0e1530; border: 1px solid #1b2344; border-radius: 12px; min-height: 220px; padding: 12px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; font-size: 13px; overflow: auto; }
    .log { white-space: pre-wrap; margin: 0; }
  </style>
</head>
<body>
  <header>
    <h1>WASI on the Web (non-threads, worker-run)</h1>
  </header>
  <main>
    <div class="row">
      <button id="runBtn">Run WASI</button>
    </div>
    <section class="console" id="console"></section>
  </main>

  <script type="module">
    const elConsole = document.getElementById('console');
    const runBtn = document.getElementById('runBtn');

    const append = (text, color) => {
      const line = document.createElement('div');
      if (color) line.style.color = color;
      line.textContent = text;
      elConsole.appendChild(line);
      elConsole.scrollTop = elConsole.scrollHeight;
    };
    "#;

    let middle = format!(
        r#"
    const component_url = new URL('./{wasm_name}.js', import.meta.url);
"#
    );

    const END: &str = r#"
    const workerCode = `
import { ConsoleStdout, File, OpenFile, PreopenDirectory, WASI } from "https://esm.sh/@bjorn3/browser_wasi_shim";
import { instantiate } from "${component_url}";

const postLog = (type, msg) => postMessage({ type, msg });

function snakeToCamel(s) {
  return s.toLowerCase().replace(/_([a-z])/g, (_, l) => l.toUpperCase());
}

self.onmessage = async (ev) => {
  const data = ev.data || {};
  if (data.cmd !== 'run') return;

  try {
    postLog('status', { running: true });

    const args = data.args || ["bin", "arg1", "arg2"];
    const env = data.env || ["FOO=bar"];
    const fds = [
      new OpenFile(new File([])), // stdin
      ConsoleStdout.lineBuffered((msg) => postLog('stdout', msg)),
      ConsoleStdout.lineBuffered((msg) => postLog('stderr', msg)),
      new PreopenDirectory('.', new Map()),
    ];

    const wasi = new WASI(args, env, fds);

    let inst;
    const imports = {};
    for (const key in wasi.wasiImport) {
      const innerKey = \`\${snakeToCamel(key)}Import\`;
      imports[innerKey] = (...a) => wasi.wasiImport[key](...a);
    }

    postLog('log', 'WASI imports prepared.');

    const root = await instantiate(undefined, {
      "wasip1-vfs:host/virtual-file-system-wasip1-core": { Wasip1: imports },
    }, async (module, importsObj) => {
      inst = await WebAssembly.instantiate(module, importsObj);
      return inst;
    });

    if (!inst) throw new Error("inst is not an instance");

    wasi.start({
      exports: {
        memory: inst.exports.memory,
        _start: () => {
          postLog('log', '[WASI main]');
          root.main();
          postLog('log', '[WASI main] done.');
        },
      },
    });

    postLog('done', 'WASI execution finished.');
  } catch (e) {
    postLog('error', e && e.message ? e.message : String(e));
  } finally {
    postLog('status', { running: false });
  }
};

postLog('ready', 'Worker initialized.');
`;

    // Create the Worker dynamically from a Blob
    const blob = new Blob([workerCode], { type: 'application/javascript' });
    const worker = new Worker(URL.createObjectURL(blob), { type: 'module' });

    let ready = false;
    let running = false;

    worker.onmessage = (ev) => {
      const { type, msg } = ev.data || {};
      switch (type) {
        case 'ready':
          ready = true;
          append('Worker ready.');
          break;
        case 'stdout':
          append('[WASI stdout] ' + msg);
          break;
        case 'stderr':
          append('[WASI stderr] ' + msg, '#ffdb89');
          break;
        case 'log':
          append('[log] ' + msg);
          break;
        case 'error':
          append('[error] ' + msg, '#ff8a8a');
          break;
        case 'done':
          append('[done] ' + msg);
          break;
        case 'status':
          if (msg.running !== undefined) {
            running = msg.running;
            runBtn.disabled = running;
          }
          break;
      }
    };

    runBtn.addEventListener('click', () => {
      if (!ready) return append('Worker not ready.');
      if (running) return append('Worker already running.');
      append('Starting WASI (non-thread build)...');
      worker.postMessage({ cmd: 'run' });
    });
  </script>
</body>
</html>
    "#;

    PRE.trim_start().to_owned() + &middle + END
}
