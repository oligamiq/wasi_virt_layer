import Wasip1 from 'wasip1';

const fetchCompile = url => fetch(url).then(WebAssembly.compileStreaming);

const instantiateCore = WebAssembly.instantiate;

function toInt32(val) {
  return val >> 0;
}

function toResultString(obj) {
  return JSON.stringify(obj, (_, v) => {
    if (v && Object.getPrototypeOf(v) === Uint8Array.prototype) {
      return `[${v[Symbol.toStringTag]} (${v.byteLength})]`;
    } else if (typeof v === 'bigint') {
      return v.toString();
    }
    return v;
  });
}



function trampoline0(arg0, arg1, arg2, arg3) {
  console.error(`[module="[static]wasip1.fd-write-import", function="[static]wasip1.fd-write-import"] call fd=${arguments[0]}, iovs=${arguments[1]}, iovs-len=${arguments[2]}, written=${arguments[3]}`);
  const ret = Wasip1.fdWriteImport(arg0, arg1, arg2, arg3);
  console.error(`[module="[static]wasip1.fd-write-import", function="[static]wasip1.fd-write-import"] return result=${toResultString(ret)}`);
  return toInt32(ret);
}


function trampoline1(arg0, arg1) {
  console.error(`[module="[static]wasip1.environ-get-import", function="[static]wasip1.environ-get-import"] call environ=${arguments[0]}, environ-buf=${arguments[1]}`);
  const ret = Wasip1.environGetImport(arg0, arg1);
  console.error(`[module="[static]wasip1.environ-get-import", function="[static]wasip1.environ-get-import"] return result=${toResultString(ret)}`);
  return toInt32(ret);
}


function trampoline2(arg0) {
  console.error(`[module="[static]wasip1.proc-exit-import", function="[static]wasip1.proc-exit-import"] call code=${arguments[0]}`);
  Wasip1.procExitImport(arg0);
  console.error(`[module="[static]wasip1.proc-exit-import", function="[static]wasip1.proc-exit-import"] return `);
}

let exports0;
let exports0World;

function world() {
  console.error(`[module="world", function="world"] call `);
  exports0World();
  console.error(`[module="world", function="world"] return `);
}

const $init = (() => {
  let gen = (function* init () {
    const module0 = fetchCompile(new URL('./wasip1_vfs.core.wasm', import.meta.url));
    ({ exports: exports0 } = yield instantiateCore(yield module0, {
      $root: {
        '[static]wasip1.environ-get-import': trampoline1,
        '[static]wasip1.fd-write-import': trampoline0,
        '[static]wasip1.proc-exit-import': trampoline2,
      },
    }));
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

await $init;

export { world,  }