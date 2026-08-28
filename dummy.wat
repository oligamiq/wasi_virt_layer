(module
  (import "wasi_snapshot_preview1" "environ_get" (func (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "_start")
    i32.const 0
    i32.const 0
    call 0
    drop
  )
)
