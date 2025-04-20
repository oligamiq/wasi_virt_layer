import Wasip1 from 'wasip1';

const fetchCompile = url => fetch(url).then(WebAssembly.compileStreaming);

const instantiateCore = WebAssembly.instantiate;

function throwUninitialized() {
  throw new TypeError('Wasm uninitialized use `await $init` first');
}

function toInt32(val) {
  return val >> 0;
}



function trampoline0(arg0, arg1, arg2, arg3) {
  const ret = Wasip1.fdWriteImport(arg0, arg1, arg2, arg3);
  return toInt32(ret);
}


function trampoline1(arg0, arg1) {
  const ret = Wasip1.environGetImport(arg0, arg1);
  return toInt32(ret);
}


function trampoline2(arg0, arg1) {
  const ret = Wasip1.environSizesGetImport(arg0, arg1);
  return toInt32(ret);
}


function trampoline3(arg0) {
  Wasip1.procExitImport(arg0);
}

let exports0;
let exports0Run;

function run() {
  if (!_initialized) throwUninitialized();
  exports0Run();
}
let exports0FdWriteExport;

function fdWriteExport(arg0, arg1, arg2, arg3) {
  if (!_initialized) throwUninitialized();
  const ret = exports0FdWriteExport(toInt32(arg0), toInt32(arg1), toInt32(arg2), toInt32(arg3));
  return ret;
}
let exports0EnvironGetExport;

function environSizesGetExport(arg0, arg1) {
  if (!_initialized) throwUninitialized();
  const ret = exports0EnvironGetExport(toInt32(arg0), toInt32(arg1));
  return ret;
}

function environGetExport(arg0, arg1) {
  if (!_initialized) throwUninitialized();
  const ret = exports0EnvironGetExport(toInt32(arg0), toInt32(arg1));
  return ret;
}
let exports0ProcExitExport;

function procExitExport(arg0) {
  if (!_initialized) throwUninitialized();
  exports0ProcExitExport(toInt32(arg0));
}

let _initialized = false;
export const $init = (() => {
  let gen = (function* init () {
    const module0 = fetchCompile(new URL('./wasip1_vfs.opt.adjusted.opt.component.core.wasm', import.meta.url));
    ({ exports: exports0 } = yield instantiateCore(yield module0, {
      $root: {
        '[static]wasip1.environ-get-import': trampoline1,
        '[static]wasip1.environ-sizes-get-import': trampoline2,
        '[static]wasip1.fd-write-import': trampoline0,
        '[static]wasip1.proc-exit-import': trampoline3,
      },
    }));
    _initialized = true;
    exports0Run = exports0.run;
    exports0FdWriteExport = exports0['fd-write-export'];
    exports0EnvironGetExport = exports0['environ-get-export'];
    exports0ProcExitExport = exports0['proc-exit-export'];
  })();
  let promise, resolve, reject;
  function runNext (value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject = _reject));
      value.then(runNext, reject);
    }
    catch (e) {
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
})();

export { environGetExport, environSizesGetExport, fdWriteExport, procExitExport, run,  }