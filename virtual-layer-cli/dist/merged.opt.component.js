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
let exports0World;

function world() {
  if (!_initialized) throwUninitialized();
  exports0World();
}

let _initialized = false;
export const $init = (() => {
  let gen = (function* init () {
    const module0 = fetchCompile(new URL('./merged.opt.component.core.wasm', import.meta.url));
    ({ exports: exports0 } = yield instantiateCore(yield module0, {
      $root: {
        '[static]wasip1.environ-get-import': trampoline1,
        '[static]wasip1.environ-sizes-get-import': trampoline2,
        '[static]wasip1.fd-write-import': trampoline0,
        '[static]wasip1.proc-exit-import': trampoline3,
      },
    }));
    _initialized = true;
    exports0World = exports0.world;
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

export { world,  }