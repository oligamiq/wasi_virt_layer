;; Minimal C/C++-like WASI module for testing.
;; Has `_start` and `memory` exports but no `__main_void`.
;; Imports standard WASI functions to exercise the ABI connection pipeline.

(module
  ;; WASI imports (subset matching what C/C++ modules typically use)
  (import "wasi_snapshot_preview1" "proc_exit"
    (func $proc_exit (param i32)))

  ;; Memory
  (memory (export "memory") 1)

  ;; _start: exit gracefully
  (func $start (export "_start")
    ;; proc_exit(0)
    (call $proc_exit (i32.const 0))
  )
)
