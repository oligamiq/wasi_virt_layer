(module $test_unreachable_target-b19f68c2c770e4ce.wasm
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;8;) (func (result i32)))
  (type (;9;) (func (param i32 i32 i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32 i32)))
  (type (;11;) (func (param i32 i32 i32 i32 i32)))
  (type (;12;) (func (param i32 i32 i32 i32 i32) (result i32)))
  (type (;13;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;14;) (func (param i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;15;) (func (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "environ_get" (func $__imported_wasi_snapshot_preview1_environ_get (;0;) (type 5)))
  (import "wasi_snapshot_preview1" "environ_sizes_get" (func $__imported_wasi_snapshot_preview1_environ_sizes_get (;1;) (type 5)))
  (import "wasi_snapshot_preview1" "fd_write" (func $__imported_wasi_snapshot_preview1_fd_write (;2;) (type 7)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__imported_wasi_snapshot_preview1_proc_exit (;3;) (type 1)))
  (table (;0;) 69 69 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (;0;) (mut i32) i32.const 1048576)
  (global $GOT.data.internal.__memory_base (;1;) i32 i32.const 0)
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "__main_void" (func $__main_void))
  (elem (;0;) (i32.const 1) func $_ZN23test_unreachable_target4main17hf6fc32d42470be30E $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h580a1e18ac7cdb97E $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h269908780ac1e580E $_RNvNtCset5xJoy1xWQ_3std5alloc24default_alloc_error_hook $_RNvXs1i_NtCsdHhIpgkcIfN_4core3fmtReNtB6_7Display3fmtCset5xJoy1xWQ_3std $_RNvXs1i_NtCsdHhIpgkcIfN_4core3fmtRNtNtNtB8_5panic8location8LocationNtB6_7Display3fmtCset5xJoy1xWQ_3std $_RNvXs1j_NtCsdHhIpgkcIfN_4core3fmtQDNtNtB8_5panic12PanicPayloadEL_NtB6_7Display3fmtCset5xJoy1xWQ_3std $_RNvXsd_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3impyNtB9_7Display3fmt $_RNvXs8_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3impmNtB9_7Display3fmt $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_ $_RNvXNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB5_13BacktraceLock5printNtB2_16DisplayBacktraceNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt $_RNvXs9_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3implNtB9_7Display3fmt $_RNvXsq_NtCsi9YzqDQQz2q_5alloc6stringNtB5_6StringNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt $_RNvXs7_NtNtCset5xJoy1xWQ_3std2io5errorNtB5_5ErrorNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRhNtB6_5Debug3fmtCset5xJoy1xWQ_3std $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRbNtB6_5Debug3fmtCset5xJoy1xWQ_3std $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtBL_6cursor6CursorQShEEEBN_ $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterINtNtB4_6cursor6CursorQShEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtB7_6cursor6CursorQShEENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtB7_6cursor6CursorQShEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterINtNtCsi9YzqDQQz2q_5alloc3vec3VechEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtCsi9YzqDQQz2q_5alloc3vec3VechEENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtCsi9YzqDQQz2q_5alloc3vec3VechEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterNtNtB4_5stdio10StdoutLockENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtB7_5stdio10StdoutLockENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtB7_5stdio10StdoutLockENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterNtNtNtNtB6_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtNtNtB9_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtNtNtB9_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeNtNtCsi9YzqDQQz2q_5alloc6string6StringECset5xJoy1xWQ_3std $_RNvXsZ_NtCsi9YzqDQQz2q_5alloc6stringNtB5_6StringNtNtCsdHhIpgkcIfN_4core3fmt5Write9write_str $_RNvXsZ_NtCsi9YzqDQQz2q_5alloc6stringNtB5_6StringNtNtCsdHhIpgkcIfN_4core3fmt5Write10write_char $_RNvYNtNtCsi9YzqDQQz2q_5alloc6string6StringNtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtCset5xJoy1xWQ_3std $_RNvXs2_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt $_RNvXs1_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload8take_box $_RNvXs1_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload3get $_RNvXs1_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload6as_str $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeNtNvNtCset5xJoy1xWQ_3std9panicking13panic_handler19FormatStringPayloadEBM_ $_RNvXs0_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_19FormatStringPayloadNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt $_RNvXs_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload8take_box $_RNvXs_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload3get $_RNvYINtNvNtCset5xJoy1xWQ_3std9panicking11begin_panic7PayloadReENtNtCsdHhIpgkcIfN_4core5panic12PanicPayload6as_strB9_ $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write5write $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write14write_vectored $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write17is_write_vectored $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write5flush $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_allBa_ $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write18write_all_vectoredBa_ $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtCsi9YzqDQQz2q_5alloc3vec3VechEECset5xJoy1xWQ_3std $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write5writeB9_ $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write14write_vectoredB9_ $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write17is_write_vectoredB9_ $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write5flushB9_ $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write9write_allB9_ $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write18write_all_vectoredB9_ $_RNvYINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtNtCset5xJoy1xWQ_3std2io5Write9write_fmtBF_ $_RNvXs9_NtNtCsdHhIpgkcIfN_4core3str5errorNtB5_9Utf8ErrorNtNtB9_3fmt5Debug3fmt $_RNvXNtCsdHhIpgkcIfN_4core3anyReNtB2_3Any7type_idCset5xJoy1xWQ_3std $_RNvXsZ_NtNtCsdHhIpgkcIfN_4core3fmt3numjNtB7_5Debug3fmt $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCset5xJoy1xWQ_3std $_RNvXNtCsdHhIpgkcIfN_4core3anyNtNtCsi9YzqDQQz2q_5alloc6string6StringNtB2_3Any7type_idCset5xJoy1xWQ_3std $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRDNtB6_5DebugEL_Bx_3fmtB8_ $_RNvXs8_NtCsdHhIpgkcIfN_4core3fmtNtB5_9ArgumentsNtB5_7Display3fmt $_RNvXs1i_NtCsdHhIpgkcIfN_4core3fmtReNtB6_7Display3fmtB8_ $_RNvXsr_NtCsdHhIpgkcIfN_4core4cellNtB5_14BorrowMutErrorNtNtB7_3fmt7Display3fmt $_RNvXs0_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str $_RNvXs0_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write10write_char $_RNvYNtNtNtCsdHhIpgkcIfN_4core3fmt8builders10PadAdapterNtB6_5Write9write_fmtB8_)
  (func $__wasm_call_ctors (;4;) (type 0))
  (func $_start (;5;) (type 0)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        global.get $GOT.data.internal.__memory_base
        i32.const 1055848
        i32.add
        i32.load
        br_if 0 (;@2;)
        global.get $GOT.data.internal.__memory_base
        i32.const 1055848
        i32.add
        i32.const 1
        i32.store
        call $__wasi_init_tp
        call $__wasm_call_ctors
        call $__main_void
        local.set 0
        call $__wasm_call_dtors
        local.get 0
        br_if 1 (;@1;)
        return
      end
      unreachable
    end
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $_ZN23test_unreachable_target4main17hf6fc32d42470be30E (;6;) (type 0)
    i32.const 1048576
    i32.const 39
    call $_RNvNtNtCset5xJoy1xWQ_3std2io5stdio6__print
    i32.const 1048595
    i32.const 185
    i32.const 1050552
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h269908780ac1e580E (;7;) (type 6) (param i32) (result i32)
    local.get 0
    i32.load
    call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17he8840be8b24e5f1bE
    i32.const 0
  )
  (func $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17he8840be8b24e5f1bE (;8;) (type 1) (param i32)
    local.get 0
    call_indirect (type 0)
  )
  (func $_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h580a1e18ac7cdb97E (;9;) (type 6) (param i32) (result i32)
    local.get 0
    i32.load
    call $_ZN3std3sys9backtrace28__rust_begin_short_backtrace17he8840be8b24e5f1bE
    i32.const 0
  )
  (func $__main_void (;10;) (type 8) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    local.get 0
    i32.const 1
    i32.store offset=12
    local.get 0
    i32.const 12
    i32.add
    i32.const 1050568
    i32.const 0
    i32.const 0
    i32.const 0
    call $_RNvNtCset5xJoy1xWQ_3std2rt19lang_start_internal
    local.set 1
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc (;11;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_RNvCsfLfy6EI15iL_7___rustc11___rdl_alloc
    return
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc (;12;) (type 9) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_RNvCsfLfy6EI15iL_7___rustc13___rdl_dealloc
    return
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc14___rust_realloc (;13;) (type 7) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $_RNvCsfLfy6EI15iL_7___rustc13___rdl_realloc
    return
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2 (;14;) (type 0)
    return
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc18___rust_start_panic (;15;) (type 5) (param i32 i32) (result i32)
    call $_RNvCsfLfy6EI15iL_7___rustc12___rust_abort
    unreachable
  )
  (func $_RINvMNtNtCset5xJoy1xWQ_3std4sync9once_lockINtB3_8OnceLockINtNtB5_14reentrant_lock13ReentrantLockINtNtCsdHhIpgkcIfN_4core4cell7RefCellINtNtNtNtB7_2io8buffered10linewriter10LineWriterNtNtB2e_5stdio9StdoutRawEEEE10initializeNCINvB2_11get_or_initNCNvB2V_6stdout0E0zEB7_ (;16;) (type 0)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load8_u offset=1055928
            br_table 0 (;@4;) 0 (;@4;) 3 (;@1;) 1 (;@3;) 0 (;@4;)
          end
          i32.const 0
          i32.const 2
          i32.store8 offset=1055928
          call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
          i32.const 1024
          i32.const 1
          call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
          local.tee 0
          i32.eqz
          br_if 1 (;@2;)
          i32.const 0
          i32.const 3
          i32.store8 offset=1055928
          i32.const 0
          local.get 0
          i32.store offset=1055912
          i32.const 0
          i64.const 4398046511104
          i64.store offset=1055904
          i32.const 0
          i64.const 0
          i64.store offset=1055888
          i32.const 0
          i32.const 0
          i32.store8 offset=1055920
          i32.const 0
          i32.const 0
          i32.store offset=1055916
          i32.const 0
          i32.const 0
          i32.store8 offset=1055900
          i32.const 0
          i32.const 0
          i32.store offset=1055896
        end
        return
      end
      i32.const 1
      i32.const 1024
      call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
      unreachable
    end
    i32.const 1050650
    i32.const 113
    i32.const 1050592
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RINvMNtNtCset5xJoy1xWQ_3std4sync9once_lockINtB3_8OnceLockINtNtB5_14reentrant_lock13ReentrantLockINtNtCsdHhIpgkcIfN_4core4cell7RefCellINtNtNtNtB7_2io8buffered10linewriter10LineWriterNtNtB2e_5stdio9StdoutRawEEEE10initializeNCINvB2_11get_or_initNCNvB2V_7cleanup0E0zEB7_ (;17;) (type 1) (param i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1055928
          br_table 1 (;@2;) 1 (;@2;) 0 (;@3;) 2 (;@1;) 1 (;@2;)
        end
        i32.const 1050650
        i32.const 113
        i32.const 1050592
        call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
        unreachable
      end
      i32.const 0
      i32.const 3
      i32.store8 offset=1055928
      i32.const 0
      i64.const 1
      i64.store offset=1055912
      i32.const 0
      i64.const 0
      i64.store offset=1055904
      i32.const 0
      i64.const 0
      i64.store offset=1055888
      local.get 0
      i32.const 1
      i32.store8
      i32.const 0
      i32.const 0
      i32.store8 offset=1055920
      i32.const 0
      i32.const 0
      i32.store8 offset=1055900
      i32.const 0
      i32.const 0
      i32.store offset=1055896
    end
  )
  (func $_RINvNtCsdHhIpgkcIfN_4core9panicking13assert_failedbbECset5xJoy1xWQ_3std (;18;) (type 10) (param i32 i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 6
    global.set $__stack_pointer
    local.get 6
    local.get 2
    i32.store offset=12
    local.get 6
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 6
    i32.const 8
    i32.add
    i32.const 1050724
    local.get 6
    i32.const 12
    i32.add
    i32.const 1050724
    local.get 3
    local.get 4
    local.get 5
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking19assert_failed_inner
    unreachable
  )
  (func $_RINvMs0_NtNtNtNtCset5xJoy1xWQ_3std3sys4sync4once10no_threadsNtB6_4Once4callNCINvMs0_NtNtBe_4sync4onceNtB1k_4Once9call_onceNCNvNtBe_2rt7cleanup0E0EBe_ (;19;) (type 1) (param i32)
    (local i32 i32 i64 i64 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load8_u offset=1055968
                      br_table 0 (;@9;) 1 (;@8;) 3 (;@6;) 7 (;@2;) 0 (;@9;)
                    end
                    i32.const 0
                    i32.const 2
                    i32.store8 offset=1055968
                    local.get 0
                    i32.load8_u
                    local.set 2
                    local.get 0
                    i32.const 0
                    i32.store8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 2
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 0
                        i32.store8 offset=7
                        block ;; label = @11
                          i32.const 0
                          i32.load8_u offset=1055928
                          i32.const 3
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 1
                          i32.const 7
                          i32.add
                          call $_RINvMNtNtCset5xJoy1xWQ_3std4sync9once_lockINtB3_8OnceLockINtNtB5_14reentrant_lock13ReentrantLockINtNtCsdHhIpgkcIfN_4core4cell7RefCellINtNtNtNtB7_2io8buffered10linewriter10LineWriterNtNtB2e_5stdio9StdoutRawEEEE10initializeNCINvB2_11get_or_initNCNvB2V_7cleanup0E0zEB7_
                          local.get 1
                          i32.load8_u offset=7
                          br_if 8 (;@3;)
                        end
                        block ;; label = @11
                          i32.const 0
                          i64.load offset=1055952
                          local.tee 3
                          i64.const 0
                          i64.ne
                          br_if 0 (;@11;)
                          i32.const 0
                          i64.load offset=1055960
                          local.set 4
                          loop ;; label = @12
                            local.get 4
                            i64.const -1
                            i64.eq
                            br_if 5 (;@7;)
                            i32.const 0
                            local.get 4
                            i64.const 1
                            i64.add
                            local.tee 3
                            i32.const 0
                            i64.load offset=1055960
                            local.tee 5
                            local.get 5
                            local.get 4
                            i64.eq
                            local.tee 0
                            select
                            i64.store offset=1055960
                            local.get 5
                            local.set 4
                            local.get 0
                            i32.eqz
                            br_if 0 (;@12;)
                          end
                          i32.const 0
                          local.get 3
                          i64.store offset=1055952
                        end
                        local.get 3
                        i32.const 0
                        i64.load offset=1055888
                        i64.eq
                        br_if 1 (;@9;)
                        i32.const 0
                        i32.load8_u offset=1055900
                        local.set 2
                        i32.const 1
                        local.set 0
                        i32.const 0
                        i32.const 1
                        i32.store8 offset=1055900
                        local.get 2
                        br_if 7 (;@3;)
                        i32.const 0
                        local.get 3
                        i64.store offset=1055888
                        br 6 (;@4;)
                      end
                      i32.const 1051024
                      call $_RNvNtCsdHhIpgkcIfN_4core6option13unwrap_failed
                      unreachable
                    end
                    i32.const 0
                    i32.load offset=1055896
                    local.tee 0
                    i32.const -1
                    i32.ne
                    br_if 3 (;@5;)
                    br 5 (;@3;)
                  end
                  i32.const 1050608
                  i32.const 85
                  i32.const 1052416
                  call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
                  unreachable
                end
                call $_RNvNvMNtNtCset5xJoy1xWQ_3std6thread2idNtB4_8ThreadId3new9exhausted
                unreachable
              end
              i32.const 1050650
              i32.const 113
              i32.const 1052416
              call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
              unreachable
            end
            local.get 0
            i32.const 1
            i32.add
            local.set 0
          end
          i32.const 0
          local.get 0
          i32.store offset=1055896
          i32.const 0
          i32.load offset=1055904
          br_if 2 (;@1;)
          i32.const 0
          i32.const -1
          i32.store offset=1055904
          block ;; label = @4
            i32.const 0
            i32.load8_u offset=1055920
            br_if 0 (;@4;)
            local.get 1
            i32.const 8
            i32.add
            i32.const 1055908
            call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE9flush_bufBa_
            local.get 1
            i32.load8_u offset=8
            i32.const 3
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=12
            local.tee 0
            i32.load
            local.set 6
            block ;; label = @5
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 2
              i32.load
              local.tee 7
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 7
              call_indirect (type 1)
            end
            block ;; label = @5
              local.get 2
              i32.load offset=4
              local.tee 7
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 7
              local.get 2
              i32.load offset=8
              call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
            end
            local.get 0
            i32.const 12
            i32.const 4
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          end
          block ;; label = @4
            i32.const 0
            i32.load offset=1055908
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1055912
            local.get 0
            i32.const 1
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          end
          i32.const 0
          i64.const 4294967296
          i64.store offset=1055908 align=4
          i32.const 0
          i32.const 0
          i32.load offset=1055904
          i32.const 1
          i32.add
          i32.store offset=1055904
          i32.const 0
          i32.const 0
          i32.load offset=1055896
          i32.const -1
          i32.add
          local.tee 0
          i32.store offset=1055896
          i32.const 0
          i32.const 0
          i32.store8 offset=1055920
          i32.const 0
          i32.const 0
          i32.store offset=1055916
          local.get 0
          br_if 0 (;@3;)
          i32.const 0
          i64.const 0
          i64.store offset=1055888
          i32.const 0
          i32.const 0
          i32.store8 offset=1055900
        end
        i32.const 0
        i32.const 3
        i32.store8 offset=1055968
      end
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1052596
    call $_RNvNtCsdHhIpgkcIfN_4core4cell22panic_already_borrowed
    unreachable
  )
  (func $_RNvNvMNtNtCset5xJoy1xWQ_3std6thread2idNtB4_8ThreadId3new9exhausted (;20;) (type 0)
    i32.const 1052840
    i32.const 111
    i32.const 1052896
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE9flush_bufBa_ (;21;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i64 i32)
    block ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 2
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.store8
      return
    end
    local.get 1
    i32.load offset=4
    local.set 3
    i32.const 0
    local.set 4
    loop ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                i32.const 1
                local.get 3
                local.get 4
                i32.add
                local.tee 5
                local.get 2
                local.get 4
                i32.sub
                local.tee 6
                call $write
                local.tee 7
                i32.const -1
                i32.ne
                br_if 0 (;@6;)
                local.get 1
                i32.const 0
                i32.store8 offset=12
                i64.const 0
                local.set 8
                local.get 6
                local.set 7
                i32.const 0
                i32.load offset=1055972
                local.tee 9
                i32.const -8
                i32.add
                br_table 1 (;@5;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 2 (;@4;) 4 (;@2;) 2 (;@4;)
              end
              local.get 1
              i32.const 0
              i32.store8 offset=12
            end
            local.get 7
            br_if 1 (;@3;)
            i64.const 2
            local.set 8
            i32.const 1052400
            local.set 9
          end
          local.get 0
          local.get 9
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 8
          i64.or
          i64.store align=4
          block ;; label = @4
            local.get 4
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              local.get 5
              local.get 6
              memory.copy
            end
            local.get 1
            local.get 6
            i32.store offset=8
          end
          return
        end
        local.get 7
        local.get 4
        i32.add
        local.set 4
      end
      local.get 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
    end
    local.get 0
    i32.const 4
    i32.store8
    block ;; label = @1
      local.get 4
      local.get 2
      i32.gt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 0
      i32.store offset=8
      return
    end
    i32.const 0
    local.get 4
    local.get 2
    i32.const 1052572
    call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
    unreachable
  )
  (func $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_ (;22;) (type 2) (param i32 i32)
    (local i32 i32)
    block ;; label = @1
      local.get 0
      i32.const 255
      i32.and
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load
      local.set 2
      block ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 0
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        call_indirect (type 1)
      end
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        local.get 0
        i32.load offset=8
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 1
      i32.const 12
      i32.const 4
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
  )
  (func $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtCsi9YzqDQQz2q_5alloc3vec3VechEECset5xJoy1xWQ_3std (;23;) (type 1) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
  )
  (func $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtBL_6cursor6CursorQShEEEBN_ (;24;) (type 1) (param i32)
    (local i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.load8_u
      i32.const 3
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.tee 0
      i32.load
      local.set 1
      block ;; label = @2
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 2
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        call_indirect (type 1)
      end
      block ;; label = @2
        local.get 2
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        local.get 2
        i32.load offset=8
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 0
      i32.const 12
      i32.const 4
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
  )
  (func $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeNtNtCsi9YzqDQQz2q_5alloc6string6StringECset5xJoy1xWQ_3std (;25;) (type 1) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
  )
  (func $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeNtNvNtCset5xJoy1xWQ_3std9panicking13panic_handler19FormatStringPayloadEBM_ (;26;) (type 1) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      i32.const 1
      i32.lt_s
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 1
      i32.const 1
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
  )
  (func $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std (;27;) (type 11) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      local.get 2
      local.get 1
      i32.add
      local.tee 1
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 0
      call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 5
    i32.const 4
    i32.add
    local.get 0
    i32.load
    local.tee 2
    local.get 0
    i32.load offset=4
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    local.tee 2
    local.get 1
    local.get 2
    i32.gt_u
    select
    local.tee 2
    i32.const 8
    i32.const 4
    local.get 4
    i32.const 1
    i32.eq
    select
    local.tee 1
    local.get 2
    local.get 1
    i32.gt_u
    select
    local.tee 2
    local.get 3
    local.get 4
    call $_RNvMs4_NtCsi9YzqDQQz2q_5alloc7raw_vecNtB5_11RawVecInner11finish_growCset5xJoy1xWQ_3std
    block ;; label = @1
      local.get 5
      i32.load offset=4
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=8
      local.get 5
      i32.load offset=12
      call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 5
    i32.load offset=8
    local.set 4
    local.get 0
    local.get 2
    i32.store
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvMs4_NtCsi9YzqDQQz2q_5alloc7raw_vecNtB5_11RawVecInner11finish_growCset5xJoy1xWQ_3std (;28;) (type 10) (param i32 i32 i32 i32 i32 i32)
    (local i32 i32 i64)
    i32.const 1
    local.set 6
    i32.const 4
    local.set 7
    block ;; label = @1
      block ;; label = @2
        local.get 5
        i64.extend_i32_u
        local.get 3
        i64.extend_i32_u
        i64.mul
        local.tee 8
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 8
        i32.wrap_i64
        local.tee 3
        i32.const -2147483648
        local.get 4
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.get 5
              local.get 1
              i32.mul
              local.get 4
              local.get 3
              call $_RNvCsfLfy6EI15iL_7___rustc14___rust_realloc
              local.set 7
              br 1 (;@4;)
            end
            block ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              local.get 4
              local.set 7
              br 2 (;@3;)
            end
            call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 3
            local.get 4
            call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
            local.set 7
          end
          local.get 7
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 7
        i32.store offset=4
        i32.const 0
        local.set 6
      end
      i32.const 8
      local.set 7
    end
    local.get 0
    local.get 7
    i32.add
    local.get 3
    i32.store
    local.get 0
    local.get 6
    i32.store
  )
  (func $_RINvNtNtCset5xJoy1xWQ_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_5alloc8rust_oom0zEB6_ (;29;) (type 1) (param i32)
    local.get 0
    call $_RNCNvNtCset5xJoy1xWQ_3std5alloc8rust_oom0B5_
    unreachable
  )
  (func $_RNCNvNtCset5xJoy1xWQ_3std5alloc8rust_oom0B5_ (;30;) (type 1) (param i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.const 0
    i32.load offset=1055864
    local.tee 0
    i32.const 4
    local.get 0
    select
    call_indirect (type 2)
    call $_RNvNtCset5xJoy1xWQ_3std7process5abort
    unreachable
  )
  (func $_RINvNtNtCset5xJoy1xWQ_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_9panicking13panic_handler0zEB6_ (;31;) (type 1) (param i32)
    local.get 0
    call $_RNCNvNtCset5xJoy1xWQ_3std9panicking13panic_handler0B5_
    unreachable
  )
  (func $_RNCNvNtCset5xJoy1xWQ_3std9panicking13panic_handler0B5_ (;32;) (type 1) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 2
      i32.load offset=4
      local.tee 3
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.load
      local.set 2
      local.get 1
      local.get 3
      i32.const 1
      i32.shr_u
      i32.store offset=4
      local.get 1
      local.get 2
      i32.store
      local.get 1
      i32.const 1051176
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u offset=8
      local.get 0
      i32.load8_u offset=9
      call $_RNvNtCset5xJoy1xWQ_3std9panicking15panic_with_hook
      unreachable
    end
    local.get 1
    i32.const -2147483648
    i32.store
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 1051204
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.load8_u offset=8
    local.get 0
    i32.load8_u offset=9
    call $_RNvNtCset5xJoy1xWQ_3std9panicking15panic_with_hook
    unreachable
  )
  (func $_RINvNtNtNtCset5xJoy1xWQ_3std3sys7helpers14small_c_string24run_with_cstr_allocatingINtNtCsdHhIpgkcIfN_4core6option6OptionNtNtNtB8_3ffi6os_str8OsStringEEB8_ (;33;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    local.get 2
    call $_RNvXs_NvMs_NtNtCsi9YzqDQQz2q_5alloc3ffi5c_strNtB9_7CString3newRShNtB4_11SpecNewImpl13spec_new_impl
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.load
          local.tee 2
          i32.const -2147483648
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=8
          local.set 1
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.load offset=4
              local.tee 4
              call $getenv
              local.tee 5
              br_if 0 (;@5;)
              local.get 0
              i32.const -2147483648
              i32.store
              br 1 (;@4;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 5
                call $strlen
                local.tee 2
                br_if 0 (;@6;)
                i32.const 1
                local.set 6
                br 1 (;@5;)
              end
              call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
              local.get 2
              i32.const 1
              call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
              local.tee 6
              i32.eqz
              br_if 4 (;@1;)
              local.get 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 5
              local.get 2
              memory.copy
            end
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            local.get 6
            i32.store offset=4
            local.get 0
            local.get 2
            i32.store
          end
          local.get 4
          i32.const 0
          i32.store8
          local.get 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 4
          local.get 1
          i32.const 1
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          br 1 (;@2;)
        end
        local.get 0
        i32.const -2147483647
        i32.store
        local.get 0
        i32.const 0
        i64.load offset=1051016
        i64.store offset=4 align=4
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.load offset=4
        local.get 2
        i32.const 1
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 3
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1
    local.get 2
    call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
    unreachable
  )
  (func $_RNvNtCset5xJoy1xWQ_3std9panicking15panic_with_hook (;34;) (type 11) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 1
    i32.store offset=32
    local.get 5
    local.get 0
    i32.store offset=28
    local.get 5
    local.get 2
    i32.store offset=36
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 1
              call $_RNvNtNtCset5xJoy1xWQ_3std9panicking11panic_count8increase
              i32.const 255
              i32.and
              br_table 2 (;@3;) 1 (;@4;) 0 (;@5;) 1 (;@4;)
            end
            i32.const 0
            i32.load offset=1055872
            local.tee 6
            i32.const -1
            i32.gt_s
            br_if 2 (;@2;)
            local.get 5
            i32.const 56
            i32.add
            local.get 5
            i32.const 79
            i32.add
            i32.const 1051316
            i32.const 115
            call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
            local.get 5
            i32.load8_u offset=56
            local.get 5
            i32.load offset=60
            call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
            br 3 (;@1;)
          end
          local.get 5
          local.get 0
          local.get 1
          i32.load offset=24
          call_indirect (type 2)
          local.get 5
          local.get 5
          i32.load offset=4
          i32.const 0
          local.get 5
          i32.load
          local.tee 1
          select
          i32.store offset=44
          local.get 5
          local.get 1
          i32.const 1
          local.get 1
          select
          i32.store offset=40
          local.get 5
          i32.const 5
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 40
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=64
          local.get 5
          i32.const 6
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 5
          i32.const 36
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=56
          local.get 5
          i32.const 48
          i32.add
          local.get 5
          i32.const 79
          i32.add
          i32.const 1050405
          local.get 5
          i32.const 56
          i32.add
          call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
          local.get 5
          i32.load8_u offset=48
          local.get 5
          i32.load offset=52
          call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
          br 2 (;@1;)
        end
        local.get 5
        i32.const 7
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 5
        i32.const 28
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=64
        local.get 5
        i32.const 6
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.get 5
        i32.const 36
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=56
        local.get 5
        i32.const 48
        i32.add
        local.get 5
        i32.const 79
        i32.add
        i32.const 1050515
        local.get 5
        i32.const 56
        i32.add
        call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
        local.get 5
        i32.load8_u offset=48
        local.get 5
        i32.load offset=52
        call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
        br 1 (;@1;)
      end
      i32.const 0
      local.get 6
      i32.const 1
      i32.add
      i32.store offset=1055872
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1055876
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.const 16
          i32.add
          local.get 0
          local.get 1
          i32.load offset=20
          call_indirect (type 2)
          local.get 5
          local.get 4
          i32.store8 offset=69
          local.get 5
          local.get 3
          i32.store8 offset=68
          local.get 5
          local.get 2
          i32.store offset=64
          local.get 5
          local.get 5
          i64.load offset=16
          i64.store offset=56 align=4
          i32.const 0
          i32.load offset=1055876
          local.get 5
          i32.const 56
          i32.add
          i32.const 0
          i32.load offset=1055880
          i32.load offset=20
          call_indirect (type 2)
          br 1 (;@2;)
        end
        local.get 5
        i32.const 8
        i32.add
        local.get 0
        local.get 1
        i32.load offset=20
        call_indirect (type 2)
        local.get 5
        local.get 4
        i32.store8 offset=69
        local.get 5
        local.get 3
        i32.store8 offset=68
        local.get 5
        local.get 2
        i32.store offset=64
        local.get 5
        local.get 5
        i64.load offset=8
        i64.store offset=56 align=4
        local.get 5
        i32.const 56
        i32.add
        call $_RNvNtCset5xJoy1xWQ_3std9panicking12default_hook
      end
      i32.const 0
      i32.const 0
      i32.load offset=1055872
      i32.const -1
      i32.add
      i32.store offset=1055872
      i32.const 0
      i32.const 0
      i32.store8 offset=1055860
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        local.get 5
        i32.const 56
        i32.add
        local.get 5
        i32.const 79
        i32.add
        i32.const 1052524
        i32.const 91
        call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
        local.get 5
        i32.load8_u offset=56
        local.get 5
        i32.load offset=60
        call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
        br 1 (;@1;)
      end
      local.get 0
      local.get 1
      call $_RNvCsfLfy6EI15iL_7___rustc10rust_panic
      unreachable
    end
    call $_RNvNtCset5xJoy1xWQ_3std7process5abort
    unreachable
  )
  (func $_RNCINvNtNtCset5xJoy1xWQ_3std6thread7current17with_current_nameNCNCNvNtB8_9panicking12default_hook00uE0B8_ (;35;) (type 2) (param i32 i32)
    (local i32 i32 i64 i64 i64 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 592
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 1
              i32.load
              local.tee 3
              i32.load offset=16
              local.tee 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.const 20
              i32.add
              i32.load
              i32.const -1
              i32.add
              local.set 3
              br 4 (;@1;)
            end
            i32.const 0
            local.set 1
            i32.const 0
            i64.load offset=1055936
            local.tee 4
            i64.eqz
            br_if 1 (;@3;)
            i32.const 1051040
            i32.const 0
            local.get 4
            local.get 3
            i64.load offset=8
            i64.eq
            select
            local.set 1
            i32.const 4
            local.set 3
            br 3 (;@1;)
          end
          i32.const 0
          local.set 1
          i32.const 0
          i64.load offset=1055936
          local.tee 4
          i64.const 0
          i64.ne
          br_if 1 (;@2;)
        end
        br 1 (;@1;)
      end
      i32.const 1051040
      i32.const 0
      i32.const 0
      i64.load offset=1055952
      local.get 4
      i64.eq
      select
      local.set 1
      i32.const 4
      local.set 3
    end
    local.get 2
    local.get 3
    i32.const 9
    local.get 1
    select
    i32.store offset=12
    local.get 2
    local.get 1
    i32.const 1051044
    local.get 1
    select
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i64.load offset=1055952
        local.tee 5
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
        i32.const 0
        i64.load offset=1055960
        local.set 4
        loop ;; label = @3
          local.get 4
          i64.const -1
          i64.eq
          br_if 2 (;@1;)
          i32.const 0
          local.get 4
          i64.const 1
          i64.add
          local.tee 5
          i32.const 0
          i64.load offset=1055960
          local.tee 6
          local.get 6
          local.get 4
          i64.eq
          local.tee 1
          select
          i64.store offset=1055960
          local.get 6
          local.set 4
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 0
        local.get 5
        i64.store offset=1055952
      end
      local.get 2
      local.get 5
      i64.store offset=16
      local.get 2
      i32.const 24
      i32.add
      i32.const 0
      i32.const 512
      memory.fill
      local.get 2
      i64.const 0
      i64.store offset=544
      local.get 2
      i32.const 512
      i32.store offset=540
      local.get 0
      i64.load32_u offset=4
      local.set 4
      local.get 2
      local.get 2
      i32.const 24
      i32.add
      i32.store offset=536
      local.get 0
      i64.load32_u
      local.set 6
      local.get 2
      local.get 4
      i32.const 5
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 5
      i64.or
      local.tee 4
      i64.store offset=584
      local.get 2
      local.get 6
      i32.const 6
      i64.extend_i32_u
      i64.const 32
      i64.shl
      i64.or
      local.tee 6
      i64.store offset=576
      local.get 2
      i32.const 8
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 2
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      local.tee 7
      i64.store offset=568
      local.get 2
      local.get 5
      local.get 2
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      local.tee 5
      i64.store offset=560
      local.get 2
      i32.const 552
      i32.add
      local.get 2
      i32.const 536
      i32.add
      i32.const 1050476
      local.get 2
      i32.const 560
      i32.add
      call $_RNvYINtNtNtCset5xJoy1xWQ_3std2io6cursor6CursorQShENtB7_5Write9write_fmtB9_
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.load8_u offset=552
            local.tee 1
            i32.const 4
            i32.ne
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=544
            local.tee 1
            i32.const 513
            i32.lt_u
            br_if 1 (;@3;)
            i32.const 0
            local.get 1
            i32.const 512
            i32.const 1051056
            call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
            unreachable
          end
          block ;; label = @4
            local.get 1
            i32.const 3
            i32.ne
            br_if 0 (;@4;)
            local.get 2
            i32.load offset=556
            local.tee 1
            i32.load
            local.set 8
            block ;; label = @5
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 3
              i32.load
              local.tee 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              local.get 9
              call_indirect (type 1)
            end
            block ;; label = @5
              local.get 3
              i32.load offset=4
              local.tee 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 8
              local.get 9
              local.get 3
              i32.load offset=8
              call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
            end
            local.get 1
            i32.const 12
            i32.const 4
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          end
          local.get 0
          i32.load offset=12
          i32.const 36
          i32.add
          i32.load
          local.set 1
          local.get 0
          i32.load offset=8
          local.set 0
          local.get 2
          local.get 4
          i64.store offset=584
          local.get 2
          local.get 6
          i64.store offset=576
          local.get 2
          local.get 7
          i64.store offset=568
          local.get 2
          local.get 5
          i64.store offset=560
          local.get 2
          i32.const 552
          i32.add
          local.get 0
          i32.const 1050476
          local.get 2
          i32.const 560
          i32.add
          local.get 1
          call_indirect (type 3)
          local.get 2
          i32.load8_u offset=552
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=556
          local.tee 1
          i32.load
          local.set 3
          block ;; label = @4
            local.get 1
            i32.const 4
            i32.add
            i32.load
            local.tee 0
            i32.load
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 8
            call_indirect (type 1)
          end
          block ;; label = @4
            local.get 0
            i32.load offset=4
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 8
            local.get 0
            i32.load offset=8
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          end
          local.get 1
          i32.const 12
          i32.const 4
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          br 1 (;@2;)
        end
        local.get 2
        i32.const 560
        i32.add
        local.get 0
        i32.load offset=8
        local.get 2
        i32.const 24
        i32.add
        local.get 1
        local.get 0
        i32.load offset=12
        i32.load offset=28
        call_indirect (type 3)
        local.get 2
        i32.load8_u offset=560
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=564
        local.tee 1
        i32.load
        local.set 3
        block ;; label = @3
          local.get 1
          i32.const 4
          i32.add
          i32.load
          local.tee 0
          i32.load
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 8
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 8
          local.get 0
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 2
      i32.const 592
      i32.add
      global.set $__stack_pointer
      return
    end
    call $_RNvNvMNtNtCset5xJoy1xWQ_3std6thread2idNtB4_8ThreadId3new9exhausted
    unreachable
  )
  (func $_RNvXs1i_NtCsdHhIpgkcIfN_4core3fmtReNtB6_7Display3fmtCset5xJoy1xWQ_3std (;36;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 1
    call $_RNvXsi_NtCsdHhIpgkcIfN_4core3fmteNtB5_7Display3fmt
  )
  (func $_RNvXs1i_NtCsdHhIpgkcIfN_4core3fmtRNtNtNtB8_5panic8location8LocationNtB6_7Display3fmtCset5xJoy1xWQ_3std (;37;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i32.load
    local.set 4
    local.get 2
    local.get 0
    i32.load
    local.tee 1
    i64.load align=4
    i64.store align=4
    local.get 2
    i32.const 9
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.tee 5
    local.get 1
    i32.const 12
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    local.get 2
    local.get 5
    local.get 1
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    local.get 2
    i32.const 5
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 2
    i64.extend_i32_u
    i64.or
    i64.store offset=8
    local.get 4
    local.get 3
    i32.const 1048687
    local.get 2
    i32.const 8
    i32.add
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_RNvYINtNtNtCset5xJoy1xWQ_3std2io6cursor6CursorQShENtB7_5Write9write_fmtB9_ (;38;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 0
    i32.const 4
    i32.store8
    local.get 4
    local.get 1
    i32.store offset=8
    local.get 4
    local.get 0
    i64.load align=4
    i64.store
    local.get 4
    i32.const 1050740
    local.get 2
    local.get 3
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
    local.set 1
    local.get 4
    i32.load8_u
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 255
          i32.and
          i32.const 4
          i32.ne
          br_if 1 (;@2;)
          i32.const 1050764
          i32.const 173
          i32.const 1050852
          call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
          unreachable
        end
        local.get 3
        i32.const 255
        i32.and
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 4
        i32.load offset=4
        local.tee 0
        i32.load
        local.set 3
        block ;; label = @3
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          local.get 1
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i64.load
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtCset5xJoy1xWQ_3std5alloc24default_alloc_error_hook (;39;) (type 2) (param i32 i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1055969
    local.set 3
    i32.const 0
    i32.const 1
    i32.store8 offset=1055969
    i32.const 9
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.store offset=12
        local.get 2
        local.get 4
        local.get 2
        i32.const 12
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=16
        local.get 2
        i32.const 4
        i32.add
        local.get 2
        i32.const 31
        i32.add
        i32.const 1050366
        local.get 2
        i32.const 16
        i32.add
        call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
        local.get 2
        i32.load8_u offset=4
        local.get 2
        i32.load offset=8
        call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
        call $_RNvNtNtCset5xJoy1xWQ_3std3sys9backtrace4lock
        local.set 1
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                call $_RNvNtCset5xJoy1xWQ_3std5panic19get_backtrace_style
                i32.const 255
                i32.and
                br_table 0 (;@6;) 1 (;@5;) 2 (;@4;) 3 (;@3;) 0 (;@6;)
              end
              local.get 2
              i32.const 16
              i32.add
              local.get 2
              i32.const 31
              i32.add
              i32.const 10
              i32.const 0
              call $_RNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB2_13BacktraceLock5print
              local.get 2
              i32.load8_u offset=16
              local.get 2
              i32.load offset=20
              call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
              br 2 (;@3;)
            end
            local.get 2
            i32.const 16
            i32.add
            local.get 2
            i32.const 31
            i32.add
            i32.const 10
            i32.const 1
            call $_RNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB2_13BacktraceLock5print
            local.get 2
            i32.load8_u offset=16
            local.get 2
            i32.load offset=20
            call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
            br 1 (;@3;)
          end
          local.get 2
          i32.const 16
          i32.add
          local.get 2
          i32.const 31
          i32.add
          i32.const 1051096
          i32.const 157
          call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
          local.get 2
          i32.load8_u offset=16
          local.get 2
          i32.load offset=20
          call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
        end
        local.get 1
        i32.const 0
        i32.store8
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store offset=12
      local.get 2
      local.get 4
      local.get 2
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=16
      local.get 2
      i32.const 4
      i32.add
      local.get 2
      i32.const 31
      i32.add
      i32.const 1050202
      local.get 2
      i32.const 16
      i32.add
      call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
      local.get 2
      i32.load8_u offset=4
      local.get 2
      i32.load offset=8
      call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtCset5xJoy1xWQ_3std7process5abort (;40;) (type 0)
    call $_RNvNtNtNtNtCset5xJoy1xWQ_3std3sys3pal4wasi7helpers14abort_internal
    unreachable
  )
  (func $_RNCNvNtCset5xJoy1xWQ_3std9panicking12default_hook0B5_ (;41;) (type 9) (param i32 i32 i32)
    (local i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    call $_RNvNtNtCset5xJoy1xWQ_3std3sys9backtrace4lock
    local.set 4
    local.get 0
    i64.load align=4
    local.set 5
    local.get 3
    local.get 2
    i32.store offset=24
    local.get 3
    local.get 1
    i32.store offset=20
    local.get 3
    local.get 5
    i64.store offset=12 align=4
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load offset=1055944
        local.tee 6
        i32.const 2
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 12
        i32.add
        i32.const 0
        call $_RNCINvNtNtCset5xJoy1xWQ_3std6thread7current17with_current_nameNCNCNvNtB8_9panicking12default_hook00uE0B8_
        br 1 (;@1;)
      end
      local.get 3
      local.get 6
      i32.const -8
      i32.add
      i32.store offset=28
      local.get 3
      i32.const 12
      i32.add
      local.get 3
      i32.const 28
      i32.add
      call $_RNCINvNtNtCset5xJoy1xWQ_3std6thread7current17with_current_nameNCNCNvNtB8_9panicking12default_hook00uE0B8_
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load offset=8
            i32.load8_u
            br_table 0 (;@4;) 1 (;@3;) 2 (;@2;) 3 (;@1;) 0 (;@4;)
          end
          local.get 3
          i32.const 12
          i32.add
          local.get 1
          local.get 2
          i32.load offset=36
          i32.const 0
          call $_RNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB2_13BacktraceLock5print
          local.get 3
          i32.load8_u offset=12
          local.get 3
          i32.load offset=16
          call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
          br 2 (;@1;)
        end
        local.get 3
        i32.const 12
        i32.add
        local.get 1
        local.get 2
        i32.load offset=36
        i32.const 1
        call $_RNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB2_13BacktraceLock5print
        local.get 3
        i32.load8_u offset=12
        local.get 3
        i32.load offset=16
        call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
        br 1 (;@1;)
      end
      i32.const 0
      i32.load8_u offset=1055832
      local.set 0
      i32.const 0
      i32.const 0
      i32.store8 offset=1055832
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 12
      i32.add
      local.get 1
      i32.const 1051096
      i32.const 157
      local.get 2
      i32.load offset=36
      call_indirect (type 3)
      local.get 3
      i32.load8_u offset=12
      local.get 3
      i32.load offset=16
      call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
    end
    local.get 4
    i32.const 0
    i32.store8
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtNtCset5xJoy1xWQ_3std3sys9backtrace4lock (;42;) (type 8) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 0
    i32.load8_u offset=1055970
    local.set 1
    i32.const 0
    i32.const 1
    i32.store8 offset=1055970
    local.get 0
    local.get 1
    i32.store8 offset=15
    block ;; label = @1
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.const 15
      i32.add
      i32.const 1052412
      i32.const 1052136
      i32.const 65
      i32.const 1052168
      call $_RINvNtCsdHhIpgkcIfN_4core9panicking13assert_failedbbECset5xJoy1xWQ_3std
      unreachable
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 1055970
  )
  (func $_RNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB2_13BacktraceLock5print (;43;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    local.get 3
    i32.store8 offset=7
    local.get 4
    i32.const 11
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 4
    i32.const 7
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=8
    local.get 0
    local.get 1
    i32.const 1048985
    local.get 4
    i32.const 8
    i32.add
    local.get 2
    call_indirect (type 3)
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtCset5xJoy1xWQ_3std9panicking12default_hook (;44;) (type 1) (param i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 3
    local.set 2
    block ;; label = @1
      local.get 0
      i32.load8_u offset=13
      br_if 0 (;@1;)
      i32.const 1
      local.set 2
      i32.const 0
      i32.load offset=1055856
      i32.const 1
      i32.gt_u
      br_if 0 (;@1;)
      call $_RNvNtCset5xJoy1xWQ_3std5panic19get_backtrace_style
      i32.const 255
      i32.and
      local.set 2
    end
    local.get 1
    local.get 2
    i32.store8 offset=11
    local.get 1
    local.get 0
    i32.load offset=8
    i32.store offset=12
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvNtCset5xJoy1xWQ_3std9panicking14payload_as_str
    local.get 1
    local.get 1
    i64.load
    i64.store offset=16 align=4
    local.get 1
    local.get 1
    i32.const 11
    i32.add
    i32.store offset=32
    local.get 1
    local.get 1
    i32.const 16
    i32.add
    i32.store offset=28
    local.get 1
    local.get 1
    i32.const 12
    i32.add
    i32.store offset=24
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1055884
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          i32.const 1
          i32.store8 offset=1055884
          i32.const 0
          i32.load offset=1055852
          local.set 0
          i32.const 0
          i32.const 0
          i32.store offset=1055852
          local.get 0
          br_if 1 (;@2;)
        end
        local.get 1
        i32.const 24
        i32.add
        local.get 1
        i32.const 47
        i32.add
        i32.const 1052432
        call $_RNCNvNtCset5xJoy1xWQ_3std9panicking12default_hook0B5_
        br 1 (;@1;)
      end
      local.get 1
      i32.const 24
      i32.add
      local.get 0
      i32.const 8
      i32.add
      call $_RNvMs5_NtNtNtCset5xJoy1xWQ_3std4sync6poison5mutexINtB5_5MutexINtNtCsi9YzqDQQz2q_5alloc3vec3VechEE4lockBb_
      local.tee 2
      i32.const 4
      i32.add
      i32.const 1052472
      call $_RNCNvNtCset5xJoy1xWQ_3std9panicking12default_hook0B5_
      local.get 2
      i32.const 0
      i32.store8
      i32.const 0
      i32.const 1
      i32.store8 offset=1055884
      i32.const 0
      i32.load offset=1055852
      local.set 2
      i32.const 0
      local.get 0
      i32.store offset=1055852
      local.get 1
      local.get 2
      i32.store offset=40
      local.get 1
      i32.const 1
      i32.store offset=36
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 2
      i32.load
      local.tee 0
      i32.const -1
      i32.add
      i32.store
      local.get 0
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 36
      i32.add
      i32.const 4
      i32.add
      call $_RNvMsn_NtCsi9YzqDQQz2q_5alloc4syncINtB5_3ArcINtNtNtNtCset5xJoy1xWQ_3std4sync6poison5mutex5MutexINtNtB7_3vec3VechEEE9drop_slowBP_
    end
    local.get 1
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc10rust_panic (;45;) (type 2) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    local.get 1
    call $_RNvCsfLfy6EI15iL_7___rustc18___rust_start_panic
    i32.store offset=4
    local.get 2
    i32.const 9
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 2
    i32.const 4
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    local.get 2
    i32.const 8
    i32.add
    local.get 2
    i32.const 31
    i32.add
    i32.const 1050298
    local.get 2
    i32.const 16
    i32.add
    call $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_
    local.get 2
    i32.load8_u offset=8
    local.get 2
    i32.load offset=12
    call $_RINvNtCsdHhIpgkcIfN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCset5xJoy1xWQ_3std2io5error5ErrorEEB19_
    call $_RNvNtCset5xJoy1xWQ_3std7process5abort
    unreachable
  )
  (func $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_fmtBa_ (;46;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 0
    i32.const 4
    i32.store8
    local.get 4
    local.get 1
    i32.store offset=8
    local.get 4
    local.get 0
    i64.load align=4
    i64.store
    local.get 4
    i32.const 1050916
    local.get 2
    local.get 3
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
    local.set 1
    local.get 4
    i32.load8_u
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 255
          i32.and
          i32.const 4
          i32.ne
          br_if 1 (;@2;)
          i32.const 1050764
          i32.const 173
          i32.const 1050852
          call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
          unreachable
        end
        local.get 3
        i32.const 255
        i32.and
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 4
        i32.load offset=4
        local.tee 0
        i32.load
        local.set 3
        block ;; label = @3
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          local.get 1
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i64.load
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc11___rdl_alloc (;47;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 8
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          local.get 0
          i32.le_u
          br_if 1 (;@2;)
        end
        local.get 2
        i32.const 0
        i32.store offset=12
        local.get 2
        i32.const 12
        i32.add
        local.get 1
        i32.const 4
        local.get 1
        i32.const 4
        i32.gt_u
        select
        local.get 0
        call $posix_memalign
        local.set 1
        i32.const 0
        local.get 2
        i32.load offset=12
        local.get 1
        select
        local.set 1
        br 1 (;@1;)
      end
      local.get 0
      call $malloc
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc12___rust_abort (;48;) (type 0)
    call $_RNvNtCset5xJoy1xWQ_3std7process5abort
    unreachable
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc13___rdl_dealloc (;49;) (type 9) (param i32 i32 i32)
    local.get 0
    call $free
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc13___rdl_realloc (;50;) (type 7) (param i32 i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 8
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.le_u
          br_if 1 (;@2;)
        end
        i32.const 0
        local.set 5
        local.get 4
        i32.const 0
        i32.store offset=12
        local.get 4
        i32.const 12
        i32.add
        local.get 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.gt_u
        select
        local.get 3
        call $posix_memalign
        br_if 1 (;@1;)
        local.get 4
        i32.load offset=12
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 3
          local.get 1
          local.get 3
          local.get 1
          i32.lt_u
          select
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 0
          local.get 3
          memory.copy
        end
        local.get 0
        call $free
        local.get 2
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      local.get 3
      call $realloc
      local.set 5
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 5
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc17rust_begin_unwind (;51;) (type 1) (param i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i64.load align=4
    local.set 2
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 2
    i64.store offset=4 align=4
    local.get 1
    i32.const 4
    i32.add
    call $_RINvNtNtCset5xJoy1xWQ_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_9panicking13panic_handler0zEB6_
    unreachable
  )
  (func $_RNvCsfLfy6EI15iL_7___rustc26___rust_alloc_error_handler (;52;) (type 2) (param i32 i32)
    local.get 1
    local.get 0
    call $_RNvNtCset5xJoy1xWQ_3std5alloc8rust_oom
    unreachable
  )
  (func $_RNvNtCset5xJoy1xWQ_3std5alloc8rust_oom (;53;) (type 2) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call $_RINvNtNtCset5xJoy1xWQ_3std3sys9backtrace26___rust_end_short_backtraceNCNvNtB6_5alloc8rust_oom0zEB6_
    unreachable
  )
  (func $_RNvXNvMNtNtCset5xJoy1xWQ_3std3sys9backtraceNtB5_13BacktraceLock5printNtB2_16DisplayBacktraceNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt (;54;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i64 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i32.load
    local.set 4
    local.get 0
    i32.load8_u
    local.set 0
    local.get 2
    i32.const 4
    i32.add
    call $_RNvNtCset5xJoy1xWQ_3std3env11current_dir
    local.get 2
    i64.load offset=8 align=4
    local.set 5
    block ;; label = @1
      local.get 2
      i32.load offset=4
      local.tee 1
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 5
      i64.const 255
      i64.and
      i64.const 3
      i64.ne
      br_if 0 (;@1;)
      local.get 5
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 6
      i32.load
      local.set 7
      block ;; label = @2
        local.get 6
        i32.const 4
        i32.add
        i32.load
        local.tee 8
        i32.load
        local.tee 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 9
        call_indirect (type 1)
      end
      block ;; label = @2
        local.get 8
        i32.load offset=4
        local.tee 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 9
        local.get 8
        i32.load offset=8
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 6
      i32.const 12
      i32.const 4
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 4
            i32.const 1052664
            i32.const 17
            local.get 3
            i32.load offset=12
            local.tee 3
            call_indirect (type 4)
            br_if 0 (;@4;)
            local.get 0
            i32.const 1
            i32.and
            br_if 1 (;@3;)
            local.get 4
            i32.const 1052681
            i32.const 88
            local.get 3
            call_indirect (type 4)
            i32.eqz
            br_if 1 (;@3;)
          end
          i32.const 1
          local.set 4
          local.get 1
          i32.const 0
          i32.gt_s
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        local.set 4
        local.get 1
        i32.const 0
        i32.le_s
        br_if 1 (;@1;)
      end
      local.get 5
      i32.wrap_i64
      local.get 1
      i32.const 1
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $_RNvXs7_NtNtCset5xJoy1xWQ_3std2io5errorNtB5_5ErrorNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt (;55;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              i32.load8_u
              br_table 3 (;@2;) 0 (;@5;) 1 (;@4;) 2 (;@3;) 3 (;@2;)
            end
            local.get 2
            local.get 0
            i32.load8_u offset=1
            i32.const 2
            i32.shl
            local.tee 0
            i32.load offset=1053064
            i32.store offset=8
            local.get 2
            local.get 0
            i32.load offset=1053232
            i32.store offset=4
            local.get 2
            i32.const 5
            i64.extend_i32_u
            i64.const 32
            i64.shl
            local.get 2
            i32.const 4
            i32.add
            i64.extend_i32_u
            i64.or
            i64.store offset=16
            local.get 1
            i32.load
            local.get 1
            i32.load offset=4
            i32.const 1048985
            local.get 2
            i32.const 16
            i32.add
            call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
            local.set 0
            br 3 (;@1;)
          end
          local.get 0
          i32.load offset=4
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=4
          local.get 1
          call $_RNvXsi_NtCsdHhIpgkcIfN_4core3fmteNtB5_7Display3fmt
          local.set 0
          br 2 (;@1;)
        end
        local.get 0
        i32.load offset=4
        local.tee 0
        i32.load
        local.get 1
        local.get 0
        i32.load offset=4
        i32.load offset=16
        call_indirect (type 5)
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.load offset=4
      local.tee 0
      i32.store
      local.get 2
      i32.const 4
      i32.add
      local.get 0
      call $_RNvNtNtNtNtCset5xJoy1xWQ_3std3sys2io5error4wasi12error_string
      local.get 2
      i32.const 12
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 2
      i64.extend_i32_u
      i64.or
      i64.store offset=24
      local.get 2
      i32.const 13
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 2
      i32.const 4
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=16
      local.get 1
      i32.load
      local.get 1
      i32.load offset=4
      i32.const 1050185
      local.get 2
      i32.const 16
      i32.add
      call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
      local.set 0
      local.get 2
      i32.load offset=4
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=8
      local.get 1
      i32.const 1
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvNtCset5xJoy1xWQ_3std3env7__var_os (;56;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 416
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 383
          i32.gt_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 20
            i32.add
            local.get 1
            local.get 2
            memory.copy
          end
          local.get 3
          i32.const 20
          i32.add
          local.get 2
          i32.add
          i32.const 0
          i32.store8
          local.get 3
          i32.const 404
          i32.add
          local.get 3
          i32.const 20
          i32.add
          local.get 2
          i32.const 1
          i32.add
          call $_RNvMs3_NtNtCsdHhIpgkcIfN_4core3ffi5c_strNtB5_4CStr19from_bytes_with_nul
          block ;; label = @4
            local.get 3
            i32.load offset=404
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            i32.const 0
            i64.load offset=1051016
            i64.store offset=12 align=4
            i32.const -2147483647
            local.set 2
            br 2 (;@2;)
          end
          block ;; label = @4
            local.get 3
            i32.load offset=408
            call $getenv
            local.tee 1
            br_if 0 (;@4;)
            i32.const -2147483648
            local.set 2
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 1
              call $strlen
              local.tee 2
              br_if 0 (;@5;)
              i32.const 1
              local.set 4
              br 1 (;@4;)
            end
            call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 2
            i32.const 1
            call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
            local.tee 4
            i32.eqz
            br_if 3 (;@1;)
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 1
            local.get 2
            memory.copy
          end
          local.get 3
          local.get 2
          i32.store offset=16
          local.get 3
          local.get 4
          i32.store offset=12
          br 1 (;@2;)
        end
        local.get 3
        i32.const 8
        i32.add
        local.get 1
        local.get 2
        call $_RINvNtNtNtCset5xJoy1xWQ_3std3sys7helpers14small_c_string24run_with_cstr_allocatingINtNtCsdHhIpgkcIfN_4core6option6OptionNtNtNtB8_3ffi6os_str8OsStringEEB8_
        local.get 3
        i32.load offset=8
        local.set 2
      end
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const -2147483647
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          i64.load offset=12 align=4
          i64.store offset=4 align=4
          local.get 0
          local.get 2
          i32.store
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 3
          i32.load8_u offset=12
          i32.const 3
          i32.ne
          br_if 0 (;@3;)
          local.get 3
          i32.load offset=16
          local.tee 2
          i32.load
          local.set 4
          block ;; label = @4
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            call_indirect (type 1)
          end
          block ;; label = @4
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 5
            local.get 1
            i32.load offset=8
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          end
          local.get 2
          i32.const 12
          i32.const 4
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const -2147483648
        i32.store
      end
      local.get 3
      i32.const 416
      i32.add
      global.set $__stack_pointer
      return
    end
    i32.const 1
    local.get 2
    call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
    unreachable
  )
  (func $_RNvMs5_NtNtNtCset5xJoy1xWQ_3std4sync6poison5mutexINtB5_5MutexINtNtCsi9YzqDQQz2q_5alloc3vec3VechEE4lockBb_ (;57;) (type 6) (param i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 2
    local.get 0
    i32.const 1
    i32.store8
    local.get 1
    local.get 2
    i32.store8 offset=15
    block ;; label = @1
      local.get 2
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      local.get 1
      i32.const 15
      i32.add
      i32.const 1052412
      i32.const 1052136
      i32.const 65
      i32.const 1052168
      call $_RINvNtCsdHhIpgkcIfN_4core9panicking13assert_failedbbECset5xJoy1xWQ_3std
      unreachable
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE14write_all_coldBa_ (;58;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 3
        local.get 1
        i32.load
        local.tee 5
        local.get 1
        i32.load offset=8
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        local.get 4
        i32.const 8
        i32.add
        local.get 1
        call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE9flush_bufBa_
        block ;; label = @3
          local.get 4
          i32.load8_u offset=8
          i32.const 4
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i64.load offset=8
          i64.store align=4
          br 2 (;@1;)
        end
        local.get 1
        i32.load
        local.set 5
      end
      block ;; label = @2
        local.get 3
        local.get 5
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=8
        local.set 5
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.get 5
          i32.add
          local.get 2
          local.get 3
          memory.copy
        end
        local.get 0
        i32.const 4
        i32.store8
        local.get 1
        local.get 5
        local.get 3
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 1
      i32.const 1
      i32.store8 offset=12
      i64.const 0
      local.set 6
      i64.const 4
      local.set 7
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              i64.const 0
              local.set 8
              br 1 (;@4;)
            end
            block ;; label = @5
              block ;; label = @6
                loop ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 1
                      local.get 2
                      local.get 3
                      call $write
                      local.tee 5
                      i32.const -1
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 4
                      i32.const 0
                      i32.store8 offset=11
                      local.get 4
                      i32.const 0
                      i32.store16 offset=9 align=1
                      local.get 4
                      i32.const 0
                      i32.store8 offset=8
                      local.get 4
                      i32.const 0
                      i32.load offset=1055972
                      local.tee 5
                      i32.store offset=12
                      local.get 5
                      i32.const 27
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 4
                      i32.const 8
                      i32.add
                      local.set 3
                      br 4 (;@5;)
                    end
                    local.get 4
                    local.get 5
                    i32.store offset=12
                    local.get 4
                    i32.const 4
                    i32.store8 offset=8
                    block ;; label = @9
                      local.get 5
                      br_if 0 (;@9;)
                      i32.const 1052656
                      local.set 3
                      br 4 (;@5;)
                    end
                    local.get 3
                    local.get 5
                    i32.lt_u
                    br_if 2 (;@6;)
                    local.get 2
                    local.get 5
                    i32.add
                    local.set 2
                    local.get 3
                    local.get 5
                    i32.sub
                    local.set 3
                  end
                  local.get 3
                  br_if 0 (;@7;)
                end
                i64.const 0
                local.set 8
                br 2 (;@4;)
              end
              local.get 5
              local.get 3
              local.get 3
              i32.const 1053048
              call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
              unreachable
            end
            local.get 3
            i64.load align=4
            local.tee 7
            i64.const 32
            i64.shr_u
            local.set 6
            block ;; label = @5
              local.get 7
              i64.const 255
              i64.and
              i64.const 0
              i64.eq
              br_if 0 (;@5;)
              local.get 7
              local.set 8
              br 1 (;@4;)
            end
            local.get 7
            local.set 8
            local.get 6
            i64.const 8
            i64.eq
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 8
          i64.const 4294967040
          i64.and
          local.get 6
          i64.const 32
          i64.shl
          i64.or
          local.get 7
          i64.const 255
          i64.and
          i64.or
          i64.store align=4
          br 1 (;@2;)
        end
        local.get 0
        i32.const 4
        i32.store8
      end
      local.get 1
      i32.const 0
      i32.store8 offset=12
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvMsn_NtCsi9YzqDQQz2q_5alloc4syncINtB5_3ArcINtNtNtNtCset5xJoy1xWQ_3std4sync6poison5mutex5MutexINtNtB7_3vec3VechEEE9drop_slowBP_ (;59;) (type 1) (param i32)
    (local i32)
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 0
      i32.const 12
      i32.add
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 16
      i32.add
      i32.load
      local.get 1
      i32.const 1
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
    block ;; label = @1
      local.get 0
      i32.const -1
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 0
      i32.load offset=4
      local.tee 1
      i32.const -1
      i32.add
      i32.store offset=4
      local.get 1
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 0
      i32.const 24
      i32.const 4
      call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
    end
  )
  (func $_RNvNtCset5xJoy1xWQ_3std2rt19lang_start_internal (;60;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i64 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i64.load offset=1055952
        local.tee 6
        i64.const 0
        i64.ne
        br_if 0 (;@2;)
        i32.const 0
        i64.load offset=1055960
        local.set 7
        loop ;; label = @3
          local.get 7
          i64.const -1
          i64.eq
          br_if 2 (;@1;)
          i32.const 0
          local.get 7
          i64.const 1
          i64.add
          local.tee 6
          i32.const 0
          i64.load offset=1055960
          local.tee 8
          local.get 8
          local.get 7
          i64.eq
          local.tee 9
          select
          i64.store offset=1055960
          local.get 8
          local.set 7
          local.get 9
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 0
        local.get 6
        i64.store offset=1055952
      end
      i32.const 0
      local.get 6
      i64.store offset=1055936
      local.get 0
      local.get 1
      i32.load offset=20
      call_indirect (type 6)
      local.set 9
      block ;; label = @2
        i32.const 0
        i32.load8_u offset=1055968
        i32.const 3
        i32.eq
        br_if 0 (;@2;)
        local.get 5
        i32.const 1
        i32.store8 offset=15
        local.get 5
        i32.const 15
        i32.add
        call $_RINvMs0_NtNtNtNtCset5xJoy1xWQ_3std3sys4sync4once10no_threadsNtB6_4Once4callNCINvMs0_NtNtBe_4sync4onceNtB1k_4Once9call_onceNCNvNtBe_2rt7cleanup0E0EBe_
      end
      local.get 5
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 9
      return
    end
    call $_RNvNvMNtNtCset5xJoy1xWQ_3std6thread2idNtB4_8ThreadId3new9exhausted
    unreachable
  )
  (func $_RNvNtCset5xJoy1xWQ_3std3env11current_dir (;61;) (type 1) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    i32.const 512
    local.set 2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 512
            i32.const 1
            call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.store offset=8
            local.get 1
            i32.const 512
            i32.store offset=4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const 512
                  call $getcwd
                  br_if 0 (;@7;)
                  i32.const 512
                  local.set 2
                  loop ;; label = @8
                    i32.const 0
                    i32.load offset=1055972
                    local.tee 4
                    i32.const 68
                    i32.ne
                    br_if 2 (;@6;)
                    local.get 1
                    local.get 2
                    i32.store offset=12
                    local.get 1
                    i32.const 4
                    i32.add
                    local.get 2
                    i32.const 1
                    i32.const 1
                    i32.const 1
                    call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
                    local.get 1
                    i32.load offset=8
                    local.tee 3
                    local.get 1
                    i32.load offset=4
                    local.tee 2
                    call $getcwd
                    i32.eqz
                    br_if 0 (;@8;)
                  end
                end
                local.get 1
                local.get 3
                call $strlen
                local.tee 4
                i32.store offset=12
                local.get 2
                local.get 4
                i32.le_u
                br_if 4 (;@2;)
                local.get 4
                br_if 1 (;@5;)
                i32.const 1
                local.set 5
                local.get 3
                local.get 2
                i32.const 1
                call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
                br 3 (;@3;)
              end
              local.get 0
              local.get 4
              i32.store offset=8
              local.get 0
              i64.const 2147483648
              i64.store align=4
              local.get 2
              i32.eqz
              br_if 4 (;@1;)
              local.get 3
              local.get 2
              i32.const 1
              call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
              br 4 (;@1;)
            end
            local.get 3
            local.get 2
            i32.const 1
            local.get 4
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_realloc
            local.tee 5
            br_if 1 (;@3;)
            i32.const 1
            local.get 4
            call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
            unreachable
          end
          i32.const 1
          i32.const 512
          call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
          unreachable
        end
        local.get 1
        local.get 4
        i32.store offset=4
        local.get 1
        local.get 5
        i32.store offset=8
      end
      local.get 0
      local.get 1
      i32.load offset=12
      i32.store offset=8
      local.get 0
      local.get 1
      i64.load offset=4 align=4
      i64.store align=4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtCset5xJoy1xWQ_3std5panic19get_backtrace_style (;62;) (type 8) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    i32.const 3
    local.set 1
    block ;; label = @1
      i32.const 0
      i32.load8_u offset=1055868
      i32.const -1
      i32.add
      local.tee 2
      i32.const 255
      i32.and
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 4
      i32.add
      i32.const 1051373
      i32.const 14
      call $_RNvNtCset5xJoy1xWQ_3std3env7__var_os
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load offset=4
          local.tee 3
          i32.const -2147483648
          i32.ne
          br_if 0 (;@3;)
          i32.const 2
          local.set 2
          br 1 (;@2;)
        end
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                i32.load offset=12
                i32.const -1
                i32.add
                br_table 0 (;@6;) 2 (;@4;) 2 (;@4;) 1 (;@5;) 2 (;@4;)
              end
              local.get 0
              i32.load offset=8
              local.tee 4
              i32.load8_u
              i32.const 48
              i32.ne
              br_if 1 (;@4;)
              i32.const 3
              local.set 1
              i32.const 2
              local.set 2
              local.get 3
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 0
            i32.load offset=8
            local.tee 4
            i32.load align=1
            i32.const 1819047270
            i32.ne
            br_if 0 (;@4;)
            i32.const 2
            local.set 1
            i32.const 1
            local.set 2
            local.get 3
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          i32.const 1
          local.set 1
          i32.const 0
          local.set 2
          local.get 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 0
          i32.load offset=8
          local.set 4
        end
        local.get 4
        local.get 3
        i32.const 1
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      i32.const 0
      i32.const 0
      i32.load8_u offset=1055868
      local.tee 3
      local.get 1
      local.get 3
      select
      i32.store8 offset=1055868
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      i32.const 3
      local.set 2
      local.get 3
      i32.const 4
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 33619971
      local.get 3
      i32.const 3
      i32.shl
      i32.const 248
      i32.and
      i32.shr_u
      local.set 2
    end
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $_RNvNtNtNtNtCset5xJoy1xWQ_3std3sys3pal4wasi7helpers14abort_internal (;63;) (type 0)
    call $abort
    unreachable
  )
  (func $_RNvNtCset5xJoy1xWQ_3std9panicking14payload_as_str (;64;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    local.get 2
    i32.load offset=12
    local.tee 4
    call_indirect (type 2)
    i32.const 4
    local.set 2
    local.get 1
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i64.load
        i64.const 7199936582794304877
        i64.xor
        local.get 3
        i64.load offset=8
        i64.const -5076933981314334344
        i64.xor
        i64.or
        i64.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        local.get 4
        call_indirect (type 2)
        block ;; label = @3
          local.get 3
          i64.load
          i64.const -7788913181612638748
          i64.xor
          local.get 3
          i64.load offset=8
          i64.const -9212764535765366089
          i64.xor
          i64.or
          i64.const 0
          i64.eq
          br_if 0 (;@3;)
          i32.const 1052512
          local.set 1
          i32.const 12
          local.set 2
          br 2 (;@1;)
        end
        local.get 1
        i32.const 4
        i32.add
        local.set 5
        i32.const 8
        local.set 2
      end
      local.get 1
      local.get 2
      i32.add
      i32.load
      local.set 2
      local.get 5
      i32.load
      local.set 1
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtNtCset5xJoy1xWQ_3std9panicking11panic_count8increase (;65;) (type 6) (param i32) (result i32)
    (local i32 i32)
    i32.const 0
    local.set 1
    i32.const 0
    i32.const 0
    i32.load offset=1055948
    local.tee 2
    i32.const 1
    i32.add
    i32.store offset=1055948
    block ;; label = @1
      local.get 2
      i32.const 0
      i32.lt_s
      br_if 0 (;@1;)
      i32.const 1
      local.set 1
      i32.const 0
      i32.load8_u offset=1055860
      br_if 0 (;@1;)
      i32.const 0
      local.get 0
      i32.store8 offset=1055860
      i32.const 0
      i32.const 0
      i32.load offset=1055856
      i32.const 1
      i32.add
      i32.store offset=1055856
      i32.const 2
      local.set 1
    end
    local.get 1
  )
  (func $_RNvXs1j_NtCsdHhIpgkcIfN_4core3fmtQDNtNtB8_5panic12PanicPayloadEL_NtB6_7Display3fmtCset5xJoy1xWQ_3std (;66;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 5)
  )
  (func $_RNvNtNtCset5xJoy1xWQ_3std2io5stdio31print_to_buffer_if_capture_used (;67;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    local.set 3
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.load8_u offset=1055884
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        i32.const 0
        i32.load offset=1055852
        local.set 4
        i32.const 0
        i32.const 0
        i32.store offset=1055852
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load8_u offset=8
        local.set 3
        local.get 4
        i32.const 1
        i32.store8 offset=8
        local.get 2
        local.get 3
        i32.store8 offset=15
        local.get 3
        i32.const 1
        i32.eq
        br_if 1 (;@1;)
        local.get 2
        local.get 4
        i32.const 12
        i32.add
        local.get 0
        local.get 1
        call $_RNvYINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtNtCset5xJoy1xWQ_3std2io5Write9write_fmtBF_
        block ;; label = @3
          local.get 2
          i32.load8_u
          i32.const 3
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          local.tee 3
          i32.load
          local.set 0
          block ;; label = @4
            local.get 3
            i32.const 4
            i32.add
            i32.load
            local.tee 1
            i32.load
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 5
            call_indirect (type 1)
          end
          block ;; label = @4
            local.get 1
            i32.load offset=4
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 5
            local.get 1
            i32.load offset=8
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
          end
          local.get 3
          i32.const 12
          i32.const 4
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 4
        i32.const 0
        i32.store8 offset=8
        i32.const 0
        i32.load offset=1055852
        local.set 3
        i32.const 0
        local.get 4
        i32.store offset=1055852
        local.get 2
        local.get 3
        i32.store offset=8
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 3
          i32.load
          local.tee 4
          i32.const -1
          i32.add
          i32.store
          local.get 4
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.const 8
          i32.add
          call $_RNvMsn_NtCsi9YzqDQQz2q_5alloc4syncINtB5_3ArcINtNtNtNtCset5xJoy1xWQ_3std4sync6poison5mutex5MutexINtNtB7_3vec3VechEEE9drop_slowBP_
        end
        i32.const 1
        local.set 3
      end
      local.get 2
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 3
      return
    end
    i32.const 0
    local.get 2
    i32.const 15
    i32.add
    i32.const 1052412
    i32.const 1052136
    i32.const 65
    i32.const 1052168
    call $_RINvNtCsdHhIpgkcIfN_4core9panicking13assert_failedbbECset5xJoy1xWQ_3std
    unreachable
  )
  (func $_RNvYINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtNtCset5xJoy1xWQ_3std2io5Write9write_fmtBF_ (;68;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 0
    i32.const 4
    i32.store8
    local.get 4
    local.get 1
    i32.store offset=8
    local.get 4
    local.get 0
    i64.load align=4
    i64.store
    local.get 4
    i32.const 1050868
    local.get 2
    local.get 3
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
    local.set 1
    local.get 4
    i32.load8_u
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          i32.const 255
          i32.and
          i32.const 4
          i32.ne
          br_if 1 (;@2;)
          i32.const 1050764
          i32.const 173
          i32.const 1050852
          call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
          unreachable
        end
        local.get 3
        i32.const 255
        i32.and
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 4
        i32.load offset=4
        local.tee 0
        i32.load
        local.set 3
        block ;; label = @3
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 1
          i32.load
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 1
          i32.load offset=4
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          local.get 1
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 0
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i64.load
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtNtCset5xJoy1xWQ_3std2io5stdio6__print (;69;) (type 2) (param i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 6
    i32.store offset=4
    local.get 2
    i32.const 1052588
    i32.store
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.get 1
        call $_RNvNtNtCset5xJoy1xWQ_3std2io5stdio31print_to_buffer_if_capture_used
        br_if 0 (;@2;)
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1055928
          i32.const 3
          i32.eq
          br_if 0 (;@3;)
          call $_RINvMNtNtCset5xJoy1xWQ_3std4sync9once_lockINtB3_8OnceLockINtNtB5_14reentrant_lock13ReentrantLockINtNtCsdHhIpgkcIfN_4core4cell7RefCellINtNtNtNtB7_2io8buffered10linewriter10LineWriterNtNtB2e_5stdio9StdoutRawEEEE10initializeNCINvB2_11get_or_initNCNvB2V_6stdout0E0zEB7_
        end
        local.get 2
        i32.const 1055888
        i32.store offset=20
        local.get 2
        local.get 2
        i32.const 20
        i32.add
        i32.store offset=32
        local.get 2
        i32.const 8
        i32.add
        local.get 2
        i32.const 32
        i32.add
        local.get 0
        local.get 1
        call $_RNvXse_NtNtCset5xJoy1xWQ_3std2io5stdioRNtB5_6StdoutNtB7_5Write9write_fmt
        local.get 2
        i32.load8_u offset=8
        i32.const 4
        i32.ne
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 48
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 2
    local.get 2
    i64.load offset=8
    i64.store offset=24
    local.get 2
    i32.const 14
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 2
    i32.const 24
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=40
    local.get 2
    i32.const 5
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 2
    i64.extend_i32_u
    i64.or
    i64.store offset=32
    i32.const 1048961
    local.get 2
    i32.const 32
    i32.add
    i32.const 1050940
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvXse_NtNtCset5xJoy1xWQ_3std2io5stdioRNtB5_6StdoutNtB7_5Write9write_fmt (;70;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 1
    i32.load
    i32.load
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i64.load offset=1055952
            local.tee 6
            i64.const 0
            i64.ne
            br_if 0 (;@4;)
            i32.const 0
            i64.load offset=1055960
            local.set 7
            loop ;; label = @5
              local.get 7
              i64.const -1
              i64.eq
              br_if 2 (;@3;)
              i32.const 0
              local.get 7
              i64.const 1
              i64.add
              local.tee 6
              i32.const 0
              i64.load offset=1055960
              local.tee 8
              local.get 8
              local.get 7
              i64.eq
              local.tee 1
              select
              i64.store offset=1055960
              local.get 8
              local.set 7
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
            end
            i32.const 0
            local.get 6
            i64.store offset=1055952
          end
          block ;; label = @4
            block ;; label = @5
              local.get 6
              local.get 5
              i64.load
              i64.eq
              br_if 0 (;@5;)
              local.get 5
              i32.load8_u offset=12
              local.set 1
              local.get 5
              i32.const 1
              i32.store8 offset=12
              local.get 4
              local.get 1
              i32.store8 offset=16
              local.get 1
              br_if 3 (;@2;)
              local.get 5
              i32.const 1
              i32.store offset=8
              local.get 5
              local.get 6
              i64.store
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=8
            local.tee 1
            i32.const -1
            i32.eq
            br_if 3 (;@1;)
            local.get 5
            local.get 1
            i32.const 1
            i32.add
            i32.store offset=8
          end
          local.get 4
          local.get 5
          i32.store offset=12
          local.get 0
          i32.const 4
          i32.store8
          local.get 4
          local.get 0
          i64.load align=4
          i64.store offset=16
          local.get 4
          local.get 4
          i32.const 12
          i32.add
          i32.store offset=24
          local.get 4
          i32.const 16
          i32.add
          i32.const 1050892
          local.get 2
          local.get 3
          call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
          local.set 1
          local.get 4
          i32.load8_u offset=16
          local.set 5
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 255
                i32.and
                i32.const 4
                i32.ne
                br_if 1 (;@5;)
                i32.const 1050764
                i32.const 173
                i32.const 1050852
                call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
                unreachable
              end
              local.get 5
              i32.const 255
              i32.and
              i32.const 3
              i32.ne
              br_if 1 (;@4;)
              local.get 4
              i32.load offset=20
              local.tee 1
              i32.load
              local.set 0
              block ;; label = @6
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.tee 5
                i32.load
                local.tee 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                local.get 3
                call_indirect (type 1)
              end
              block ;; label = @6
                local.get 5
                i32.load offset=4
                local.tee 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 0
                local.get 3
                local.get 5
                i32.load offset=8
                call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
              end
              local.get 1
              i32.const 12
              i32.const 4
              call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
              br 1 (;@4;)
            end
            local.get 0
            local.get 4
            i64.load offset=16
            i64.store align=4
          end
          local.get 4
          i32.load offset=12
          local.tee 1
          local.get 1
          i32.load offset=8
          i32.const -1
          i32.add
          local.tee 5
          i32.store offset=8
          block ;; label = @4
            local.get 5
            br_if 0 (;@4;)
            local.get 1
            i32.const 0
            i32.store8 offset=12
            local.get 1
            i64.const 0
            i64.store
          end
          local.get 4
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
        call $_RNvNvMNtNtCset5xJoy1xWQ_3std6thread2idNtB4_8ThreadId3new9exhausted
        unreachable
      end
      i32.const 0
      local.get 4
      i32.const 16
      i32.add
      i32.const 1052412
      i32.const 1052136
      i32.const 65
      i32.const 1052168
      call $_RINvNtCsdHhIpgkcIfN_4core9panicking13assert_failedbbECset5xJoy1xWQ_3std
      unreachable
    end
    i32.const 1052184
    i32.const 38
    i32.const 1052224
    call $_RNvNtCsdHhIpgkcIfN_4core6option13expect_failed
    unreachable
  )
  (func $_RNvNtNtNtNtCset5xJoy1xWQ_3std3sys2io5error4wasi12error_string (;71;) (type 2) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 1056
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 12
    i32.add
    i32.const 0
    i32.const 1024
    memory.fill
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 2
          i32.const 12
          i32.add
          i32.const 1024
          call $strerror_r
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          local.get 2
          i32.const 1036
          i32.add
          local.get 2
          i32.const 12
          i32.add
          local.get 2
          i32.const 12
          i32.add
          call $strlen
          call $_RNvMNtCsdHhIpgkcIfN_4core3stre9from_utf8
          local.get 2
          i32.load offset=1036
          br_if 1 (;@2;)
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=1044
              local.tee 1
              br_if 0 (;@5;)
              i32.const 1
              local.set 3
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=1040
            local.set 4
            call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 1
            i32.const 1
            call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
            local.tee 3
            i32.eqz
            br_if 3 (;@1;)
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 4
            local.get 1
            memory.copy
          end
          local.get 0
          local.get 1
          i32.store offset=8
          local.get 0
          local.get 3
          i32.store offset=4
          local.get 0
          local.get 1
          i32.store
          local.get 2
          i32.const 1056
          i32.add
          global.set $__stack_pointer
          return
        end
        i32.const 1052804
        i32.const 37
        i32.const 1052824
        call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
        unreachable
      end
      local.get 2
      local.get 2
      i64.load offset=1040 align=4
      i64.store offset=1048
      i32.const 1051232
      i32.const 43
      local.get 2
      i32.const 1048
      i32.add
      i32.const 1052772
      i32.const 1052788
      call $_RNvNtCsdHhIpgkcIfN_4core6result13unwrap_failed
      unreachable
    end
    i32.const 1
    local.get 1
    call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
    unreachable
  )
  (func $_RNvXNtCsdHhIpgkcIfN_4core3anyNtNtCsi9YzqDQQz2q_5alloc6string6StringNtB2_3Any7type_idCset5xJoy1xWQ_3std (;72;) (type 2) (param i32 i32)
    local.get 0
    i32.const 0
    i64.load offset=1051308 align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 0
    i64.load offset=1051300 align=4
    i64.store align=4
  )
  (func $_RNvXNtCsdHhIpgkcIfN_4core3anyReNtB2_3Any7type_idCset5xJoy1xWQ_3std (;73;) (type 2) (param i32 i32)
    local.get 0
    i32.const 0
    i64.load offset=1051292 align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 0
    i64.load offset=1051284 align=4
    i64.store align=4
  )
  (func $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterINtNtB4_6cursor6CursorQShEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ (;74;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64 i32 i32 i64)
    i32.const 0
    local.set 3
    block ;; label = @1
      i32.const 0
      local.get 0
      i32.load offset=8
      local.tee 4
      i32.load offset=4
      local.tee 5
      local.get 4
      i64.load offset=8
      local.tee 6
      i64.const 4294967295
      local.get 6
      i64.const 4294967295
      i64.lt_u
      select
      i32.wrap_i64
      i32.sub
      local.tee 7
      local.get 7
      local.get 5
      i32.gt_u
      select
      local.tee 7
      local.get 2
      local.get 7
      local.get 2
      i32.lt_u
      select
      local.tee 8
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.load
      local.get 6
      local.get 5
      i64.extend_i32_u
      local.tee 9
      local.get 6
      local.get 9
      i64.lt_u
      select
      i32.wrap_i64
      i32.add
      local.get 1
      local.get 8
      memory.copy
    end
    local.get 4
    local.get 6
    local.get 8
    i64.extend_i32_u
    i64.add
    i64.store offset=8
    block ;; label = @1
      local.get 7
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 3
      i32.const 0
      i64.load offset=1052656
      local.tee 6
      i64.const 255
      i64.and
      i64.const 4
      i64.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.tee 2
        i32.load
        local.set 7
        block ;; label = @3
          local.get 2
          i32.const 4
          i32.add
          i32.load
          local.tee 4
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          local.get 3
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 4
          i32.load offset=4
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          local.get 3
          local.get 4
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 2
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 0
      local.get 6
      i64.store align=4
      i32.const 1
      local.set 3
    end
    local.get 3
  )
  (func $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterINtNtCsi9YzqDQQz2q_5alloc3vec3VechEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ (;75;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          local.get 0
          i32.load offset=8
          local.tee 0
          i32.load
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 2
          i32.const 1
          i32.const 1
          call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
          local.get 0
          i32.load offset=8
          local.set 3
          br 1 (;@2;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 1
      local.get 2
      memory.copy
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterNtNtB4_5stdio10StdoutLockENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ (;76;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 1
    local.get 2
    call $_RNvXsh_NtNtCset5xJoy1xWQ_3std2io5stdioNtB5_10StdoutLockNtB7_5Write9write_all
    block ;; label = @1
      local.get 3
      i32.load8_u offset=8
      local.tee 2
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.load
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 4
          i32.add
          i32.load
          local.tee 5
          i32.load
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 6
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 5
          i32.load offset=4
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 6
          local.get 5
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 0
      local.get 3
      i64.load offset=8
      i64.store align=4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
    i32.const 4
    i32.ne
  )
  (func $_RNvXsh_NtNtCset5xJoy1xWQ_3std2io5stdioNtB5_10StdoutLockNtB7_5Write9write_all (;77;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 1
                  i32.load
                  local.tee 1
                  i32.load offset=16
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const -1
                  i32.store offset=16
                  local.get 4
                  i32.const 10
                  local.get 2
                  local.get 3
                  call $_RNvNtNtCsdHhIpgkcIfN_4core5slice6memchr7memrchr
                  local.get 1
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 4
                  i32.load
                  i32.const 1
                  i32.ne
                  br_if 3 (;@4;)
                  local.get 3
                  local.get 4
                  i32.load offset=4
                  i32.const 1
                  i32.add
                  local.tee 6
                  i32.lt_u
                  br_if 1 (;@6;)
                  block ;; label = @8
                    local.get 1
                    i32.load offset=28
                    local.tee 7
                    br_if 0 (;@8;)
                    local.get 6
                    i32.eqz
                    br_if 6 (;@2;)
                    local.get 2
                    local.set 8
                    local.get 6
                    local.set 9
                    loop ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          i32.const 1
                          local.get 8
                          local.get 9
                          call $write
                          local.tee 7
                          i32.const -1
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 4
                          i32.const 0
                          i32.store8 offset=11
                          local.get 4
                          i32.const 0
                          i32.store16 offset=9 align=1
                          local.get 4
                          i32.const 0
                          i32.store8 offset=8
                          local.get 4
                          i32.const 0
                          i32.load offset=1055972
                          local.tee 7
                          i32.store offset=12
                          local.get 7
                          i32.const 27
                          i32.eq
                          br_if 1 (;@10;)
                          local.get 4
                          i32.const 8
                          i32.add
                          local.set 7
                          br 8 (;@3;)
                        end
                        local.get 4
                        local.get 7
                        i32.store offset=12
                        local.get 4
                        i32.const 4
                        i32.store8 offset=8
                        block ;; label = @11
                          local.get 7
                          br_if 0 (;@11;)
                          i32.const 1052656
                          local.set 7
                          br 8 (;@3;)
                        end
                        local.get 9
                        local.get 7
                        i32.lt_u
                        br_if 5 (;@5;)
                        local.get 8
                        local.get 7
                        i32.add
                        local.set 8
                        local.get 9
                        local.get 7
                        i32.sub
                        local.set 9
                      end
                      local.get 9
                      br_if 0 (;@9;)
                      br 7 (;@2;)
                    end
                  end
                  block ;; label = @8
                    block ;; label = @9
                      local.get 6
                      local.get 5
                      i32.load
                      local.get 7
                      i32.sub
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 4
                      i32.const 8
                      i32.add
                      local.get 5
                      local.get 2
                      local.get 6
                      call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE14write_all_coldBa_
                      local.get 4
                      i32.load8_u offset=8
                      i32.const 4
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 0
                      local.get 4
                      i64.load offset=8
                      i64.store align=4
                      br 8 (;@1;)
                    end
                    block ;; label = @9
                      local.get 6
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      i32.load offset=24
                      local.get 7
                      i32.add
                      local.get 2
                      local.get 6
                      memory.copy
                    end
                    local.get 1
                    local.get 7
                    local.get 6
                    i32.add
                    i32.store offset=28
                  end
                  local.get 4
                  i32.const 8
                  i32.add
                  local.get 5
                  call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE9flush_bufBa_
                  local.get 4
                  i32.load8_u offset=8
                  i32.const 4
                  i32.eq
                  br_if 5 (;@2;)
                  local.get 0
                  local.get 4
                  i64.load offset=8
                  i64.store align=4
                  br 6 (;@1;)
                end
                i32.const 1053032
                call $_RNvNtCsdHhIpgkcIfN_4core4cell22panic_already_borrowed
                unreachable
              end
              i32.const 1051275
              i32.const 19
              i32.const 1053000
              call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
              unreachable
            end
            local.get 7
            local.get 9
            local.get 9
            i32.const 1053048
            call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
            unreachable
          end
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.load offset=28
                local.tee 7
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                br 1 (;@5;)
              end
              local.get 1
              i32.load offset=24
              local.get 7
              i32.add
              i32.const -1
              i32.add
              i32.load8_u
              i32.const 10
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.const 8
              i32.add
              local.get 5
              call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE9flush_bufBa_
              local.get 4
              i32.load8_u offset=8
              i32.const 4
              i32.ne
              br_if 1 (;@4;)
              local.get 1
              i32.load offset=28
              local.set 7
            end
            block ;; label = @5
              local.get 3
              local.get 5
              i32.load
              local.get 7
              i32.sub
              i32.lt_u
              br_if 0 (;@5;)
              local.get 0
              local.get 5
              local.get 2
              local.get 3
              call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE14write_all_coldBa_
              br 4 (;@1;)
            end
            block ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              i32.load offset=24
              local.get 7
              i32.add
              local.get 2
              local.get 3
              memory.copy
            end
            local.get 0
            i32.const 4
            i32.store8
            local.get 1
            local.get 7
            local.get 3
            i32.add
            i32.store offset=28
            br 3 (;@1;)
          end
          local.get 0
          local.get 4
          i64.load offset=8
          i64.store align=4
          br 2 (;@1;)
        end
        local.get 7
        i64.load
        local.tee 10
        i64.const -4294967041
        i64.and
        i64.const 34359738368
        i64.eq
        br_if 0 (;@2;)
        local.get 10
        i64.const 255
        i64.and
        i64.const 4
        i64.eq
        br_if 0 (;@2;)
        local.get 0
        local.get 10
        i64.store align=4
        br 1 (;@1;)
      end
      local.get 2
      local.get 6
      i32.add
      local.set 8
      block ;; label = @2
        local.get 3
        local.get 6
        i32.sub
        local.tee 7
        local.get 1
        i32.load offset=20
        local.get 1
        i32.load offset=28
        local.tee 9
        i32.sub
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        local.get 8
        local.get 7
        call $_RNvMs_NtNtNtCset5xJoy1xWQ_3std2io8buffered9bufwriterINtB4_9BufWriterNtNtB8_5stdio9StdoutRawE14write_all_coldBa_
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 7
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=24
        local.get 9
        i32.add
        local.get 8
        local.get 7
        memory.copy
      end
      local.get 0
      i32.const 4
      i32.store8
      local.get 1
      local.get 9
      local.get 7
      i32.add
      i32.store offset=28
    end
    local.get 1
    local.get 1
    i32.load offset=16
    i32.const 1
    i32.add
    i32.store offset=16
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterNtNtNtNtB6_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_ (;78;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              block ;; label = @6
                i32.const 2
                local.get 1
                local.get 2
                call $write
                local.tee 4
                i32.const -1
                i32.ne
                br_if 0 (;@6;)
                local.get 3
                i32.const 0
                i32.store8 offset=11
                local.get 3
                i32.const 0
                i32.store16 offset=9 align=1
                local.get 3
                i32.const 0
                i32.store8 offset=8
                local.get 3
                i32.const 0
                i32.load offset=1055972
                local.tee 4
                i32.store offset=12
                local.get 4
                i32.const 27
                i32.eq
                br_if 1 (;@5;)
                local.get 3
                i32.const 8
                i32.add
                local.set 4
                br 4 (;@2;)
              end
              local.get 3
              local.get 4
              i32.store offset=12
              local.get 3
              i32.const 4
              i32.store8 offset=8
              block ;; label = @6
                local.get 4
                br_if 0 (;@6;)
                i32.const 1052656
                local.set 4
                br 4 (;@2;)
              end
              local.get 2
              local.get 4
              i32.lt_u
              br_if 2 (;@3;)
              local.get 1
              local.get 4
              i32.add
              local.set 1
              local.get 2
              local.get 4
              i32.sub
              local.set 2
            end
            local.get 2
            br_if 0 (;@4;)
          end
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 4
        local.get 2
        local.get 2
        i32.const 1053048
        call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
        unreachable
      end
      block ;; label = @2
        local.get 4
        i64.load
        local.tee 5
        i64.const 255
        i64.and
        i64.const 4
        i64.ne
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.tee 4
        i32.load
        local.set 1
        block ;; label = @3
          local.get 4
          i32.const 4
          i32.add
          i32.load
          local.tee 2
          i32.load
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 6
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 2
          i32.load offset=4
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 6
          local.get 2
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 4
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 0
      local.get 5
      i64.store align=4
      i32.const 1
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write18write_all_vectoredBa_ (;79;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 4
          i32.add
          local.set 5
          local.get 3
          i32.const 3
          i32.shl
          local.tee 6
          i32.const -8
          i32.add
          i32.const 3
          i32.shr_u
          i32.const 1
          i32.add
          local.set 7
          i32.const 0
          local.set 8
          block ;; label = @4
            loop ;; label = @5
              local.get 5
              i32.load
              br_if 1 (;@4;)
              local.get 5
              i32.const 8
              i32.add
              local.set 5
              local.get 8
              i32.const 1
              i32.add
              local.set 8
              local.get 6
              i32.const -8
              i32.add
              local.tee 6
              br_if 0 (;@5;)
            end
            local.get 7
            local.set 8
          end
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                local.get 8
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                local.get 8
                i32.eq
                br_if 3 (;@3;)
                local.get 3
                local.get 8
                i32.sub
                local.set 7
                local.get 2
                local.get 8
                i32.const 3
                i32.shl
                i32.add
                local.set 9
                loop ;; label = @7
                  block ;; label = @8
                    i32.const 2
                    local.get 9
                    local.get 7
                    i32.const 16
                    local.get 7
                    i32.const 16
                    i32.lt_u
                    select
                    call $writev
                    local.tee 5
                    i32.const -1
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 4
                    i32.const 0
                    i32.store8 offset=11
                    local.get 4
                    i32.const 0
                    i32.store16 offset=9 align=1
                    local.get 4
                    i32.const 0
                    i32.store8 offset=8
                    local.get 4
                    i32.const 0
                    i32.load offset=1055972
                    local.tee 5
                    i32.store offset=12
                    local.get 5
                    i32.const 27
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 4
                    i32.const 8
                    i32.add
                    local.set 5
                    br 6 (;@2;)
                  end
                  local.get 4
                  local.get 5
                  i32.store offset=12
                  local.get 4
                  i32.const 4
                  i32.store8 offset=8
                  block ;; label = @8
                    local.get 5
                    br_if 0 (;@8;)
                    i32.const 1052656
                    local.set 5
                    br 6 (;@2;)
                  end
                  local.get 9
                  i32.const 4
                  i32.add
                  local.set 8
                  local.get 7
                  i32.const 3
                  i32.shl
                  local.tee 3
                  i32.const -8
                  i32.add
                  i32.const 3
                  i32.shr_u
                  i32.const 1
                  i32.add
                  local.set 10
                  i32.const 0
                  local.set 6
                  block ;; label = @8
                    loop ;; label = @9
                      local.get 5
                      local.get 8
                      i32.load
                      local.tee 2
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 8
                      i32.const 8
                      i32.add
                      local.set 8
                      local.get 6
                      i32.const 1
                      i32.add
                      local.set 6
                      local.get 5
                      local.get 2
                      i32.sub
                      local.set 5
                      local.get 3
                      i32.const -8
                      i32.add
                      local.tee 3
                      br_if 0 (;@9;)
                    end
                    local.get 10
                    local.set 6
                  end
                  local.get 7
                  local.get 6
                  i32.lt_u
                  br_if 2 (;@5;)
                  block ;; label = @8
                    local.get 7
                    local.get 6
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 5
                    i32.eqz
                    br_if 5 (;@3;)
                    i32.const 1052240
                    i32.const 79
                    i32.const 1052280
                    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
                    unreachable
                  end
                  local.get 9
                  local.get 6
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 9
                  i32.load offset=4
                  local.tee 8
                  local.get 5
                  i32.lt_u
                  br_if 3 (;@4;)
                  local.get 7
                  local.get 6
                  i32.sub
                  local.set 7
                  local.get 9
                  local.get 8
                  local.get 5
                  i32.sub
                  i32.store offset=4
                  local.get 9
                  local.get 9
                  i32.load
                  local.get 5
                  i32.add
                  i32.store
                  local.get 4
                  i32.load8_u offset=8
                  local.tee 5
                  i32.const 4
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 3
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load offset=12
                  local.tee 5
                  i32.load
                  local.set 6
                  block ;; label = @8
                    local.get 5
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 8
                    i32.load
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    local.get 3
                    call_indirect (type 1)
                  end
                  block ;; label = @8
                    local.get 8
                    i32.load offset=4
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    local.get 3
                    local.get 8
                    i32.load offset=8
                    call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
                  end
                  local.get 5
                  i32.const 12
                  i32.const 4
                  call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
                  br 0 (;@7;)
                end
              end
              local.get 8
              local.get 3
              local.get 3
              i32.const 1052348
              call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
              unreachable
            end
            local.get 6
            local.get 7
            local.get 7
            i32.const 1052348
            call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
            unreachable
          end
          i32.const 1052296
          i32.const 71
          i32.const 1052332
          call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
          unreachable
        end
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 0
      local.get 5
      i64.load
      i64.store align=4
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvXs0_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_19FormatStringPayloadNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt (;80;) (type 5) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      i32.load
      i32.const -2147483648
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter9write_str
      return
    end
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    i32.load offset=12
    i32.load
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvXs1_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload3get (;81;) (type 2) (param i32 i32)
    local.get 0
    i32.const 1052912
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_RNvXs1_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload6as_str (;82;) (type 2) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store
  )
  (func $_RNvXs1_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload8take_box (;83;) (type 2) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      i32.const 8
      i32.const 4
      call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 8
      call $_RNvNtCsi9YzqDQQz2q_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1052912
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCset5xJoy1xWQ_3std (;84;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load
        local.tee 0
        i32.load8_u
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        i32.const 1
        i32.add
        i32.store offset=12
        local.get 1
        i32.const 1052993
        i32.const 4
        local.get 2
        i32.const 12
        i32.add
        i32.const 1050708
        call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish
        local.set 0
        br 1 (;@1;)
      end
      local.get 1
      i32.const 1052989
      i32.const 4
      call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter9write_str
      local.set 0
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRbNtB6_5Debug3fmtCset5xJoy1xWQ_3std (;85;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_RNvXsg_NtCsdHhIpgkcIfN_4core3fmtbNtB5_7Display3fmt
  )
  (func $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRhNtB6_5Debug3fmtCset5xJoy1xWQ_3std (;86;) (type 5) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.set 0
    block ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 2
      i32.const 33554432
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        i32.const 67108864
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_RNvXNtNtNtCsdHhIpgkcIfN_4core3fmt3num3imphNtB6_7Display3fmt
        return
      end
      local.get 0
      local.get 1
      call $_RNvXsg_NtNtCsdHhIpgkcIfN_4core3fmt3numhNtB7_8UpperHex3fmt
      return
    end
    local.get 0
    local.get 1
    call $_RNvXse_NtNtCsdHhIpgkcIfN_4core3fmt3numhNtB7_8LowerHex3fmt
  )
  (func $_RNvXs2_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB5_16StaticStrPayloadNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt (;87;) (type 5) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter9write_str
  )
  (func $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write14write_vectored (;88;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    i32.const 4
    local.set 4
    block ;; label = @1
      i32.const 2
      local.get 2
      local.get 3
      i32.const 16
      local.get 3
      i32.const 16
      i32.lt_u
      select
      call $writev
      local.tee 3
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      local.get 0
      i32.const 0
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 0
      i32.load offset=1055972
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store8
  )
  (func $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write17is_write_vectored (;89;) (type 6) (param i32) (result i32)
    i32.const 1
  )
  (func $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write5flush (;90;) (type 2) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_RNvXs3_NtNtNtCset5xJoy1xWQ_3std3sys5stdio4unixNtB5_6StderrNtNtBb_2io5Write5write (;91;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    i32.const 4
    local.set 4
    block ;; label = @1
      i32.const 2
      local.get 2
      local.get 3
      call $write
      local.tee 3
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      local.get 0
      i32.const 0
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 0
      i32.load offset=1055972
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store8
  )
  (func $_RNvXsq_NtCsi9YzqDQQz2q_5alloc6stringNtB5_6StringNtNtCsdHhIpgkcIfN_4core3fmt7Display3fmt (;92;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 1
    call $_RNvXsi_NtCsdHhIpgkcIfN_4core3fmteNtB5_7Display3fmt
  )
  (func $_RNvXs9_NtNtCsdHhIpgkcIfN_4core3str5errorNtB5_9Utf8ErrorNtNtB9_3fmt5Debug3fmt (;93;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    local.get 0
    i32.const 4
    i32.add
    i32.store offset=12
    local.get 1
    i32.const 1052960
    i32.const 9
    i32.const 1052969
    i32.const 11
    local.get 0
    i32.const 1052928
    i32.const 1052980
    i32.const 9
    local.get 2
    i32.const 12
    i32.add
    i32.const 1052944
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write14write_vectoredB9_ (;94;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.const 3
      i32.and
      local.set 5
      i32.const 0
      local.set 6
      i32.const 0
      local.set 4
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          i32.const 28
          i32.add
          local.set 7
          local.get 3
          i32.const 268435452
          i32.and
          local.set 8
          i32.const 0
          local.set 4
          i32.const 0
          local.set 6
          loop ;; label = @4
            local.get 7
            i32.load
            local.get 7
            i32.const -8
            i32.add
            i32.load
            local.get 7
            i32.const -16
            i32.add
            i32.load
            local.get 7
            i32.const -24
            i32.add
            i32.load
            local.get 4
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 4
            local.get 7
            i32.const 32
            i32.add
            local.set 7
            local.get 8
            local.get 6
            i32.const 4
            i32.add
            local.tee 6
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 5
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 6
        i32.const 3
        i32.shl
        local.get 2
        i32.add
        i32.const 4
        i32.add
        local.set 7
        loop ;; label = @3
          local.get 7
          i32.load
          local.get 4
          i32.add
          local.set 4
          local.get 7
          i32.const 8
          i32.add
          local.set 7
          local.get 5
          i32.const -1
          i32.add
          local.tee 5
          br_if 0 (;@3;)
        end
      end
      local.get 3
      i32.const 3
      i32.shl
      local.set 7
      block ;; label = @2
        local.get 4
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 5
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 5
        local.get 4
        i32.const 1
        i32.const 1
        call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
      end
      local.get 2
      local.get 7
      i32.add
      local.set 8
      local.get 1
      i32.load offset=8
      local.set 5
      loop ;; label = @2
        local.get 2
        i32.load
        local.set 6
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.const 4
              i32.add
              i32.load
              local.tee 7
              local.get 1
              i32.load
              local.get 5
              i32.sub
              i32.le_u
              br_if 0 (;@5;)
              local.get 1
              local.get 5
              local.get 7
              i32.const 1
              i32.const 1
              call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
              local.get 1
              i32.load offset=8
              local.set 5
              br 1 (;@4;)
            end
            local.get 7
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.get 5
          i32.add
          local.get 6
          local.get 7
          memory.copy
        end
        local.get 1
        local.get 5
        local.get 7
        i32.add
        local.tee 5
        i32.store offset=8
        local.get 2
        i32.const 8
        i32.add
        local.tee 2
        local.get 8
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0
    i32.const 4
    i32.store8
    local.get 0
    local.get 4
    i32.store offset=4
  )
  (func $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write17is_write_vectoredB9_ (;95;) (type 6) (param i32) (result i32)
    i32.const 1
  )
  (func $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write18write_all_vectoredB9_ (;96;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 3
      i32.and
      local.set 4
      i32.const 0
      local.set 5
      i32.const 0
      local.set 6
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          i32.const 28
          i32.add
          local.set 7
          local.get 3
          i32.const 268435452
          i32.and
          local.set 8
          i32.const 0
          local.set 5
          i32.const 0
          local.set 6
          loop ;; label = @4
            local.get 7
            i32.load
            local.get 7
            i32.const -8
            i32.add
            i32.load
            local.get 7
            i32.const -16
            i32.add
            i32.load
            local.get 7
            i32.const -24
            i32.add
            i32.load
            local.get 6
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 6
            local.get 7
            i32.const 32
            i32.add
            local.set 7
            local.get 8
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 5
        i32.const 3
        i32.shl
        local.get 2
        i32.add
        i32.const 4
        i32.add
        local.set 7
        loop ;; label = @3
          local.get 7
          i32.load
          local.get 6
          i32.add
          local.set 6
          local.get 7
          i32.const 8
          i32.add
          local.set 7
          local.get 4
          i32.const -1
          i32.add
          local.tee 4
          br_if 0 (;@3;)
        end
      end
      local.get 3
      i32.const 3
      i32.shl
      local.set 7
      block ;; label = @2
        local.get 6
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        local.get 4
        local.get 6
        i32.const 1
        i32.const 1
        call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
        local.get 1
        i32.load offset=8
        local.set 4
      end
      local.get 2
      local.get 7
      i32.add
      local.set 5
      loop ;; label = @2
        local.get 2
        i32.load
        local.set 6
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.const 4
              i32.add
              i32.load
              local.tee 7
              local.get 1
              i32.load
              local.get 4
              i32.sub
              i32.le_u
              br_if 0 (;@5;)
              local.get 1
              local.get 4
              local.get 7
              i32.const 1
              i32.const 1
              call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
              local.get 1
              i32.load offset=8
              local.set 4
              br 1 (;@4;)
            end
            local.get 7
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.get 4
          i32.add
          local.get 6
          local.get 7
          memory.copy
        end
        local.get 1
        local.get 4
        local.get 7
        i32.add
        local.tee 4
        i32.store offset=8
        local.get 2
        i32.const 8
        i32.add
        local.tee 2
        local.get 5
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write5flushB9_ (;97;) (type 2) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write5writeB9_ (;98;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          local.get 1
          i32.load
          local.get 1
          i32.load offset=8
          local.tee 4
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          local.get 1
          local.get 4
          local.get 3
          i32.const 1
          i32.const 1
          call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
          local.get 1
          i32.load offset=8
          local.set 4
          br 1 (;@2;)
        end
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=4
      local.get 4
      i32.add
      local.get 2
      local.get 3
      memory.copy
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    i32.const 4
    i32.store8
    local.get 1
    local.get 4
    local.get 3
    i32.add
    i32.store offset=8
  )
  (func $_RNvXs9_NtNtCset5xJoy1xWQ_3std2io5implsINtNtCsi9YzqDQQz2q_5alloc3vec3VechENtB7_5Write9write_allB9_ (;99;) (type 3) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          local.get 1
          i32.load
          local.get 1
          i32.load offset=8
          local.tee 4
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          local.get 1
          local.get 4
          local.get 3
          i32.const 1
          i32.const 1
          call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
          local.get 1
          i32.load offset=8
          local.set 4
          br 1 (;@2;)
        end
        local.get 3
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 3
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=4
      local.get 4
      i32.add
      local.get 2
      local.get 3
      memory.copy
    end
    local.get 0
    i32.const 4
    i32.store8
    local.get 1
    local.get 4
    local.get 3
    i32.add
    i32.store offset=8
  )
  (func $_RNvXsZ_NtCsi9YzqDQQz2q_5alloc6stringNtB5_6StringNtNtCsdHhIpgkcIfN_4core3fmt5Write10write_char (;100;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=8
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 128
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 3
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 1
        i32.const 2048
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 2
        local.set 3
        br 1 (;@1;)
      end
      i32.const 3
      i32.const 4
      local.get 1
      i32.const 65536
      i32.lt_u
      select
      local.set 3
    end
    local.get 2
    local.set 4
    block ;; label = @1
      local.get 3
      local.get 0
      i32.load
      local.get 2
      i32.sub
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 3
      i32.const 1
      i32.const 1
      call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
      local.get 0
      i32.load offset=8
      local.set 4
    end
    local.get 0
    i32.load offset=4
    local.get 4
    i32.add
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 5
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 6
        block ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.store8 offset=1
          local.get 4
          local.get 6
          i32.const 192
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 7
        local.get 6
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 6
        block ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          i32.store8 offset=2
          local.get 4
          local.get 6
          i32.store8 offset=1
          local.get 4
          local.get 7
          i32.const 224
          i32.or
          i32.store8
          br 2 (;@1;)
        end
        local.get 4
        local.get 5
        i32.store8 offset=3
        local.get 4
        local.get 6
        i32.store8 offset=2
        local.get 4
        local.get 7
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=1
        local.get 4
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8
        br 1 (;@1;)
      end
      local.get 4
      local.get 1
      i32.store8
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func $_RNvXsZ_NtCsi9YzqDQQz2q_5alloc6stringNtB5_6StringNtNtCsdHhIpgkcIfN_4core3fmt5Write9write_str (;101;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          local.get 0
          i32.load
          local.get 0
          i32.load offset=8
          local.tee 3
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          local.get 0
          local.get 3
          local.get 2
          i32.const 1
          i32.const 1
          call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
          local.get 0
          i32.load offset=8
          local.set 3
          br 1 (;@2;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 1
      local.get 2
      memory.copy
    end
    local.get 0
    local.get 3
    local.get 2
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func $_RNvXsZ_NtNtCsdHhIpgkcIfN_4core3fmt3numjNtB7_5Debug3fmt (;102;) (type 5) (param i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 2
      i32.const 33554432
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        i32.const 67108864
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 1
        call $_RNvXs8_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3impmNtB9_7Display3fmt
        return
      end
      local.get 0
      local.get 1
      call $_RNvXs8_NtNtCsdHhIpgkcIfN_4core3fmt3numjNtB7_8UpperHex3fmt
      return
    end
    local.get 0
    local.get 1
    call $_RNvXs6_NtNtCsdHhIpgkcIfN_4core3fmt3numjNtB7_8LowerHex3fmt
  )
  (func $_RNvXs_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload3get (;103;) (type 2) (param i32 i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      i32.load
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 0
      i32.store offset=28
      local.get 2
      i64.const 4294967296
      i64.store offset=20 align=4
      local.get 2
      i32.const 20
      i32.add
      i32.const 1051072
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
      drop
      local.get 2
      local.get 2
      i32.load offset=28
      local.tee 3
      i32.store offset=16
      local.get 2
      local.get 2
      i64.load offset=20 align=4
      local.tee 4
      i64.store offset=8
      local.get 1
      local.get 3
      i32.store offset=8
      local.get 1
      local.get 4
      i64.store align=4
    end
    local.get 0
    i32.const 1053016
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvXs_NvNtCset5xJoy1xWQ_3std9panicking13panic_handlerNtB4_19FormatStringPayloadNtNtCsdHhIpgkcIfN_4core5panic12PanicPayload8take_box (;104;) (type 2) (param i32 i32)
    (local i32 i32 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      local.get 1
      i32.load
      i32.const -2147483648
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 3
      local.get 2
      i32.const 0
      i32.store offset=44
      local.get 2
      i64.const 4294967296
      i64.store offset=36 align=4
      local.get 2
      i32.const 36
      i32.add
      i32.const 1051072
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
      drop
      local.get 2
      local.get 2
      i32.load offset=44
      local.tee 3
      i32.store offset=32
      local.get 2
      local.get 2
      i64.load offset=36 align=4
      local.tee 4
      i64.store offset=24
      local.get 1
      local.get 3
      i32.store offset=8
      local.get 1
      local.get 4
      i64.store align=4
    end
    local.get 1
    i32.load offset=8
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=8
    local.get 1
    i64.load align=4
    local.set 4
    local.get 1
    i64.const 4294967296
    i64.store align=4
    local.get 2
    local.get 3
    i32.store offset=16
    local.get 2
    local.get 4
    i64.store offset=8
    call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      i32.const 12
      i32.const 4
      call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
      local.tee 1
      br_if 0 (;@1;)
      i32.const 4
      i32.const 12
      call $_RNvNtCsi9YzqDQQz2q_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 1
    local.get 2
    i32.load offset=16
    i32.store offset=8
    local.get 1
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 0
    i32.const 1053016
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtB7_6cursor6CursorQShEENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ (;105;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i32 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=13
          local.get 2
          local.get 4
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 5
        local.get 4
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=14
          local.get 2
          local.get 4
          i32.store8 offset=13
          local.get 2
          local.get 5
          i32.const 224
          i32.or
          i32.store8 offset=12
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 3
        i32.store8 offset=15
        local.get 2
        local.get 4
        i32.store8 offset=14
        local.get 2
        local.get 5
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    i32.const 0
    local.set 5
    block ;; label = @1
      i32.const 0
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.load offset=4
      local.tee 6
      local.get 3
      i64.load offset=8
      local.tee 7
      i64.const 4294967295
      local.get 7
      i64.const 4294967295
      i64.lt_u
      select
      i32.wrap_i64
      i32.sub
      local.tee 4
      local.get 4
      local.get 6
      i32.gt_u
      select
      local.tee 4
      local.get 1
      local.get 4
      local.get 1
      i32.lt_u
      select
      local.tee 8
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.load
      local.get 7
      local.get 6
      i64.extend_i32_u
      local.tee 9
      local.get 7
      local.get 9
      i64.lt_u
      select
      i32.wrap_i64
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 8
      memory.copy
    end
    local.get 3
    local.get 7
    local.get 8
    i64.extend_i32_u
    i64.add
    i64.store offset=8
    block ;; label = @1
      local.get 4
      local.get 1
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 0
      local.set 5
      i32.const 0
      i64.load offset=1052656
      local.tee 7
      i64.const 255
      i64.and
      i64.const 4
      i64.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.load
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 4
          i32.add
          i32.load
          local.tee 3
          i32.load
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 3
          i32.load offset=4
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 5
          local.get 3
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 1
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 0
      local.get 7
      i64.store align=4
      i32.const 1
      local.set 5
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 5
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtB7_6cursor6CursorQShEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ (;106;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050740
    local.get 1
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtCsi9YzqDQQz2q_5alloc3vec3VechEENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ (;107;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=13
          local.get 2
          local.get 4
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 5
        local.get 4
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=14
          local.get 2
          local.get 4
          i32.store8 offset=13
          local.get 2
          local.get 5
          i32.const 224
          i32.or
          i32.store8 offset=12
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 3
        i32.store8 offset=15
        local.get 2
        local.get 4
        i32.store8 offset=14
        local.get 2
        local.get 5
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    block ;; label = @1
      local.get 1
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load
      local.get 0
      i32.load offset=8
      local.tee 3
      i32.sub
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      local.get 1
      i32.const 1
      i32.const 1
      call $_RINvNvMs2_NtCsi9YzqDQQz2q_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECset5xJoy1xWQ_3std
      local.get 0
      i32.load offset=8
      local.set 3
    end
    block ;; label = @1
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      memory.copy
    end
    local.get 0
    local.get 3
    local.get 1
    i32.add
    i32.store offset=8
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 0
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterINtNtCsi9YzqDQQz2q_5alloc3vec3VechEENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ (;108;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050868
    local.get 1
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtB7_5stdio10StdoutLockENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ (;109;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=4
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=5
          local.get 2
          local.get 4
          i32.const 192
          i32.or
          i32.store8 offset=4
          i32.const 2
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 5
        local.get 4
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=6
          local.get 2
          local.get 4
          i32.store8 offset=5
          local.get 2
          local.get 5
          i32.const 224
          i32.or
          i32.store8 offset=4
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 3
        i32.store8 offset=7
        local.get 2
        local.get 4
        i32.store8 offset=6
        local.get 2
        local.get 5
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=5
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8 offset=4
        i32.const 4
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=4
      i32.const 1
      local.set 1
    end
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.load offset=8
    local.get 2
    i32.const 4
    i32.add
    local.get 1
    call $_RNvXsh_NtNtCset5xJoy1xWQ_3std2io5stdioNtB5_10StdoutLockNtB7_5Write9write_all
    block ;; label = @1
      local.get 2
      i32.load8_u offset=8
      local.tee 1
      i32.const 4
      i32.eq
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.load
        local.set 5
        block ;; label = @3
          local.get 3
          i32.const 4
          i32.add
          i32.load
          local.tee 4
          i32.load
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 6
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 4
          i32.load offset=4
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 6
          local.get 4
          i32.load offset=8
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
        end
        local.get 3
        i32.const 12
        i32.const 4
        call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
      end
      local.get 0
      local.get 2
      i64.load offset=8
      i64.store align=4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
    i32.const 4
    i32.ne
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtB7_5stdio10StdoutLockENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ (;110;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050892
    local.get 1
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtNtNtB9_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write10write_charB9_ (;111;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 2
    i32.const 0
    i32.store offset=12
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.const 128
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 2048
          i32.ge_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=13
          local.get 2
          local.get 4
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          local.set 1
          br 2 (;@1;)
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 5
        local.get 4
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 4
        block ;; label = @3
          local.get 1
          i32.const 65535
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          i32.store8 offset=14
          local.get 2
          local.get 4
          i32.store8 offset=13
          local.get 2
          local.get 5
          i32.const 224
          i32.or
          i32.store8 offset=12
          i32.const 3
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 3
        i32.store8 offset=15
        local.get 2
        local.get 4
        i32.store8 offset=14
        local.get 2
        local.get 5
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8 offset=12
        i32.const 4
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_RNvXNvNtCset5xJoy1xWQ_3std2io17default_write_fmtINtB2_7AdapterNtNtNtNtB6_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_strB6_
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 1
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std2io17default_write_fmt7AdapterNtNtNtNtB9_3sys5stdio4unix6StderrENtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtB9_ (;112;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050916
    local.get 1
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvYINtNvNtCset5xJoy1xWQ_3std9panicking11begin_panic7PayloadReENtNtCsdHhIpgkcIfN_4core5panic12PanicPayload6as_strB9_ (;113;) (type 2) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_RNvYNtNtCsi9YzqDQQz2q_5alloc6string6StringNtNtCsdHhIpgkcIfN_4core3fmt5Write9write_fmtCset5xJoy1xWQ_3std (;114;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1051072
    local.get 1
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvYNtNtNtNtCset5xJoy1xWQ_3std3sys5stdio4unix6StderrNtNtBa_2io5Write9write_allBa_ (;115;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          loop ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    i32.const 2
                    local.get 2
                    local.get 3
                    call $write
                    local.tee 5
                    i32.const -1
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 4
                    i32.const 0
                    i32.store8 offset=11
                    local.get 4
                    i32.const 0
                    i32.store16 offset=9 align=1
                    local.get 4
                    i32.const 0
                    i32.store8 offset=8
                    local.get 4
                    i32.const 0
                    i32.load offset=1055972
                    local.tee 5
                    i32.store offset=12
                    local.get 5
                    i32.const 27
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 4
                    i32.const 8
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 5
                  i32.store offset=12
                  local.get 4
                  i32.const 4
                  i32.store8 offset=8
                  local.get 5
                  br_if 1 (;@6;)
                  i32.const 1052656
                  local.set 5
                end
                local.get 0
                local.get 5
                i64.load
                i64.store align=4
                br 5 (;@1;)
              end
              local.get 3
              local.get 5
              i32.lt_u
              br_if 3 (;@2;)
              local.get 2
              local.get 5
              i32.add
              local.set 2
              local.get 3
              local.get 5
              i32.sub
              local.set 3
            end
            local.get 3
            br_if 0 (;@4;)
          end
        end
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 5
      local.get 3
      local.get 3
      i32.const 1053048
      call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
      unreachable
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $writev (;116;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const -1
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const -1
        i32.gt_s
        br_if 0 (;@2;)
        i32.const 0
        i32.const 28
        i32.store offset=1055972
        br 1 (;@1;)
      end
      local.get 3
      i32.const 0
      i32.store offset=12
      block ;; label = @2
        local.get 0
        local.get 1
        local.get 2
        local.get 3
        i32.const 12
        i32.add
        call $__wasi_fd_write
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 2
        i32.store offset=1055972
        i32.const -1
        local.set 4
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=12
      local.set 4
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $write (;117;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.get 3
        i32.const 8
        i32.add
        i32.const 1
        local.get 3
        i32.const 4
        i32.add
        call $__wasi_fd_write
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        i32.const 8
        local.get 2
        local.get 2
        i32.const 76
        i32.eq
        select
        i32.store offset=1055972
        i32.const -1
        local.set 2
        br 1 (;@1;)
      end
      local.get 3
      i32.load offset=4
      local.set 2
    end
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )
  (func $_Exit (;118;) (type 1) (param i32)
    local.get 0
    call $__wasi_proc_exit
    unreachable
  )
  (func $__wasilibc_ensure_environ (;119;) (type 0)
    block ;; label = @1
      i32.const 0
      i32.load offset=1055836
      i32.const -1
      i32.ne
      br_if 0 (;@1;)
      call $__wasilibc_initialize_environ
    end
  )
  (func $__wasilibc_initialize_environ (;120;) (type 0)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 0
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 12
        i32.add
        local.get 0
        i32.const 8
        i32.add
        call $__wasi_environ_sizes_get
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          i32.load offset=12
          local.tee 1
          br_if 0 (;@3;)
          i32.const 1055976
          local.set 1
          br 2 (;@1;)
        end
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 1
            i32.add
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.load offset=8
            call $malloc
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.const 4
            call $calloc
            local.tee 1
            br_if 1 (;@3;)
            local.get 2
            call $free
          end
          i32.const 70
          call $_Exit
          unreachable
        end
        local.get 1
        local.get 2
        call $__wasi_environ_get
        i32.eqz
        br_if 1 (;@1;)
        local.get 2
        call $free
        local.get 1
        call $free
      end
      i32.const 71
      call $_Exit
      unreachable
    end
    i32.const 0
    local.get 1
    i32.store offset=1055836
    local.get 0
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (func $__wasi_environ_get (;121;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_environ_sizes_get (;122;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__imported_wasi_snapshot_preview1_environ_sizes_get
    i32.const 65535
    i32.and
  )
  (func $__wasi_fd_write (;123;) (type 7) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call $__imported_wasi_snapshot_preview1_fd_write
    i32.const 65535
    i32.and
  )
  (func $__wasi_proc_exit (;124;) (type 1) (param i32)
    local.get 0
    call $__imported_wasi_snapshot_preview1_proc_exit
    unreachable
  )
  (func $abort (;125;) (type 0)
    unreachable
  )
  (func $getcwd (;126;) (type 5) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    i32.load offset=1055840
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        local.get 2
        call $strdup
        local.tee 0
        br_if 1 (;@1;)
        i32.const 0
        i32.const 48
        i32.store offset=1055972
        i32.const 0
        return
      end
      block ;; label = @2
        local.get 1
        local.get 2
        call $strlen
        i32.const 1
        i32.add
        i32.ge_u
        br_if 0 (;@2;)
        i32.const 0
        i32.const 68
        i32.store offset=1055972
        i32.const 0
        return
      end
      local.get 0
      local.get 2
      call $strcpy
      local.set 0
    end
    local.get 0
  )
  (func $__wasi_init_tp (;127;) (type 0)
    (local i32 i32)
    i32.const 0
    i32.const 1055984
    i32.store offset=1055984
    i32.const 1048576
    local.set 0
    block ;; label = @1
      block ;; label = @2
        i32.const 1048576
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1048576
        i32.const 0
        i32.sub
        local.set 1
        br 1 (;@1;)
      end
      global.get $__stack_pointer
      local.set 1
      i32.const 1056624
      i32.const 1056616
      i32.sub
      i32.const 1048576
      local.get 1
      i32.const 1048576
      i32.gt_u
      local.tee 0
      select
      local.set 1
      i32.const 1056624
      i32.const 1048576
      local.get 0
      select
      local.set 0
    end
    i32.const 56
    i32.const 0
    i32.store offset=1055984
    i32.const 52
    local.get 1
    i32.store offset=1055984
    i32.const 48
    local.get 0
    i32.store offset=1055984
    i32.const 8
    i32.const 1055984
    i32.store offset=1055984
    i32.const 4
    i32.const 1055984
    i32.store offset=1055984
    i32.const 12
    i32.const 0
    i32.load offset=1055980
    i32.store offset=1055984
    i32.const 0
    local.get 1
    i32.const 8388608
    local.get 1
    i32.const 8388608
    i32.lt_u
    select
    i32.store offset=1055844
  )
  (func $getenv (;128;) (type 6) (param i32) (result i32)
    (local i32 i32 i32 i32)
    call $__wasilibc_ensure_environ
    block ;; label = @1
      local.get 0
      i32.const 61
      call $__strchrnul
      local.tee 1
      local.get 0
      i32.ne
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    i32.const 0
    local.set 2
    block ;; label = @1
      local.get 0
      local.get 1
      local.get 0
      i32.sub
      local.tee 3
      i32.add
      i32.load8_u
      br_if 0 (;@1;)
      i32.const 0
      i32.load offset=1055836
      local.tee 4
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.load
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 4
      i32.const 4
      i32.add
      local.set 4
      block ;; label = @2
        loop ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 1
            local.get 3
            call $strncmp
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.add
            local.tee 1
            i32.load8_u
            i32.const 61
            i32.eq
            br_if 2 (;@2;)
          end
          local.get 4
          i32.load
          local.set 1
          local.get 4
          i32.const 4
          i32.add
          local.set 4
          local.get 1
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 2
    end
    local.get 2
  )
  (func $dummy (;129;) (type 0))
  (func $__wasm_call_dtors (;130;) (type 0)
    call $dummy
    call $dummy
  )
  (func $__strchrnul (;131;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 255
            i32.and
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            i32.const 3
            i32.and
            i32.eqz
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 0
              i32.load8_u
              local.tee 3
              br_if 0 (;@5;)
              local.get 0
              return
            end
            local.get 3
            local.get 1
            i32.const 255
            i32.and
            i32.ne
            br_if 1 (;@3;)
            local.get 0
            return
          end
          local.get 0
          local.get 0
          call $strlen
          i32.add
          return
        end
        block ;; label = @3
          local.get 0
          i32.const 1
          i32.add
          local.tee 3
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 0
          br 1 (;@2;)
        end
        local.get 3
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.const 255
        i32.and
        i32.eq
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 0
          i32.const 2
          i32.add
          local.tee 3
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 0
          br 1 (;@2;)
        end
        local.get 3
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.const 255
        i32.and
        i32.eq
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 0
          i32.const 3
          i32.add
          local.tee 3
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.set 0
          br 1 (;@2;)
        end
        local.get 3
        i32.load8_u
        local.tee 4
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        i32.const 255
        i32.and
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.set 0
      end
      block ;; label = @2
        i32.const 16843008
        local.get 0
        i32.load
        local.tee 3
        i32.sub
        local.get 3
        i32.or
        i32.const -2139062144
        i32.and
        i32.const -2139062144
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.const 16843009
        i32.mul
        local.set 2
        loop ;; label = @3
          i32.const 16843008
          local.get 3
          local.get 2
          i32.xor
          local.tee 3
          i32.sub
          local.get 3
          i32.or
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.ne
          br_if 1 (;@2;)
          i32.const 16843008
          local.get 0
          i32.const 4
          i32.add
          local.tee 0
          i32.load
          local.tee 3
          i32.sub
          local.get 3
          i32.or
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.eq
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.const -1
      i32.add
      local.set 3
      loop ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        i32.load8_u
        local.tee 0
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.const 255
        i32.and
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 3
  )
  (func $__stpcpy (;132;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          local.get 0
          i32.xor
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load8_u
          local.set 2
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 1
          i32.const 3
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 1
          i32.load8_u
          local.tee 2
          i32.store8
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            local.get 0
            return
          end
          local.get 0
          i32.const 1
          i32.add
          local.set 3
          block ;; label = @4
            local.get 1
            i32.const 1
            i32.add
            local.tee 2
            i32.const 3
            i32.and
            br_if 0 (;@4;)
            local.get 3
            local.set 0
            local.get 2
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.load8_u
          local.tee 2
          i32.store8
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.const 2
          i32.add
          local.set 3
          block ;; label = @4
            local.get 1
            i32.const 2
            i32.add
            local.tee 2
            i32.const 3
            i32.and
            br_if 0 (;@4;)
            local.get 3
            local.set 0
            local.get 2
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.load8_u
          local.tee 2
          i32.store8
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.const 3
          i32.add
          local.set 3
          block ;; label = @4
            local.get 1
            i32.const 3
            i32.add
            local.tee 2
            i32.const 3
            i32.and
            br_if 0 (;@4;)
            local.get 3
            local.set 0
            local.get 2
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.load8_u
          local.tee 2
          i32.store8
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          local.get 1
          i32.const 4
          i32.add
          local.set 1
        end
        i32.const 16843008
        local.get 1
        i32.load
        local.tee 2
        i32.sub
        local.get 2
        i32.or
        i32.const -2139062144
        i32.and
        i32.const -2139062144
        i32.ne
        br_if 0 (;@2;)
        loop ;; label = @3
          local.get 0
          local.get 2
          i32.store
          local.get 0
          i32.const 4
          i32.add
          local.set 0
          i32.const 16843008
          local.get 1
          i32.const 4
          i32.add
          local.tee 1
          i32.load
          local.tee 2
          i32.sub
          local.get 2
          i32.or
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.eq
          br_if 0 (;@3;)
        end
      end
      local.get 0
      local.get 2
      i32.store8
      block ;; label = @2
        local.get 2
        i32.const 255
        i32.and
        br_if 0 (;@2;)
        local.get 0
        return
      end
      local.get 1
      i32.const 1
      i32.add
      local.set 2
      local.get 0
      local.set 3
      loop ;; label = @2
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 2
        i32.load8_u
        local.tee 0
        i32.store8
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 0
        br_if 0 (;@2;)
      end
    end
    local.get 3
  )
  (func $strcpy (;133;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $__stpcpy
    drop
    local.get 0
  )
  (func $strdup (;134;) (type 6) (param i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      local.get 0
      call $strlen
      i32.const 1
      i32.add
      local.tee 1
      call $malloc
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      local.get 0
      local.get 1
      memory.copy
    end
    local.get 2
  )
  (func $"#func135 dummy" (@name "dummy") (;135;) (type 5) (param i32 i32) (result i32)
    local.get 0
  )
  (func $__lctrans (;136;) (type 5) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $"#func135 dummy"
  )
  (func $strerror (;137;) (type 6) (param i32) (result i32)
    (local i32)
    block ;; label = @1
      i32.const 0
      i32.load offset=1056116
      local.tee 1
      br_if 0 (;@1;)
      i32.const 1056092
      local.set 1
      i32.const 0
      i32.const 1056092
      i32.store offset=1056116
    end
    i32.const 0
    local.get 0
    local.get 0
    i32.const 76
    i32.gt_u
    select
    i32.const 1
    i32.shl
    i32.load16_u offset=1054960
    i32.const 1053400
    i32.add
    local.get 1
    i32.load offset=20
    call $__lctrans
  )
  (func $strerror_r (;138;) (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        call $strerror
        local.tee 3
        call $strlen
        local.tee 0
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 68
        local.set 0
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 3
          local.get 2
          memory.copy
        end
        local.get 1
        local.get 2
        i32.add
        i32.const 0
        i32.store8
        i32.const 68
        return
      end
      block ;; label = @2
        local.get 0
        i32.const 1
        i32.add
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        local.get 2
        memory.copy
      end
      i32.const 0
      local.set 0
    end
    local.get 0
  )
  (func $strlen (;139;) (type 6) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          i32.load8_u
          br_if 0 (;@3;)
          local.get 0
          local.get 0
          i32.sub
          return
        end
        local.get 0
        i32.const 1
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 2
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 3
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.const 4
        i32.add
        local.tee 1
        i32.const 3
        i32.and
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const -4
      i32.add
      local.set 2
      local.get 1
      i32.const -5
      i32.add
      local.set 1
      loop ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        local.set 1
        i32.const 16843008
        local.get 2
        i32.const 4
        i32.add
        local.tee 2
        i32.load
        local.tee 3
        i32.sub
        local.get 3
        i32.or
        i32.const -2139062144
        i32.and
        i32.const -2139062144
        i32.eq
        br_if 0 (;@2;)
      end
      loop ;; label = @2
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.load8_u
        local.set 3
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        local.get 3
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub
  )
  (func $strncmp (;140;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      local.get 2
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load8_u
        local.tee 3
        br_if 0 (;@2;)
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      local.get 2
      i32.const -1
      i32.add
      local.set 2
      block ;; label = @2
        loop ;; label = @3
          local.get 3
          i32.const 255
          i32.and
          local.get 1
          i32.load8_u
          local.tee 4
          i32.ne
          br_if 1 (;@2;)
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          i32.const 0
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.const -1
          i32.add
          local.set 2
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 0
          i32.load8_u
          local.set 3
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 3
          br_if 0 (;@3;)
        end
        i32.const 0
        local.set 3
      end
      local.get 3
      i32.const 255
      i32.and
      local.set 3
    end
    local.get 3
    local.get 1
    i32.load8_u
    i32.sub
  )
  (func $sbrk (;141;) (type 6) (param i32) (result i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      memory.size
      i32.const 16
      i32.shl
      return
    end
    block ;; label = @1
      local.get 0
      i32.const 65535
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.const -1
      i32.le_s
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 0
        i32.const 16
        i32.shr_u
        memory.grow
        local.tee 0
        i32.const -1
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        i32.const 48
        i32.store offset=1055972
        i32.const -1
        return
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    end
    call $abort
    unreachable
  )
  (func $malloc (;142;) (type 6) (param i32) (result i32)
    local.get 0
    call $dlmalloc
  )
  (func $dlmalloc (;143;) (type 6) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1056144
                              local.tee 2
                              br_if 0 (;@13;)
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1056592
                                local.tee 3
                                br_if 0 (;@14;)
                                i32.const 0
                                i64.const -1
                                i64.store offset=1056604 align=4
                                i32.const 0
                                i64.const 281474976776192
                                i64.store offset=1056596 align=4
                                i32.const 0
                                local.get 1
                                i32.const 8
                                i32.add
                                i32.const -16
                                i32.and
                                i32.const 1431655768
                                i32.xor
                                local.tee 3
                                i32.store offset=1056592
                                i32.const 0
                                i32.const 0
                                i32.store offset=1056612
                                i32.const 0
                                i32.const 0
                                i32.store offset=1056564
                              end
                              i32.const 1114112
                              i32.const 1056624
                              i32.lt_u
                              br_if 1 (;@12;)
                              i32.const 0
                              local.set 2
                              i32.const 1114112
                              i32.const 1056624
                              i32.sub
                              i32.const 89
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 4
                              i32.const 0
                              i32.const 1056624
                              i32.store offset=1056568
                              i32.const 0
                              i32.const 1056624
                              i32.store offset=1056136
                              i32.const 0
                              local.get 3
                              i32.store offset=1056156
                              i32.const 0
                              i32.const -1
                              i32.store offset=1056152
                              i32.const 0
                              i32.const 1114112
                              i32.const 1056624
                              i32.sub
                              local.tee 3
                              i32.store offset=1056572
                              i32.const 0
                              local.get 3
                              i32.store offset=1056556
                              i32.const 0
                              local.get 3
                              i32.store offset=1056552
                              loop ;; label = @14
                                local.get 4
                                i32.const 1056180
                                i32.add
                                local.get 4
                                i32.const 1056168
                                i32.add
                                local.tee 3
                                i32.store
                                local.get 3
                                local.get 4
                                i32.const 1056160
                                i32.add
                                local.tee 5
                                i32.store
                                local.get 4
                                i32.const 1056172
                                i32.add
                                local.get 5
                                i32.store
                                local.get 4
                                i32.const 1056188
                                i32.add
                                local.get 4
                                i32.const 1056176
                                i32.add
                                local.tee 5
                                i32.store
                                local.get 5
                                local.get 3
                                i32.store
                                local.get 4
                                i32.const 1056196
                                i32.add
                                local.get 4
                                i32.const 1056184
                                i32.add
                                local.tee 3
                                i32.store
                                local.get 3
                                local.get 5
                                i32.store
                                local.get 4
                                i32.const 1056192
                                i32.add
                                local.get 3
                                i32.store
                                local.get 4
                                i32.const 32
                                i32.add
                                local.tee 4
                                i32.const 256
                                i32.ne
                                br_if 0 (;@14;)
                              end
                              i32.const 1114112
                              i32.const -52
                              i32.add
                              i32.const 56
                              i32.store
                              i32.const 0
                              i32.const 0
                              i32.load offset=1056608
                              i32.store offset=1056148
                              i32.const 0
                              i32.const 1056624
                              i32.const -8
                              i32.const 1056624
                              i32.sub
                              i32.const 15
                              i32.and
                              local.tee 4
                              i32.add
                              local.tee 2
                              i32.store offset=1056144
                              i32.const 0
                              i32.const 1114112
                              i32.const 1056624
                              i32.sub
                              local.get 4
                              i32.sub
                              i32.const -56
                              i32.add
                              local.tee 4
                              i32.store offset=1056132
                              local.get 2
                              local.get 4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 0
                                i32.const 236
                                i32.gt_u
                                br_if 0 (;@14;)
                                block ;; label = @15
                                  i32.const 0
                                  i32.load offset=1056120
                                  local.tee 6
                                  i32.const 16
                                  local.get 0
                                  i32.const 19
                                  i32.add
                                  i32.const 496
                                  i32.and
                                  local.get 0
                                  i32.const 11
                                  i32.lt_u
                                  select
                                  local.tee 5
                                  i32.const 3
                                  i32.shr_u
                                  local.tee 3
                                  i32.shr_u
                                  local.tee 4
                                  i32.const 3
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 4
                                      i32.const 1
                                      i32.and
                                      local.get 3
                                      i32.or
                                      i32.const 1
                                      i32.xor
                                      local.tee 5
                                      i32.const 3
                                      i32.shl
                                      local.tee 3
                                      i32.const 1056160
                                      i32.add
                                      local.tee 4
                                      local.get 3
                                      i32.load offset=1056168
                                      local.tee 3
                                      i32.load offset=8
                                      local.tee 0
                                      i32.ne
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.get 6
                                      i32.const -2
                                      local.get 5
                                      i32.rotl
                                      i32.and
                                      i32.store offset=1056120
                                      br 1 (;@16;)
                                    end
                                    local.get 4
                                    local.get 0
                                    i32.store offset=8
                                    local.get 0
                                    local.get 4
                                    i32.store offset=12
                                  end
                                  local.get 3
                                  i32.const 8
                                  i32.add
                                  local.set 4
                                  local.get 3
                                  local.get 5
                                  i32.const 3
                                  i32.shl
                                  local.tee 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 5
                                  i32.add
                                  local.tee 3
                                  local.get 3
                                  i32.load offset=4
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  br 14 (;@1;)
                                end
                                local.get 5
                                i32.const 0
                                i32.load offset=1056128
                                local.tee 7
                                i32.le_u
                                br_if 1 (;@13;)
                                block ;; label = @15
                                  local.get 4
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 4
                                      local.get 3
                                      i32.shl
                                      i32.const 2
                                      local.get 3
                                      i32.shl
                                      local.tee 4
                                      i32.const 0
                                      local.get 4
                                      i32.sub
                                      i32.or
                                      i32.and
                                      i32.ctz
                                      local.tee 3
                                      i32.const 3
                                      i32.shl
                                      local.tee 4
                                      i32.const 1056160
                                      i32.add
                                      local.tee 0
                                      local.get 4
                                      i32.load offset=1056168
                                      local.tee 4
                                      i32.load offset=8
                                      local.tee 8
                                      i32.ne
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.get 6
                                      i32.const -2
                                      local.get 3
                                      i32.rotl
                                      i32.and
                                      local.tee 6
                                      i32.store offset=1056120
                                      br 1 (;@16;)
                                    end
                                    local.get 0
                                    local.get 8
                                    i32.store offset=8
                                    local.get 8
                                    local.get 0
                                    i32.store offset=12
                                  end
                                  local.get 4
                                  local.get 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  local.get 4
                                  local.get 3
                                  i32.const 3
                                  i32.shl
                                  local.tee 3
                                  i32.add
                                  local.get 3
                                  local.get 5
                                  i32.sub
                                  local.tee 0
                                  i32.store
                                  local.get 4
                                  local.get 5
                                  i32.add
                                  local.tee 8
                                  local.get 0
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  block ;; label = @16
                                    local.get 7
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 7
                                    i32.const -8
                                    i32.and
                                    i32.const 1056160
                                    i32.add
                                    local.set 5
                                    i32.const 0
                                    i32.load offset=1056140
                                    local.set 3
                                    block ;; label = @17
                                      block ;; label = @18
                                        local.get 6
                                        i32.const 1
                                        local.get 7
                                        i32.const 3
                                        i32.shr_u
                                        i32.shl
                                        local.tee 9
                                        i32.and
                                        br_if 0 (;@18;)
                                        i32.const 0
                                        local.get 6
                                        local.get 9
                                        i32.or
                                        i32.store offset=1056120
                                        local.get 5
                                        local.set 9
                                        br 1 (;@17;)
                                      end
                                      local.get 5
                                      i32.load offset=8
                                      local.set 9
                                    end
                                    local.get 9
                                    local.get 3
                                    i32.store offset=12
                                    local.get 5
                                    local.get 3
                                    i32.store offset=8
                                    local.get 3
                                    local.get 5
                                    i32.store offset=12
                                    local.get 3
                                    local.get 9
                                    i32.store offset=8
                                  end
                                  local.get 4
                                  i32.const 8
                                  i32.add
                                  local.set 4
                                  i32.const 0
                                  local.get 8
                                  i32.store offset=1056140
                                  i32.const 0
                                  local.get 0
                                  i32.store offset=1056128
                                  br 14 (;@1;)
                                end
                                i32.const 0
                                i32.load offset=1056124
                                local.tee 10
                                i32.eqz
                                br_if 1 (;@13;)
                                local.get 10
                                i32.ctz
                                i32.const 2
                                i32.shl
                                i32.load offset=1056424
                                local.tee 8
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 5
                                i32.sub
                                local.set 3
                                local.get 8
                                local.set 0
                                block ;; label = @15
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 0
                                      i32.load offset=16
                                      local.tee 4
                                      br_if 0 (;@17;)
                                      local.get 0
                                      i32.load offset=20
                                      local.tee 4
                                      i32.eqz
                                      br_if 2 (;@15;)
                                    end
                                    local.get 4
                                    i32.load offset=4
                                    i32.const -8
                                    i32.and
                                    local.get 5
                                    i32.sub
                                    local.tee 0
                                    local.get 3
                                    local.get 0
                                    local.get 3
                                    i32.lt_u
                                    local.tee 0
                                    select
                                    local.set 3
                                    local.get 4
                                    local.get 8
                                    local.get 0
                                    select
                                    local.set 8
                                    local.get 4
                                    local.set 0
                                    br 0 (;@16;)
                                  end
                                end
                                local.get 8
                                i32.load offset=24
                                local.set 2
                                block ;; label = @15
                                  local.get 8
                                  i32.load offset=12
                                  local.tee 4
                                  local.get 8
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 8
                                  i32.load offset=8
                                  local.tee 0
                                  local.get 4
                                  i32.store offset=12
                                  local.get 4
                                  local.get 0
                                  i32.store offset=8
                                  br 13 (;@2;)
                                end
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 8
                                    i32.load offset=20
                                    local.tee 0
                                    i32.eqz
                                    br_if 0 (;@16;)
                                    local.get 8
                                    i32.const 20
                                    i32.add
                                    local.set 9
                                    br 1 (;@15;)
                                  end
                                  local.get 8
                                  i32.load offset=16
                                  local.tee 0
                                  i32.eqz
                                  br_if 4 (;@11;)
                                  local.get 8
                                  i32.const 16
                                  i32.add
                                  local.set 9
                                end
                                loop ;; label = @15
                                  local.get 9
                                  local.set 11
                                  local.get 0
                                  local.tee 4
                                  i32.const 20
                                  i32.add
                                  local.set 9
                                  local.get 4
                                  i32.load offset=20
                                  local.tee 0
                                  br_if 0 (;@15;)
                                  local.get 4
                                  i32.const 16
                                  i32.add
                                  local.set 9
                                  local.get 4
                                  i32.load offset=16
                                  local.tee 0
                                  br_if 0 (;@15;)
                                end
                                local.get 11
                                i32.const 0
                                i32.store
                                br 12 (;@2;)
                              end
                              i32.const -1
                              local.set 5
                              local.get 0
                              i32.const -65
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 0
                              i32.const 19
                              i32.add
                              local.tee 4
                              i32.const -16
                              i32.and
                              local.set 5
                              i32.const 0
                              i32.load offset=1056124
                              local.tee 10
                              i32.eqz
                              br_if 0 (;@13;)
                              i32.const 31
                              local.set 7
                              block ;; label = @14
                                local.get 0
                                i32.const 16777196
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 5
                                i32.const 38
                                local.get 4
                                i32.const 8
                                i32.shr_u
                                i32.clz
                                local.tee 4
                                i32.sub
                                i32.shr_u
                                i32.const 1
                                i32.and
                                local.get 4
                                i32.const 1
                                i32.shl
                                i32.sub
                                i32.const 62
                                i32.add
                                local.set 7
                              end
                              i32.const 0
                              local.get 5
                              i32.sub
                              local.set 3
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      local.get 7
                                      i32.const 2
                                      i32.shl
                                      i32.load offset=1056424
                                      local.tee 0
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.set 4
                                      i32.const 0
                                      local.set 9
                                      br 1 (;@16;)
                                    end
                                    i32.const 0
                                    local.set 4
                                    local.get 5
                                    i32.const 0
                                    i32.const 25
                                    local.get 7
                                    i32.const 1
                                    i32.shr_u
                                    i32.sub
                                    local.get 7
                                    i32.const 31
                                    i32.eq
                                    select
                                    i32.shl
                                    local.set 8
                                    i32.const 0
                                    local.set 9
                                    loop ;; label = @17
                                      block ;; label = @18
                                        local.get 0
                                        i32.load offset=4
                                        i32.const -8
                                        i32.and
                                        local.get 5
                                        i32.sub
                                        local.tee 6
                                        local.get 3
                                        i32.ge_u
                                        br_if 0 (;@18;)
                                        local.get 6
                                        local.set 3
                                        local.get 0
                                        local.set 9
                                        local.get 6
                                        br_if 0 (;@18;)
                                        i32.const 0
                                        local.set 3
                                        local.get 0
                                        local.set 9
                                        local.get 0
                                        local.set 4
                                        br 3 (;@15;)
                                      end
                                      local.get 4
                                      local.get 0
                                      i32.load offset=20
                                      local.tee 6
                                      local.get 6
                                      local.get 0
                                      local.get 8
                                      i32.const 29
                                      i32.shr_u
                                      i32.const 4
                                      i32.and
                                      i32.add
                                      i32.load offset=16
                                      local.tee 11
                                      i32.eq
                                      select
                                      local.get 4
                                      local.get 6
                                      select
                                      local.set 4
                                      local.get 8
                                      i32.const 1
                                      i32.shl
                                      local.set 8
                                      local.get 11
                                      local.set 0
                                      local.get 11
                                      br_if 0 (;@17;)
                                    end
                                  end
                                  block ;; label = @16
                                    local.get 4
                                    local.get 9
                                    i32.or
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 9
                                    i32.const 2
                                    local.get 7
                                    i32.shl
                                    local.tee 4
                                    i32.const 0
                                    local.get 4
                                    i32.sub
                                    i32.or
                                    local.get 10
                                    i32.and
                                    local.tee 4
                                    i32.eqz
                                    br_if 3 (;@13;)
                                    local.get 4
                                    i32.ctz
                                    i32.const 2
                                    i32.shl
                                    i32.load offset=1056424
                                    local.set 4
                                  end
                                  local.get 4
                                  i32.eqz
                                  br_if 1 (;@14;)
                                end
                                loop ;; label = @15
                                  local.get 4
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 5
                                  i32.sub
                                  local.tee 6
                                  local.get 3
                                  i32.lt_u
                                  local.set 8
                                  block ;; label = @16
                                    local.get 4
                                    i32.load offset=16
                                    local.tee 0
                                    br_if 0 (;@16;)
                                    local.get 4
                                    i32.load offset=20
                                    local.set 0
                                  end
                                  local.get 6
                                  local.get 3
                                  local.get 8
                                  select
                                  local.set 3
                                  local.get 4
                                  local.get 9
                                  local.get 8
                                  select
                                  local.set 9
                                  local.get 0
                                  local.set 4
                                  local.get 0
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 9
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 3
                              i32.const 0
                              i32.load offset=1056128
                              local.get 5
                              i32.sub
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 9
                              i32.load offset=24
                              local.set 11
                              block ;; label = @14
                                local.get 9
                                i32.load offset=12
                                local.tee 4
                                local.get 9
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 9
                                i32.load offset=8
                                local.tee 0
                                local.get 4
                                i32.store offset=12
                                local.get 4
                                local.get 0
                                i32.store offset=8
                                br 11 (;@3;)
                              end
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 9
                                  i32.load offset=20
                                  local.tee 0
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 9
                                  i32.const 20
                                  i32.add
                                  local.set 8
                                  br 1 (;@14;)
                                end
                                local.get 9
                                i32.load offset=16
                                local.tee 0
                                i32.eqz
                                br_if 4 (;@10;)
                                local.get 9
                                i32.const 16
                                i32.add
                                local.set 8
                              end
                              loop ;; label = @14
                                local.get 8
                                local.set 6
                                local.get 0
                                local.tee 4
                                i32.const 20
                                i32.add
                                local.set 8
                                local.get 4
                                i32.load offset=20
                                local.tee 0
                                br_if 0 (;@14;)
                                local.get 4
                                i32.const 16
                                i32.add
                                local.set 8
                                local.get 4
                                i32.load offset=16
                                local.tee 0
                                br_if 0 (;@14;)
                              end
                              local.get 6
                              i32.const 0
                              i32.store
                              br 10 (;@3;)
                            end
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1056128
                              local.tee 4
                              local.get 5
                              i32.lt_u
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.load offset=1056140
                              local.set 3
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 4
                                  local.get 5
                                  i32.sub
                                  local.tee 0
                                  i32.const 16
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 3
                                  local.get 5
                                  i32.add
                                  local.tee 8
                                  local.get 0
                                  i32.const 1
                                  i32.or
                                  i32.store offset=4
                                  local.get 3
                                  local.get 4
                                  i32.add
                                  local.get 0
                                  i32.store
                                  local.get 3
                                  local.get 5
                                  i32.const 3
                                  i32.or
                                  i32.store offset=4
                                  br 1 (;@14;)
                                end
                                local.get 3
                                local.get 4
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 3
                                local.get 4
                                i32.add
                                local.tee 4
                                local.get 4
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                i32.const 0
                                local.set 8
                                i32.const 0
                                local.set 0
                              end
                              i32.const 0
                              local.get 0
                              i32.store offset=1056128
                              i32.const 0
                              local.get 8
                              i32.store offset=1056140
                              local.get 3
                              i32.const 8
                              i32.add
                              local.set 4
                              br 12 (;@1;)
                            end
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1056132
                              local.tee 0
                              local.get 5
                              i32.le_u
                              br_if 0 (;@13;)
                              local.get 2
                              local.get 5
                              i32.add
                              local.tee 4
                              local.get 0
                              local.get 5
                              i32.sub
                              local.tee 3
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.get 4
                              i32.store offset=1056144
                              i32.const 0
                              local.get 3
                              i32.store offset=1056132
                              local.get 2
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 2
                              i32.const 8
                              i32.add
                              local.set 4
                              br 12 (;@1;)
                            end
                            block ;; label = @13
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1056592
                                i32.eqz
                                br_if 0 (;@14;)
                                i32.const 0
                                i32.load offset=1056600
                                local.set 3
                                br 1 (;@13;)
                              end
                              i32.const 0
                              i64.const -1
                              i64.store offset=1056604 align=4
                              i32.const 0
                              i64.const 281474976776192
                              i64.store offset=1056596 align=4
                              i32.const 0
                              local.get 1
                              i32.const 12
                              i32.add
                              i32.const -16
                              i32.and
                              i32.const 1431655768
                              i32.xor
                              i32.store offset=1056592
                              i32.const 0
                              i32.const 0
                              i32.store offset=1056612
                              i32.const 0
                              i32.const 0
                              i32.store offset=1056564
                              i32.const 65536
                              local.set 3
                            end
                            i32.const 0
                            local.set 4
                            block ;; label = @13
                              local.get 3
                              local.get 5
                              i32.const 71
                              i32.add
                              local.tee 11
                              i32.add
                              local.tee 8
                              i32.const 0
                              local.get 3
                              i32.sub
                              local.tee 6
                              i32.and
                              local.tee 9
                              local.get 5
                              i32.gt_u
                              br_if 0 (;@13;)
                              i32.const 0
                              i32.const 48
                              i32.store offset=1055972
                              br 12 (;@1;)
                            end
                            block ;; label = @13
                              i32.const 0
                              i32.load offset=1056560
                              local.tee 4
                              i32.eqz
                              br_if 0 (;@13;)
                              block ;; label = @14
                                i32.const 0
                                i32.load offset=1056552
                                local.tee 3
                                local.get 9
                                i32.add
                                local.tee 7
                                local.get 3
                                i32.le_u
                                br_if 0 (;@14;)
                                local.get 7
                                local.get 4
                                i32.le_u
                                br_if 1 (;@13;)
                              end
                              i32.const 0
                              local.set 4
                              i32.const 0
                              i32.const 48
                              i32.store offset=1055972
                              br 12 (;@1;)
                            end
                            i32.const 0
                            i32.load8_u offset=1056564
                            i32.const 4
                            i32.and
                            br_if 5 (;@7;)
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 2
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  i32.const 1056568
                                  local.set 4
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 2
                                      local.get 4
                                      i32.load
                                      local.tee 3
                                      i32.lt_u
                                      br_if 0 (;@17;)
                                      local.get 2
                                      local.get 3
                                      local.get 4
                                      i32.load offset=4
                                      i32.add
                                      i32.lt_u
                                      br_if 3 (;@14;)
                                    end
                                    local.get 4
                                    i32.load offset=8
                                    local.tee 4
                                    br_if 0 (;@16;)
                                  end
                                end
                                i32.const 0
                                call $sbrk
                                local.tee 8
                                i32.const -1
                                i32.eq
                                br_if 6 (;@8;)
                                local.get 9
                                local.set 6
                                block ;; label = @15
                                  i32.const 0
                                  i32.load offset=1056596
                                  local.tee 4
                                  i32.const -1
                                  i32.add
                                  local.tee 3
                                  local.get 8
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 9
                                  local.get 8
                                  i32.sub
                                  local.get 3
                                  local.get 8
                                  i32.add
                                  i32.const 0
                                  local.get 4
                                  i32.sub
                                  i32.and
                                  i32.add
                                  local.set 6
                                end
                                local.get 6
                                local.get 5
                                i32.le_u
                                br_if 6 (;@8;)
                                local.get 6
                                i32.const 2147483646
                                i32.gt_u
                                br_if 6 (;@8;)
                                block ;; label = @15
                                  i32.const 0
                                  i32.load offset=1056560
                                  local.tee 4
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  i32.load offset=1056552
                                  local.tee 3
                                  local.get 6
                                  i32.add
                                  local.tee 0
                                  local.get 3
                                  i32.le_u
                                  br_if 7 (;@8;)
                                  local.get 0
                                  local.get 4
                                  i32.gt_u
                                  br_if 7 (;@8;)
                                end
                                local.get 6
                                call $sbrk
                                local.tee 4
                                local.get 8
                                i32.ne
                                br_if 1 (;@13;)
                                br 8 (;@6;)
                              end
                              local.get 8
                              local.get 0
                              i32.sub
                              local.get 6
                              i32.and
                              local.tee 6
                              i32.const 2147483646
                              i32.gt_u
                              br_if 5 (;@8;)
                              local.get 6
                              call $sbrk
                              local.tee 8
                              local.get 4
                              i32.load
                              local.get 4
                              i32.load offset=4
                              i32.add
                              i32.eq
                              br_if 4 (;@9;)
                              local.get 8
                              local.set 4
                            end
                            block ;; label = @13
                              local.get 6
                              local.get 5
                              i32.const 72
                              i32.add
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const -1
                              i32.eq
                              br_if 0 (;@13;)
                              block ;; label = @14
                                local.get 11
                                local.get 6
                                i32.sub
                                i32.const 0
                                i32.load offset=1056600
                                local.tee 3
                                i32.add
                                i32.const 0
                                local.get 3
                                i32.sub
                                i32.and
                                local.tee 3
                                i32.const 2147483646
                                i32.le_u
                                br_if 0 (;@14;)
                                local.get 4
                                local.set 8
                                br 8 (;@6;)
                              end
                              block ;; label = @14
                                local.get 3
                                call $sbrk
                                i32.const -1
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 3
                                local.get 6
                                i32.add
                                local.set 6
                                local.get 4
                                local.set 8
                                br 8 (;@6;)
                              end
                              i32.const 0
                              local.get 6
                              i32.sub
                              call $sbrk
                              drop
                              br 5 (;@8;)
                            end
                            local.get 4
                            local.set 8
                            local.get 4
                            i32.const -1
                            i32.ne
                            br_if 6 (;@6;)
                            br 4 (;@8;)
                          end
                          unreachable
                        end
                        i32.const 0
                        local.set 4
                        br 8 (;@2;)
                      end
                      i32.const 0
                      local.set 4
                      br 6 (;@3;)
                    end
                    local.get 8
                    i32.const -1
                    i32.ne
                    br_if 2 (;@6;)
                  end
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056564
                  i32.const 4
                  i32.or
                  i32.store offset=1056564
                end
                local.get 9
                i32.const 2147483646
                i32.gt_u
                br_if 1 (;@5;)
                local.get 9
                call $sbrk
                local.set 8
                i32.const 0
                call $sbrk
                local.set 4
                local.get 8
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                local.get 4
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                local.get 8
                local.get 4
                i32.ge_u
                br_if 1 (;@5;)
                local.get 4
                local.get 8
                i32.sub
                local.tee 6
                local.get 5
                i32.const 56
                i32.add
                i32.le_u
                br_if 1 (;@5;)
              end
              i32.const 0
              i32.const 0
              i32.load offset=1056552
              local.get 6
              i32.add
              local.tee 4
              i32.store offset=1056552
              block ;; label = @6
                local.get 4
                i32.const 0
                i32.load offset=1056556
                i32.le_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 4
                i32.store offset=1056556
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load offset=1056144
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                      i32.const 1056568
                      local.set 4
                      loop ;; label = @10
                        local.get 8
                        local.get 4
                        i32.load
                        local.tee 0
                        local.get 4
                        i32.load offset=4
                        local.tee 9
                        i32.add
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 4
                        i32.load offset=8
                        local.tee 4
                        br_if 0 (;@10;)
                        br 3 (;@7;)
                      end
                    end
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1056136
                        local.tee 4
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 8
                        local.get 4
                        i32.ge_u
                        br_if 1 (;@9;)
                      end
                      i32.const 0
                      local.get 8
                      i32.store offset=1056136
                    end
                    i32.const 0
                    local.set 4
                    i32.const 0
                    local.get 6
                    i32.store offset=1056572
                    i32.const 0
                    local.get 8
                    i32.store offset=1056568
                    i32.const 0
                    i32.const -1
                    i32.store offset=1056152
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056592
                    i32.store offset=1056156
                    i32.const 0
                    i32.const 0
                    i32.store offset=1056580
                    loop ;; label = @9
                      local.get 4
                      i32.const 1056180
                      i32.add
                      local.get 4
                      i32.const 1056168
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 4
                      i32.const 1056160
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 4
                      i32.const 1056172
                      i32.add
                      local.get 0
                      i32.store
                      local.get 4
                      i32.const 1056188
                      i32.add
                      local.get 4
                      i32.const 1056176
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 0
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 1056196
                      i32.add
                      local.get 4
                      i32.const 1056184
                      i32.add
                      local.tee 3
                      i32.store
                      local.get 3
                      local.get 0
                      i32.store
                      local.get 4
                      i32.const 1056192
                      i32.add
                      local.get 3
                      i32.store
                      local.get 4
                      i32.const 32
                      i32.add
                      local.tee 4
                      i32.const 256
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 8
                    i32.const -8
                    local.get 8
                    i32.sub
                    i32.const 15
                    i32.and
                    local.tee 4
                    i32.add
                    local.tee 3
                    local.get 6
                    i32.const -56
                    i32.add
                    local.tee 0
                    local.get 4
                    i32.sub
                    local.tee 4
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056608
                    i32.store offset=1056148
                    i32.const 0
                    local.get 4
                    i32.store offset=1056132
                    i32.const 0
                    local.get 3
                    i32.store offset=1056144
                    local.get 8
                    local.get 0
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br 2 (;@6;)
                  end
                  local.get 3
                  local.get 8
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 0
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 4
                  i32.load offset=12
                  i32.const 8
                  i32.and
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const -8
                  local.get 3
                  i32.sub
                  i32.const 15
                  i32.and
                  local.tee 0
                  i32.add
                  local.tee 8
                  i32.const 0
                  i32.load offset=1056132
                  local.get 6
                  i32.add
                  local.tee 11
                  local.get 0
                  i32.sub
                  local.tee 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 4
                  local.get 9
                  local.get 6
                  i32.add
                  i32.store offset=4
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056608
                  i32.store offset=1056148
                  i32.const 0
                  local.get 0
                  i32.store offset=1056132
                  i32.const 0
                  local.get 8
                  i32.store offset=1056144
                  local.get 3
                  local.get 11
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 1 (;@6;)
                end
                block ;; label = @7
                  local.get 8
                  i32.const 0
                  i32.load offset=1056136
                  i32.ge_u
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 8
                  i32.store offset=1056136
                end
                local.get 8
                local.get 6
                i32.add
                local.set 0
                i32.const 1056568
                local.set 4
                block ;; label = @7
                  block ;; label = @8
                    loop ;; label = @9
                      local.get 4
                      i32.load
                      local.tee 9
                      local.get 0
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 4
                      i32.load offset=8
                      local.tee 4
                      br_if 0 (;@9;)
                      br 2 (;@7;)
                    end
                  end
                  local.get 4
                  i32.load8_u offset=12
                  i32.const 8
                  i32.and
                  i32.eqz
                  br_if 3 (;@4;)
                end
                i32.const 1056568
                local.set 4
                block ;; label = @7
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 3
                      local.get 4
                      i32.load
                      local.tee 0
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 3
                      local.get 0
                      local.get 4
                      i32.load offset=4
                      i32.add
                      local.tee 0
                      i32.lt_u
                      br_if 2 (;@7;)
                    end
                    local.get 4
                    i32.load offset=8
                    local.set 4
                    br 0 (;@8;)
                  end
                end
                local.get 8
                i32.const -8
                local.get 8
                i32.sub
                i32.const 15
                i32.and
                local.tee 4
                i32.add
                local.tee 11
                local.get 6
                i32.const -56
                i32.add
                local.tee 9
                local.get 4
                i32.sub
                local.tee 4
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 8
                local.get 9
                i32.add
                i32.const 56
                i32.store offset=4
                local.get 3
                local.get 0
                i32.const 55
                local.get 0
                i32.sub
                i32.const 15
                i32.and
                i32.add
                i32.const -63
                i32.add
                local.tee 9
                local.get 9
                local.get 3
                i32.const 16
                i32.add
                i32.lt_u
                select
                local.tee 9
                i32.const 35
                i32.store offset=4
                i32.const 0
                i32.const 0
                i32.load offset=1056608
                i32.store offset=1056148
                i32.const 0
                local.get 4
                i32.store offset=1056132
                i32.const 0
                local.get 11
                i32.store offset=1056144
                local.get 9
                i32.const 16
                i32.add
                i32.const 0
                i64.load offset=1056576 align=4
                i64.store align=4
                local.get 9
                i32.const 0
                i64.load offset=1056568 align=4
                i64.store offset=8 align=4
                i32.const 0
                local.get 9
                i32.const 8
                i32.add
                i32.store offset=1056576
                i32.const 0
                local.get 6
                i32.store offset=1056572
                i32.const 0
                local.get 8
                i32.store offset=1056568
                i32.const 0
                i32.const 0
                i32.store offset=1056580
                local.get 9
                i32.const 36
                i32.add
                local.set 4
                loop ;; label = @7
                  local.get 4
                  i32.const 7
                  i32.store
                  local.get 4
                  i32.const 4
                  i32.add
                  local.tee 4
                  local.get 0
                  i32.lt_u
                  br_if 0 (;@7;)
                end
                local.get 9
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                local.get 9
                local.get 9
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 9
                local.get 9
                local.get 3
                i32.sub
                local.tee 8
                i32.store
                local.get 3
                local.get 8
                i32.const 1
                i32.or
                i32.store offset=4
                block ;; label = @7
                  block ;; label = @8
                    local.get 8
                    i32.const 255
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const -8
                    i32.and
                    i32.const 1056160
                    i32.add
                    local.set 4
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1056120
                        local.tee 0
                        i32.const 1
                        local.get 8
                        i32.const 3
                        i32.shr_u
                        i32.shl
                        local.tee 8
                        i32.and
                        br_if 0 (;@10;)
                        i32.const 0
                        local.get 0
                        local.get 8
                        i32.or
                        i32.store offset=1056120
                        local.get 4
                        local.set 0
                        br 1 (;@9;)
                      end
                      local.get 4
                      i32.load offset=8
                      local.set 0
                    end
                    local.get 0
                    local.get 3
                    i32.store offset=12
                    local.get 4
                    local.get 3
                    i32.store offset=8
                    i32.const 12
                    local.set 8
                    i32.const 8
                    local.set 9
                    br 1 (;@7;)
                  end
                  i32.const 31
                  local.set 4
                  block ;; label = @8
                    local.get 8
                    i32.const 16777215
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 38
                    local.get 8
                    i32.const 8
                    i32.shr_u
                    i32.clz
                    local.tee 4
                    i32.sub
                    i32.shr_u
                    i32.const 1
                    i32.and
                    local.get 4
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 62
                    i32.add
                    local.set 4
                  end
                  local.get 3
                  local.get 4
                  i32.store offset=28
                  local.get 3
                  i64.const 0
                  i64.store offset=16 align=4
                  local.get 4
                  i32.const 2
                  i32.shl
                  i32.const 1056424
                  i32.add
                  local.set 0
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load offset=1056124
                        local.tee 9
                        i32.const 1
                        local.get 4
                        i32.shl
                        local.tee 6
                        i32.and
                        br_if 0 (;@10;)
                        local.get 0
                        local.get 3
                        i32.store
                        i32.const 0
                        local.get 9
                        local.get 6
                        i32.or
                        i32.store offset=1056124
                        local.get 3
                        local.get 0
                        i32.store offset=24
                        br 1 (;@9;)
                      end
                      local.get 8
                      i32.const 0
                      i32.const 25
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      local.get 4
                      i32.const 31
                      i32.eq
                      select
                      i32.shl
                      local.set 4
                      local.get 0
                      i32.load
                      local.set 9
                      loop ;; label = @10
                        local.get 9
                        local.tee 0
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 8
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 4
                        i32.const 29
                        i32.shr_u
                        local.set 9
                        local.get 4
                        i32.const 1
                        i32.shl
                        local.set 4
                        local.get 0
                        local.get 9
                        i32.const 4
                        i32.and
                        i32.add
                        local.tee 6
                        i32.load offset=16
                        local.tee 9
                        br_if 0 (;@10;)
                      end
                      local.get 6
                      i32.const 16
                      i32.add
                      local.get 3
                      i32.store
                      local.get 3
                      local.get 0
                      i32.store offset=24
                    end
                    i32.const 8
                    local.set 8
                    i32.const 12
                    local.set 9
                    local.get 3
                    local.set 0
                    local.get 3
                    local.set 4
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.load offset=8
                  local.set 4
                  local.get 0
                  local.get 3
                  i32.store offset=8
                  local.get 4
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 4
                  i32.store offset=8
                  i32.const 0
                  local.set 4
                  i32.const 24
                  local.set 8
                  i32.const 12
                  local.set 9
                end
                local.get 3
                local.get 9
                i32.add
                local.get 0
                i32.store
                local.get 3
                local.get 8
                i32.add
                local.get 4
                i32.store
              end
              i32.const 0
              i32.load offset=1056132
              local.tee 4
              local.get 5
              i32.le_u
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1056144
              local.tee 3
              local.get 5
              i32.add
              local.tee 0
              local.get 4
              local.get 5
              i32.sub
              local.tee 4
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 0
              local.get 4
              i32.store offset=1056132
              i32.const 0
              local.get 0
              i32.store offset=1056144
              local.get 3
              local.get 5
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 3
              i32.const 8
              i32.add
              local.set 4
              br 4 (;@1;)
            end
            i32.const 0
            local.set 4
            i32.const 0
            i32.const 48
            i32.store offset=1055972
            br 3 (;@1;)
          end
          local.get 4
          local.get 8
          i32.store
          local.get 4
          local.get 4
          i32.load offset=4
          local.get 6
          i32.add
          i32.store offset=4
          local.get 8
          local.get 9
          local.get 5
          call $prepend_alloc
          local.set 4
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 11
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 9
              local.get 9
              i32.load offset=28
              local.tee 8
              i32.const 2
              i32.shl
              local.tee 0
              i32.load offset=1056424
              i32.ne
              br_if 0 (;@5;)
              local.get 0
              i32.const 1056424
              i32.add
              local.get 4
              i32.store
              local.get 4
              br_if 1 (;@4;)
              i32.const 0
              local.get 10
              i32.const -2
              local.get 8
              i32.rotl
              i32.and
              local.tee 10
              i32.store offset=1056124
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 11
                i32.load offset=16
                local.get 9
                i32.ne
                br_if 0 (;@6;)
                local.get 11
                local.get 4
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 11
              local.get 4
              i32.store offset=20
            end
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 4
          local.get 11
          i32.store offset=24
          block ;; label = @4
            local.get 9
            i32.load offset=16
            local.tee 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 0
            i32.store offset=16
            local.get 0
            local.get 4
            i32.store offset=24
          end
          local.get 9
          i32.load offset=20
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 0
          i32.store offset=20
          local.get 0
          local.get 4
          i32.store offset=24
        end
        block ;; label = @3
          block ;; label = @4
            local.get 3
            i32.const 15
            i32.gt_u
            br_if 0 (;@4;)
            local.get 9
            local.get 3
            local.get 5
            i32.or
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 9
            local.get 4
            i32.add
            local.tee 4
            local.get 4
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          end
          local.get 9
          local.get 5
          i32.add
          local.tee 8
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 9
          local.get 5
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 3
          i32.add
          local.get 3
          i32.store
          block ;; label = @4
            local.get 3
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.and
            i32.const 1056160
            i32.add
            local.set 4
            block ;; label = @5
              block ;; label = @6
                i32.const 0
                i32.load offset=1056120
                local.tee 5
                i32.const 1
                local.get 3
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 3
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.get 5
                local.get 3
                i32.or
                i32.store offset=1056120
                local.get 4
                local.set 3
                br 1 (;@5;)
              end
              local.get 4
              i32.load offset=8
              local.set 3
            end
            local.get 3
            local.get 8
            i32.store offset=12
            local.get 4
            local.get 8
            i32.store offset=8
            local.get 8
            local.get 4
            i32.store offset=12
            local.get 8
            local.get 3
            i32.store offset=8
            br 1 (;@3;)
          end
          i32.const 31
          local.set 4
          block ;; label = @4
            local.get 3
            i32.const 16777215
            i32.gt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 38
            local.get 3
            i32.const 8
            i32.shr_u
            i32.clz
            local.tee 4
            i32.sub
            i32.shr_u
            i32.const 1
            i32.and
            local.get 4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set 4
          end
          local.get 8
          local.get 4
          i32.store offset=28
          local.get 8
          i64.const 0
          i64.store offset=16 align=4
          local.get 4
          i32.const 2
          i32.shl
          i32.const 1056424
          i32.add
          local.set 5
          block ;; label = @4
            local.get 10
            i32.const 1
            local.get 4
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            local.get 5
            local.get 8
            i32.store
            i32.const 0
            local.get 10
            local.get 0
            i32.or
            i32.store offset=1056124
            local.get 8
            local.get 5
            i32.store offset=24
            local.get 8
            local.get 8
            i32.store offset=8
            local.get 8
            local.get 8
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 3
          i32.const 0
          i32.const 25
          local.get 4
          i32.const 1
          i32.shr_u
          i32.sub
          local.get 4
          i32.const 31
          i32.eq
          select
          i32.shl
          local.set 4
          local.get 5
          i32.load
          local.set 0
          block ;; label = @4
            loop ;; label = @5
              local.get 0
              local.tee 5
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 4
              i32.const 29
              i32.shr_u
              local.set 0
              local.get 4
              i32.const 1
              i32.shl
              local.set 4
              local.get 5
              local.get 0
              i32.const 4
              i32.and
              i32.add
              local.tee 6
              i32.load offset=16
              local.tee 0
              br_if 0 (;@5;)
            end
            local.get 6
            i32.const 16
            i32.add
            local.get 8
            i32.store
            local.get 8
            local.get 5
            i32.store offset=24
            local.get 8
            local.get 8
            i32.store offset=12
            local.get 8
            local.get 8
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 5
          i32.load offset=8
          local.tee 4
          local.get 8
          i32.store offset=12
          local.get 5
          local.get 8
          i32.store offset=8
          local.get 8
          i32.const 0
          i32.store offset=24
          local.get 8
          local.get 5
          i32.store offset=12
          local.get 8
          local.get 4
          i32.store offset=8
        end
        local.get 9
        i32.const 8
        i32.add
        local.set 4
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 8
            local.get 8
            i32.load offset=28
            local.tee 9
            i32.const 2
            i32.shl
            local.tee 0
            i32.load offset=1056424
            i32.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 1056424
            i32.add
            local.get 4
            i32.store
            local.get 4
            br_if 1 (;@3;)
            i32.const 0
            local.get 10
            i32.const -2
            local.get 9
            i32.rotl
            i32.and
            i32.store offset=1056124
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 2
              i32.load offset=16
              local.get 8
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              local.get 4
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 2
            local.get 4
            i32.store offset=20
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 4
        local.get 2
        i32.store offset=24
        block ;; label = @3
          local.get 8
          i32.load offset=16
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.get 0
          i32.store offset=16
          local.get 0
          local.get 4
          i32.store offset=24
        end
        local.get 8
        i32.load offset=20
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.store offset=20
        local.get 0
        local.get 4
        i32.store offset=24
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 8
          local.get 3
          local.get 5
          i32.or
          local.tee 4
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 8
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 8
        local.get 5
        i32.add
        local.tee 0
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 8
        local.get 5
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 0
        local.get 3
        i32.add
        local.get 3
        i32.store
        block ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          i32.const -8
          i32.and
          i32.const 1056160
          i32.add
          local.set 5
          i32.const 0
          i32.load offset=1056140
          local.set 4
          block ;; label = @4
            block ;; label = @5
              i32.const 1
              local.get 7
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 9
              local.get 6
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.get 9
              local.get 6
              i32.or
              i32.store offset=1056120
              local.get 5
              local.set 9
              br 1 (;@4;)
            end
            local.get 5
            i32.load offset=8
            local.set 9
          end
          local.get 9
          local.get 4
          i32.store offset=12
          local.get 5
          local.get 4
          i32.store offset=8
          local.get 4
          local.get 5
          i32.store offset=12
          local.get 4
          local.get 9
          i32.store offset=8
        end
        i32.const 0
        local.get 0
        i32.store offset=1056140
        i32.const 0
        local.get 3
        i32.store offset=1056128
      end
      local.get 8
      i32.const 8
      i32.add
      local.set 4
    end
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 4
  )
  (func $prepend_alloc (;144;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const -8
    local.get 0
    i32.sub
    i32.const 15
    i32.and
    i32.add
    local.tee 3
    local.get 2
    i32.const 3
    i32.or
    i32.store offset=4
    local.get 1
    i32.const -8
    local.get 1
    i32.sub
    i32.const 15
    i32.and
    i32.add
    local.tee 4
    local.get 3
    local.get 2
    i32.add
    local.tee 5
    i32.sub
    local.set 0
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.const 0
        i32.load offset=1056144
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 5
        i32.store offset=1056144
        i32.const 0
        i32.const 0
        i32.load offset=1056132
        local.get 0
        i32.add
        local.tee 2
        i32.store offset=1056132
        local.get 5
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 4
        i32.const 0
        i32.load offset=1056140
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 5
        i32.store offset=1056140
        i32.const 0
        i32.const 0
        i32.load offset=1056128
        local.get 0
        i32.add
        local.tee 2
        i32.store offset=1056128
        local.get 5
        local.get 2
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 5
        local.get 2
        i32.add
        local.get 2
        i32.store
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 4
        i32.load offset=4
        local.tee 1
        i32.const 3
        i32.and
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        local.set 6
        local.get 4
        i32.load offset=12
        local.set 2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 2
              local.get 4
              i32.load offset=8
              local.tee 7
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1056120
              i32.const -2
              local.get 1
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=1056120
              br 2 (;@3;)
            end
            local.get 2
            local.get 7
            i32.store offset=8
            local.get 7
            local.get 2
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 4
          i32.load offset=24
          local.set 8
          block ;; label = @4
            block ;; label = @5
              local.get 2
              local.get 4
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=8
              local.tee 1
              local.get 2
              i32.store offset=12
              local.get 2
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            end
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  i32.load offset=20
                  local.tee 1
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  i32.const 20
                  i32.add
                  local.set 7
                  br 1 (;@6;)
                end
                local.get 4
                i32.load offset=16
                local.tee 1
                i32.eqz
                br_if 1 (;@5;)
                local.get 4
                i32.const 16
                i32.add
                local.set 7
              end
              loop ;; label = @6
                local.get 7
                local.set 9
                local.get 1
                local.tee 2
                i32.const 20
                i32.add
                local.set 7
                local.get 2
                i32.load offset=20
                local.tee 1
                br_if 0 (;@6;)
                local.get 2
                i32.const 16
                i32.add
                local.set 7
                local.get 2
                i32.load offset=16
                local.tee 1
                br_if 0 (;@6;)
              end
              local.get 9
              i32.const 0
              i32.store
              br 1 (;@4;)
            end
            i32.const 0
            local.set 2
          end
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 4
              local.get 4
              i32.load offset=28
              local.tee 7
              i32.const 2
              i32.shl
              local.tee 1
              i32.load offset=1056424
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 1056424
              i32.add
              local.get 2
              i32.store
              local.get 2
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1056124
              i32.const -2
              local.get 7
              i32.rotl
              i32.and
              i32.store offset=1056124
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.load offset=16
                local.get 4
                i32.ne
                br_if 0 (;@6;)
                local.get 8
                local.get 2
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 8
              local.get 2
              i32.store offset=20
            end
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 2
          local.get 8
          i32.store offset=24
          block ;; label = @4
            local.get 4
            i32.load offset=16
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 2
            i32.store offset=24
          end
          local.get 4
          i32.load offset=20
          local.tee 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.store offset=20
          local.get 1
          local.get 2
          i32.store offset=24
        end
        local.get 6
        local.get 0
        i32.add
        local.set 0
        local.get 4
        local.get 6
        i32.add
        local.tee 4
        i32.load offset=4
        local.set 1
      end
      local.get 4
      local.get 1
      i32.const -2
      i32.and
      i32.store offset=4
      local.get 5
      local.get 0
      i32.add
      local.get 0
      i32.store
      local.get 5
      local.get 0
      i32.const 1
      i32.or
      i32.store offset=4
      block ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 1056160
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1056120
            local.tee 1
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 1
            local.get 0
            i32.or
            i32.store offset=1056120
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 5
        i32.store offset=12
        local.get 2
        local.get 5
        i32.store offset=8
        local.get 5
        local.get 2
        i32.store offset=12
        local.get 5
        local.get 0
        i32.store offset=8
        br 1 (;@1;)
      end
      i32.const 31
      local.set 2
      block ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 2
      end
      local.get 5
      local.get 2
      i32.store offset=28
      local.get 5
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1056424
      i32.add
      local.set 1
      block ;; label = @2
        i32.const 0
        i32.load offset=1056124
        local.tee 7
        i32.const 1
        local.get 2
        i32.shl
        local.tee 4
        i32.and
        br_if 0 (;@2;)
        local.get 1
        local.get 5
        i32.store
        i32.const 0
        local.get 7
        local.get 4
        i32.or
        i32.store offset=1056124
        local.get 5
        local.get 1
        i32.store offset=24
        local.get 5
        local.get 5
        i32.store offset=8
        local.get 5
        local.get 5
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.const 25
      local.get 2
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 2
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 2
      local.get 1
      i32.load
      local.set 7
      block ;; label = @2
        loop ;; label = @3
          local.get 7
          local.tee 1
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 0
          i32.eq
          br_if 1 (;@2;)
          local.get 2
          i32.const 29
          i32.shr_u
          local.set 7
          local.get 2
          i32.const 1
          i32.shl
          local.set 2
          local.get 1
          local.get 7
          i32.const 4
          i32.and
          i32.add
          local.tee 4
          i32.load offset=16
          local.tee 7
          br_if 0 (;@3;)
        end
        local.get 4
        i32.const 16
        i32.add
        local.get 5
        i32.store
        local.get 5
        local.get 1
        i32.store offset=24
        local.get 5
        local.get 5
        i32.store offset=12
        local.get 5
        local.get 5
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=8
      local.tee 2
      local.get 5
      i32.store offset=12
      local.get 1
      local.get 5
      i32.store offset=8
      local.get 5
      i32.const 0
      i32.store offset=24
      local.get 5
      local.get 1
      i32.store offset=12
      local.get 5
      local.get 2
      i32.store offset=8
    end
    local.get 3
    i32.const 8
    i32.add
  )
  (func $free (;145;) (type 1) (param i32)
    local.get 0
    call $dlfree
  )
  (func $dlfree (;146;) (type 1) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -8
      i32.add
      local.tee 1
      local.get 0
      i32.const -4
      i32.add
      i32.load
      local.tee 2
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 3
      block ;; label = @2
        local.get 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        local.get 1
        i32.load
        local.tee 4
        i32.sub
        local.tee 1
        i32.const 0
        i32.load offset=1056136
        i32.lt_u
        br_if 1 (;@1;)
        local.get 4
        local.get 0
        i32.add
        local.set 0
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.const 0
                i32.load offset=1056140
                i32.eq
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=12
                local.set 2
                block ;; label = @7
                  local.get 4
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 1
                  i32.load offset=8
                  local.tee 5
                  i32.ne
                  br_if 2 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056120
                  i32.const -2
                  local.get 4
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store offset=1056120
                  br 5 (;@2;)
                end
                local.get 1
                i32.load offset=24
                local.set 6
                block ;; label = @7
                  local.get 2
                  local.get 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 1
                  i32.load offset=8
                  local.tee 4
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 4
                  i32.store offset=8
                  br 4 (;@3;)
                end
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load offset=20
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 20
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 1
                  i32.const 16
                  i32.add
                  local.set 5
                end
                loop ;; label = @7
                  local.get 5
                  local.set 7
                  local.get 4
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 3 (;@3;)
              end
              local.get 3
              i32.load offset=4
              local.tee 2
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              local.get 3
              local.get 2
              i32.const -2
              i32.and
              i32.store offset=4
              i32.const 0
              local.get 0
              i32.store offset=1056128
              local.get 3
              local.get 0
              i32.store
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              return
            end
            local.get 2
            local.get 5
            i32.store offset=8
            local.get 5
            local.get 2
            i32.store offset=12
            br 2 (;@2;)
          end
          i32.const 0
          local.set 2
        end
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 1
            local.get 1
            i32.load offset=28
            local.tee 5
            i32.const 2
            i32.shl
            local.tee 4
            i32.load offset=1056424
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            i32.const 1056424
            i32.add
            local.get 2
            i32.store
            local.get 2
            br_if 1 (;@3;)
            i32.const 0
            i32.const 0
            i32.load offset=1056124
            i32.const -2
            local.get 5
            i32.rotl
            i32.and
            i32.store offset=1056124
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 6
              i32.load offset=16
              local.get 1
              i32.ne
              br_if 0 (;@5;)
              local.get 6
              local.get 2
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 6
            local.get 2
            i32.store offset=20
          end
          local.get 2
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 2
        local.get 6
        i32.store offset=24
        block ;; label = @3
          local.get 1
          i32.load offset=16
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          i32.store offset=16
          local.get 4
          local.get 2
          i32.store offset=24
        end
        local.get 1
        i32.load offset=20
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 4
        i32.store offset=20
        local.get 4
        local.get 2
        i32.store offset=24
      end
      local.get 1
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=4
      local.tee 4
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.const 2
                i32.and
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 3
                  i32.const 0
                  i32.load offset=1056144
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 1
                  i32.store offset=1056144
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056132
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store offset=1056132
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  i32.const 0
                  i32.load offset=1056140
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=1056128
                  i32.const 0
                  i32.const 0
                  i32.store offset=1056140
                  return
                end
                block ;; label = @7
                  local.get 3
                  i32.const 0
                  i32.load offset=1056140
                  local.tee 6
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 1
                  i32.store offset=1056140
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056128
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store offset=1056128
                  local.get 1
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 0
                  i32.add
                  local.get 0
                  i32.store
                  return
                end
                local.get 4
                i32.const -8
                i32.and
                local.get 0
                i32.add
                local.set 0
                local.get 3
                i32.load offset=12
                local.set 2
                block ;; label = @7
                  local.get 4
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    local.get 3
                    i32.load offset=8
                    local.tee 5
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056120
                    i32.const -2
                    local.get 4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1056120
                    br 5 (;@3;)
                  end
                  local.get 2
                  local.get 5
                  i32.store offset=8
                  local.get 5
                  local.get 2
                  i32.store offset=12
                  br 4 (;@3;)
                end
                local.get 3
                i32.load offset=24
                local.set 8
                block ;; label = @7
                  local.get 2
                  local.get 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 4
                  i32.store offset=8
                  br 3 (;@4;)
                end
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    i32.load offset=20
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 20
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 3
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 5
                end
                loop ;; label = @7
                  local.get 5
                  local.set 7
                  local.get 4
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 2
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 2 (;@4;)
              end
              local.get 3
              local.get 4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 1
              local.get 0
              i32.add
              local.get 0
              i32.store
              local.get 1
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              br 3 (;@2;)
            end
            i32.const 0
            local.set 2
          end
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 3
              local.get 3
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              local.tee 4
              i32.load offset=1056424
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.const 1056424
              i32.add
              local.get 2
              i32.store
              local.get 2
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1056124
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1056124
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.load offset=16
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 8
                local.get 2
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 8
              local.get 2
              i32.store offset=20
            end
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 2
          local.get 8
          i32.store offset=24
          block ;; label = @4
            local.get 3
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 2
            i32.store offset=24
          end
          local.get 3
          i32.load offset=20
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          i32.store offset=20
          local.get 4
          local.get 2
          i32.store offset=24
        end
        local.get 1
        local.get 0
        i32.add
        local.get 0
        i32.store
        local.get 1
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 6
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 0
        i32.store offset=1056128
        return
      end
      block ;; label = @2
        local.get 0
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -8
        i32.and
        i32.const 1056160
        i32.add
        local.set 2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1056120
            local.tee 4
            i32.const 1
            local.get 0
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 0
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 0
            i32.or
            i32.store offset=1056120
            local.get 2
            local.set 0
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.set 0
        end
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 2
        local.get 1
        i32.store offset=8
        local.get 1
        local.get 2
        i32.store offset=12
        local.get 1
        local.get 0
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 2
      block ;; label = @2
        local.get 0
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 2
      end
      local.get 1
      local.get 2
      i32.store offset=28
      local.get 1
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1056424
      i32.add
      local.set 5
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load offset=1056124
              local.tee 4
              i32.const 1
              local.get 2
              i32.shl
              local.tee 3
              i32.and
              br_if 0 (;@5;)
              local.get 5
              local.get 1
              i32.store
              i32.const 0
              local.get 4
              local.get 3
              i32.or
              i32.store offset=1056124
              i32.const 8
              local.set 0
              i32.const 24
              local.set 2
              br 1 (;@4;)
            end
            local.get 0
            i32.const 0
            i32.const 25
            local.get 2
            i32.const 1
            i32.shr_u
            i32.sub
            local.get 2
            i32.const 31
            i32.eq
            select
            i32.shl
            local.set 2
            local.get 5
            i32.load
            local.set 5
            loop ;; label = @5
              local.get 5
              local.tee 4
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 0
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 29
              i32.shr_u
              local.set 5
              local.get 2
              i32.const 1
              i32.shl
              local.set 2
              local.get 4
              local.get 5
              i32.const 4
              i32.and
              i32.add
              local.tee 3
              i32.load offset=16
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 3
            i32.const 16
            i32.add
            local.get 1
            i32.store
            i32.const 8
            local.set 0
            i32.const 24
            local.set 2
            local.get 4
            local.set 5
          end
          local.get 1
          local.set 4
          local.get 1
          local.set 3
          br 1 (;@2;)
        end
        local.get 4
        i32.load offset=8
        local.tee 5
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 1
        i32.store offset=8
        i32.const 0
        local.set 3
        i32.const 24
        local.set 0
        i32.const 8
        local.set 2
      end
      local.get 1
      local.get 2
      i32.add
      local.get 5
      i32.store
      local.get 1
      local.get 4
      i32.store offset=12
      local.get 1
      local.get 0
      i32.add
      local.get 3
      i32.store
      i32.const 0
      i32.const 0
      i32.load offset=1056152
      i32.const -1
      i32.add
      local.tee 1
      i32.const -1
      local.get 1
      select
      i32.store offset=1056152
    end
  )
  (func $calloc (;147;) (type 5) (param i32 i32) (result i32)
    (local i32 i64)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      local.get 0
      i64.extend_i32_u
      local.get 1
      i64.extend_i32_u
      i64.mul
      local.tee 3
      i32.wrap_i64
      local.set 2
      local.get 1
      local.get 0
      i32.or
      i32.const 65536
      i32.lt_u
      br_if 0 (;@1;)
      i32.const -1
      local.get 2
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      i32.const 0
      i32.ne
      select
      local.set 2
    end
    block ;; label = @1
      local.get 2
      call $dlmalloc
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const -4
      i32.add
      i32.load8_u
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      local.get 2
      memory.fill
    end
    local.get 0
  )
  (func $realloc (;148;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      local.get 1
      call $dlmalloc
      return
    end
    block ;; label = @1
      local.get 1
      i32.const -64
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1055972
      i32.const 0
      return
    end
    i32.const 16
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    local.get 1
    i32.const 11
    i32.lt_u
    select
    local.set 2
    local.get 0
    i32.const -4
    i32.add
    local.tee 3
    i32.load
    local.tee 4
    i32.const -8
    i32.and
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 4
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.const 256
          i32.lt_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          i32.le_u
          br_if 1 (;@2;)
          local.get 5
          local.get 2
          i32.sub
          i32.const 0
          i32.load offset=1056600
          i32.const 1
          i32.shl
          i32.le_u
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        local.get 0
        i32.const -8
        i32.add
        local.tee 6
        local.get 5
        i32.add
        local.set 7
        block ;; label = @3
          local.get 5
          local.get 2
          i32.lt_u
          br_if 0 (;@3;)
          local.get 5
          local.get 2
          i32.sub
          local.tee 1
          i32.const 16
          i32.lt_u
          br_if 2 (;@1;)
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 2
          i32.add
          local.tee 2
          local.get 1
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 7
          local.get 7
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 2
          local.get 1
          call $dispose_chunk
          local.get 0
          return
        end
        block ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=1056144
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1056132
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.le_u
          br_if 1 (;@2;)
          local.get 3
          local.get 2
          local.get 4
          i32.const 1
          i32.and
          i32.or
          i32.const 2
          i32.or
          i32.store
          i32.const 0
          local.get 6
          local.get 2
          i32.add
          local.tee 1
          i32.store offset=1056144
          i32.const 0
          local.get 5
          local.get 2
          i32.sub
          local.tee 2
          i32.store offset=1056132
          local.get 1
          local.get 2
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        block ;; label = @3
          local.get 7
          i32.const 0
          i32.load offset=1056140
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          i32.load offset=1056128
          local.get 5
          i32.add
          local.tee 5
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          block ;; label = @4
            block ;; label = @5
              local.get 5
              local.get 2
              i32.sub
              local.tee 1
              i32.const 16
              i32.lt_u
              br_if 0 (;@5;)
              local.get 3
              local.get 2
              local.get 4
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 6
              local.get 2
              i32.add
              local.tee 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 6
              local.get 5
              i32.add
              local.tee 5
              local.get 1
              i32.store
              local.get 5
              local.get 5
              i32.load offset=4
              i32.const -2
              i32.and
              i32.store offset=4
              br 1 (;@4;)
            end
            local.get 3
            local.get 4
            i32.const 1
            i32.and
            local.get 5
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 6
            local.get 5
            i32.add
            local.tee 1
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            i32.const 0
            local.set 1
            i32.const 0
            local.set 2
          end
          i32.const 0
          local.get 2
          i32.store offset=1056140
          i32.const 0
          local.get 1
          i32.store offset=1056128
          local.get 0
          return
        end
        local.get 7
        i32.load offset=4
        local.tee 8
        i32.const 2
        i32.and
        br_if 0 (;@2;)
        local.get 8
        i32.const -8
        i32.and
        local.get 5
        i32.add
        local.tee 9
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 9
        local.get 2
        i32.sub
        local.set 10
        local.get 7
        i32.load offset=12
        local.set 1
        block ;; label = @3
          block ;; label = @4
            local.get 8
            i32.const 255
            i32.gt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 1
              local.get 7
              i32.load offset=8
              local.tee 5
              i32.ne
              br_if 0 (;@5;)
              i32.const 0
              i32.const 0
              i32.load offset=1056120
              i32.const -2
              local.get 8
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=1056120
              br 2 (;@3;)
            end
            local.get 1
            local.get 5
            i32.store offset=8
            local.get 5
            local.get 1
            i32.store offset=12
            br 1 (;@3;)
          end
          local.get 7
          i32.load offset=24
          local.set 11
          block ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 7
              i32.eq
              br_if 0 (;@5;)
              local.get 7
              i32.load offset=8
              local.tee 5
              local.get 1
              i32.store offset=12
              local.get 1
              local.get 5
              i32.store offset=8
              br 1 (;@4;)
            end
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 7
                  i32.load offset=20
                  local.tee 5
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 7
                  i32.const 20
                  i32.add
                  local.set 8
                  br 1 (;@6;)
                end
                local.get 7
                i32.load offset=16
                local.tee 5
                i32.eqz
                br_if 1 (;@5;)
                local.get 7
                i32.const 16
                i32.add
                local.set 8
              end
              loop ;; label = @6
                local.get 8
                local.set 12
                local.get 5
                local.tee 1
                i32.const 20
                i32.add
                local.set 8
                local.get 1
                i32.load offset=20
                local.tee 5
                br_if 0 (;@6;)
                local.get 1
                i32.const 16
                i32.add
                local.set 8
                local.get 1
                i32.load offset=16
                local.tee 5
                br_if 0 (;@6;)
              end
              local.get 12
              i32.const 0
              i32.store
              br 1 (;@4;)
            end
            i32.const 0
            local.set 1
          end
          local.get 11
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 7
              local.get 7
              i32.load offset=28
              local.tee 8
              i32.const 2
              i32.shl
              local.tee 5
              i32.load offset=1056424
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              i32.const 1056424
              i32.add
              local.get 1
              i32.store
              local.get 1
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1056124
              i32.const -2
              local.get 8
              i32.rotl
              i32.and
              i32.store offset=1056124
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 11
                i32.load offset=16
                local.get 7
                i32.ne
                br_if 0 (;@6;)
                local.get 11
                local.get 1
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 11
              local.get 1
              i32.store offset=20
            end
            local.get 1
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 1
          local.get 11
          i32.store offset=24
          block ;; label = @4
            local.get 7
            i32.load offset=16
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 5
            i32.store offset=16
            local.get 5
            local.get 1
            i32.store offset=24
          end
          local.get 7
          i32.load offset=20
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 5
          i32.store offset=20
          local.get 5
          local.get 1
          i32.store offset=24
        end
        block ;; label = @3
          local.get 10
          i32.const 15
          i32.gt_u
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.const 1
          i32.and
          local.get 9
          i32.or
          i32.const 2
          i32.or
          i32.store
          local.get 6
          local.get 9
          i32.add
          local.tee 1
          local.get 1
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 0
          return
        end
        local.get 3
        local.get 2
        local.get 4
        i32.const 1
        i32.and
        i32.or
        i32.const 2
        i32.or
        i32.store
        local.get 6
        local.get 2
        i32.add
        local.tee 1
        local.get 10
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 6
        local.get 9
        i32.add
        local.tee 2
        local.get 2
        i32.load offset=4
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 1
        local.get 10
        call $dispose_chunk
        local.get 0
        return
      end
      block ;; label = @2
        local.get 1
        call $dlmalloc
        local.tee 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      block ;; label = @2
        i32.const -4
        i32.const -8
        local.get 3
        i32.load
        local.tee 5
        i32.const 3
        i32.and
        select
        local.get 5
        i32.const -8
        i32.and
        i32.add
        local.tee 5
        local.get 1
        local.get 5
        local.get 1
        i32.lt_u
        select
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        local.get 1
        memory.copy
      end
      local.get 0
      call $dlfree
      local.get 2
      local.set 0
    end
    local.get 0
  )
  (func $dispose_chunk (;149;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 3
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 4
        local.get 1
        i32.add
        local.set 1
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                local.get 4
                i32.sub
                local.tee 0
                i32.const 0
                i32.load offset=1056140
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.load offset=12
                local.set 3
                block ;; label = @7
                  local.get 4
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 0
                  i32.load offset=8
                  local.tee 5
                  i32.ne
                  br_if 2 (;@5;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056120
                  i32.const -2
                  local.get 4
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store offset=1056120
                  br 5 (;@2;)
                end
                local.get 0
                i32.load offset=24
                local.set 6
                block ;; label = @7
                  local.get 3
                  local.get 0
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=8
                  local.tee 4
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 4
                  i32.store offset=8
                  br 4 (;@3;)
                end
                block ;; label = @7
                  block ;; label = @8
                    local.get 0
                    i32.load offset=20
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 20
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.const 16
                  i32.add
                  local.set 5
                end
                loop ;; label = @7
                  local.get 5
                  local.set 7
                  local.get 4
                  local.tee 3
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 3
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 3
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 3 (;@3;)
              end
              local.get 2
              i32.load offset=4
              local.tee 3
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              local.get 2
              local.get 3
              i32.const -2
              i32.and
              i32.store offset=4
              i32.const 0
              local.get 1
              i32.store offset=1056128
              local.get 2
              local.get 1
              i32.store
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              return
            end
            local.get 3
            local.get 5
            i32.store offset=8
            local.get 5
            local.get 3
            i32.store offset=12
            br 2 (;@2;)
          end
          i32.const 0
          local.set 3
        end
        local.get 6
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 0
            i32.load offset=28
            local.tee 5
            i32.const 2
            i32.shl
            local.tee 4
            i32.load offset=1056424
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            i32.const 1056424
            i32.add
            local.get 3
            i32.store
            local.get 3
            br_if 1 (;@3;)
            i32.const 0
            i32.const 0
            i32.load offset=1056124
            i32.const -2
            local.get 5
            i32.rotl
            i32.and
            i32.store offset=1056124
            br 2 (;@2;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 6
              i32.load offset=16
              local.get 0
              i32.ne
              br_if 0 (;@5;)
              local.get 6
              local.get 3
              i32.store offset=16
              br 1 (;@4;)
            end
            local.get 6
            local.get 3
            i32.store offset=20
          end
          local.get 3
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 3
        local.get 6
        i32.store offset=24
        block ;; label = @3
          local.get 0
          i32.load offset=16
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.store offset=16
          local.get 4
          local.get 3
          i32.store offset=24
        end
        local.get 0
        i32.load offset=20
        local.tee 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 4
        i32.store offset=20
        local.get 4
        local.get 3
        i32.store offset=24
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i32.load offset=4
                local.tee 4
                i32.const 2
                i32.and
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 2
                  i32.const 0
                  i32.load offset=1056144
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1056144
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056132
                  local.get 1
                  i32.add
                  local.tee 1
                  i32.store offset=1056132
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  i32.const 0
                  i32.load offset=1056140
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 0
                  i32.const 0
                  i32.store offset=1056128
                  i32.const 0
                  i32.const 0
                  i32.store offset=1056140
                  return
                end
                block ;; label = @7
                  local.get 2
                  i32.const 0
                  i32.load offset=1056140
                  local.tee 6
                  i32.ne
                  br_if 0 (;@7;)
                  i32.const 0
                  local.get 0
                  i32.store offset=1056140
                  i32.const 0
                  i32.const 0
                  i32.load offset=1056128
                  local.get 1
                  i32.add
                  local.tee 1
                  i32.store offset=1056128
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 1
                  i32.add
                  local.get 1
                  i32.store
                  return
                end
                local.get 4
                i32.const -8
                i32.and
                local.get 1
                i32.add
                local.set 1
                local.get 2
                i32.load offset=12
                local.set 3
                block ;; label = @7
                  local.get 4
                  i32.const 255
                  i32.gt_u
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 3
                    local.get 2
                    i32.load offset=8
                    local.tee 5
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.const 0
                    i32.load offset=1056120
                    i32.const -2
                    local.get 4
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store offset=1056120
                    br 5 (;@3;)
                  end
                  local.get 3
                  local.get 5
                  i32.store offset=8
                  local.get 5
                  local.get 3
                  i32.store offset=12
                  br 4 (;@3;)
                end
                local.get 2
                i32.load offset=24
                local.set 8
                block ;; label = @7
                  local.get 3
                  local.get 2
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load offset=8
                  local.tee 4
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 4
                  i32.store offset=8
                  br 3 (;@4;)
                end
                block ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.load offset=20
                    local.tee 4
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 2
                    i32.const 20
                    i32.add
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 2
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 5
                end
                loop ;; label = @7
                  local.get 5
                  local.set 7
                  local.get 4
                  local.tee 3
                  i32.const 20
                  i32.add
                  local.set 5
                  local.get 3
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 5
                  local.get 3
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 2 (;@4;)
              end
              local.get 2
              local.get 4
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 0
              local.get 1
              i32.add
              local.get 1
              i32.store
              local.get 0
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              br 3 (;@2;)
            end
            i32.const 0
            local.set 3
          end
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 2
              local.get 2
              i32.load offset=28
              local.tee 5
              i32.const 2
              i32.shl
              local.tee 4
              i32.load offset=1056424
              i32.ne
              br_if 0 (;@5;)
              local.get 4
              i32.const 1056424
              i32.add
              local.get 3
              i32.store
              local.get 3
              br_if 1 (;@4;)
              i32.const 0
              i32.const 0
              i32.load offset=1056124
              i32.const -2
              local.get 5
              i32.rotl
              i32.and
              i32.store offset=1056124
              br 2 (;@3;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.load offset=16
                local.get 2
                i32.ne
                br_if 0 (;@6;)
                local.get 8
                local.get 3
                i32.store offset=16
                br 1 (;@5;)
              end
              local.get 8
              local.get 3
              i32.store offset=20
            end
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 3
          local.get 8
          i32.store offset=24
          block ;; label = @4
            local.get 2
            i32.load offset=16
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 4
            i32.store offset=16
            local.get 4
            local.get 3
            i32.store offset=24
          end
          local.get 2
          i32.load offset=20
          local.tee 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.store offset=20
          local.get 4
          local.get 3
          i32.store offset=24
        end
        local.get 0
        local.get 1
        i32.add
        local.get 1
        i32.store
        local.get 0
        local.get 1
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 0
        local.get 6
        i32.ne
        br_if 0 (;@2;)
        i32.const 0
        local.get 1
        i32.store offset=1056128
        return
      end
      block ;; label = @2
        local.get 1
        i32.const 255
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -8
        i32.and
        i32.const 1056160
        i32.add
        local.set 3
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1056120
            local.tee 4
            i32.const 1
            local.get 1
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 1
            i32.and
            br_if 0 (;@4;)
            i32.const 0
            local.get 4
            local.get 1
            i32.or
            i32.store offset=1056120
            local.get 3
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          i32.load offset=8
          local.set 1
        end
        local.get 1
        local.get 0
        i32.store offset=12
        local.get 3
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 3
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        return
      end
      i32.const 31
      local.set 3
      block ;; label = @2
        local.get 1
        i32.const 16777215
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 38
        local.get 1
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 3
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 3
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 3
      end
      local.get 0
      local.get 3
      i32.store offset=28
      local.get 0
      i64.const 0
      i64.store offset=16 align=4
      local.get 3
      i32.const 2
      i32.shl
      i32.const 1056424
      i32.add
      local.set 4
      block ;; label = @2
        i32.const 0
        i32.load offset=1056124
        local.tee 5
        i32.const 1
        local.get 3
        i32.shl
        local.tee 2
        i32.and
        br_if 0 (;@2;)
        local.get 4
        local.get 0
        i32.store
        i32.const 0
        local.get 5
        local.get 2
        i32.or
        i32.store offset=1056124
        local.get 0
        local.get 4
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 0
        i32.store offset=12
        return
      end
      local.get 1
      i32.const 0
      i32.const 25
      local.get 3
      i32.const 1
      i32.shr_u
      i32.sub
      local.get 3
      i32.const 31
      i32.eq
      select
      i32.shl
      local.set 3
      local.get 4
      i32.load
      local.set 5
      block ;; label = @2
        loop ;; label = @3
          local.get 5
          local.tee 4
          i32.load offset=4
          i32.const -8
          i32.and
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          i32.const 29
          i32.shr_u
          local.set 5
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 4
          local.get 5
          i32.const 4
          i32.and
          i32.add
          local.tee 2
          i32.load offset=16
          local.tee 5
          br_if 0 (;@3;)
        end
        local.get 2
        i32.const 16
        i32.add
        local.get 0
        i32.store
        local.get 0
        local.get 4
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=12
        local.get 0
        local.get 0
        i32.store offset=8
        return
      end
      local.get 4
      i32.load offset=8
      local.tee 1
      local.get 0
      i32.store offset=12
      local.get 4
      local.get 0
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=24
      local.get 0
      local.get 4
      i32.store offset=12
      local.get 0
      local.get 1
      i32.store offset=8
    end
  )
  (func $posix_memalign (;150;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 16
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          call $dlmalloc
          local.set 1
          br 1 (;@2;)
        end
        i32.const 28
        local.set 3
        local.get 1
        i32.const 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        i32.const 3
        i32.and
        br_if 1 (;@1;)
        local.get 1
        i32.const 2
        i32.shr_u
        local.tee 4
        local.get 4
        i32.const -1
        i32.add
        i32.and
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 2
          i32.const -64
          local.get 1
          i32.sub
          i32.le_u
          br_if 0 (;@3;)
          i32.const 48
          return
        end
        local.get 1
        i32.const 16
        local.get 1
        i32.const 16
        i32.gt_u
        select
        local.get 2
        call $internal_memalign
        local.set 1
      end
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        i32.const 48
        return
      end
      local.get 0
      local.get 1
      i32.store
      i32.const 0
      local.set 3
    end
    local.get 3
  )
  (func $internal_memalign (;151;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const 16
        local.get 0
        i32.const 16
        i32.gt_u
        select
        local.tee 2
        local.get 2
        i32.const -1
        i32.add
        i32.and
        br_if 0 (;@2;)
        local.get 2
        local.set 0
        br 1 (;@1;)
      end
      i32.const 32
      local.set 3
      loop ;; label = @2
        local.get 3
        local.tee 0
        i32.const 1
        i32.shl
        local.set 3
        local.get 0
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      local.get 1
      i32.const -64
      local.get 0
      i32.sub
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 0
      i32.const 48
      i32.store offset=1055972
      i32.const 0
      return
    end
    block ;; label = @1
      local.get 0
      i32.const 16
      local.get 1
      i32.const 19
      i32.add
      i32.const -16
      i32.and
      local.get 1
      i32.const 11
      i32.lt_u
      select
      local.tee 1
      i32.add
      i32.const 12
      i32.add
      call $dlmalloc
      local.tee 3
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 3
    i32.const -8
    i32.add
    local.set 2
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.const -1
        i32.add
        local.get 3
        i32.and
        br_if 0 (;@2;)
        local.get 2
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      i32.const -4
      i32.add
      local.tee 4
      i32.load
      local.tee 5
      i32.const -8
      i32.and
      local.get 3
      local.get 0
      i32.add
      i32.const -1
      i32.add
      i32.const 0
      local.get 0
      i32.sub
      i32.and
      i32.const -8
      i32.add
      local.tee 3
      i32.const 0
      local.get 0
      local.get 3
      local.get 2
      i32.sub
      i32.const 15
      i32.gt_u
      select
      i32.add
      local.tee 0
      local.get 2
      i32.sub
      local.tee 3
      i32.sub
      local.set 6
      block ;; label = @2
        local.get 5
        i32.const 3
        i32.and
        br_if 0 (;@2;)
        local.get 0
        local.get 6
        i32.store offset=4
        local.get 0
        local.get 2
        i32.load
        local.get 3
        i32.add
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 6
      local.get 0
      i32.load offset=4
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store offset=4
      local.get 0
      local.get 6
      i32.add
      local.tee 6
      local.get 6
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 4
      local.get 3
      local.get 4
      i32.load
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store
      local.get 2
      local.get 3
      i32.add
      local.tee 6
      local.get 6
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 2
      local.get 3
      call $dispose_chunk
    end
    block ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 3
      i32.const 3
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const -8
      i32.and
      local.tee 2
      local.get 1
      i32.const 16
      i32.add
      i32.le_u
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      local.get 3
      i32.const 1
      i32.and
      i32.or
      i32.const 2
      i32.or
      i32.store offset=4
      local.get 0
      local.get 1
      i32.add
      local.tee 3
      local.get 2
      local.get 1
      i32.sub
      local.tee 1
      i32.const 3
      i32.or
      i32.store offset=4
      local.get 0
      local.get 2
      i32.add
      local.tee 2
      local.get 2
      i32.load offset=4
      i32.const 1
      i32.or
      i32.store offset=4
      local.get 3
      local.get 1
      call $dispose_chunk
    end
    local.get 0
    i32.const 8
    i32.add
  )
  (func $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error (;152;) (type 2) (param i32 i32)
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      call $_RNvNtCsi9YzqDQQz2q_5alloc5alloc18handle_alloc_error
      unreachable
    end
    call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec17capacity_overflow
    unreachable
  )
  (func $_RNvMs4_NtCsi9YzqDQQz2q_5alloc7raw_vecNtB5_11RawVecInner11finish_growB7_ (;153;) (type 3) (param i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.const 0
        i32.ge_s
        br_if 0 (;@2;)
        i32.const 1
        local.set 1
        i32.const 4
        local.set 2
        i32.const 0
        local.set 3
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.const 1
              local.get 3
              call $_RNvCsfLfy6EI15iL_7___rustc14___rust_realloc
              local.set 1
              br 1 (;@4;)
            end
            block ;; label = @5
              local.get 3
              br_if 0 (;@5;)
              i32.const 1
              local.set 1
              br 2 (;@3;)
            end
            call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            local.get 3
            i32.const 1
            call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
            local.set 1
          end
          local.get 1
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 0
          i32.const 1
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        i32.const 0
        local.set 1
      end
      i32.const 8
      local.set 2
    end
    local.get 0
    local.get 2
    i32.add
    local.get 3
    i32.store
    local.get 0
    local.get 1
    i32.store
  )
  (func $_RNvNtCsi9YzqDQQz2q_5alloc5alloc18handle_alloc_error (;154;) (type 2) (param i32 i32)
    local.get 1
    local.get 0
    call $_RNvCsfLfy6EI15iL_7___rustc26___rust_alloc_error_handler
    unreachable
  )
  (func $_RNvMs_NtNtCsi9YzqDQQz2q_5alloc3ffi5c_strNtB4_7CString19__from_vec_unchecked (;155;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load
          local.tee 3
          local.get 1
          i32.load offset=8
          local.tee 4
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.const 4
          i32.add
          local.get 4
          local.get 1
          i32.load offset=4
          local.get 4
          i32.const 1
          i32.add
          local.tee 3
          call $_RNvMs4_NtCsi9YzqDQQz2q_5alloc7raw_vecNtB5_11RawVecInner11finish_growB7_
          local.get 2
          i32.load offset=4
          i32.const 1
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          local.get 2
          i32.load offset=8
          i32.store offset=4
        end
        local.get 1
        i32.load offset=4
        local.tee 5
        local.get 4
        i32.add
        i32.const 0
        i32.store8
        block ;; label = @3
          block ;; label = @4
            local.get 3
            local.get 4
            i32.const 1
            i32.add
            local.tee 1
            i32.gt_u
            br_if 0 (;@4;)
            local.get 5
            local.set 4
            br 1 (;@3;)
          end
          block ;; label = @4
            local.get 1
            br_if 0 (;@4;)
            i32.const 1
            local.set 4
            local.get 5
            local.get 3
            i32.const 1
            call $_RNvCsfLfy6EI15iL_7___rustc14___rust_dealloc
            br 1 (;@3;)
          end
          local.get 5
          local.get 3
          i32.const 1
          local.get 1
          call $_RNvCsfLfy6EI15iL_7___rustc14___rust_realloc
          local.tee 4
          i32.eqz
          br_if 2 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        local.get 0
        local.get 4
        i32.store
        local.get 2
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
      local.get 2
      i32.load offset=8
      local.get 2
      i32.load offset=12
      call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
      unreachable
    end
    i32.const 1
    local.get 1
    call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
    unreachable
  )
  (func $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec17capacity_overflow (;156;) (type 0)
    i32.const 1055114
    i32.const 35
    i32.const 1055132
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvXs_NvMs_NtNtCsi9YzqDQQz2q_5alloc3ffi5c_strNtB9_7CString3newRShNtB4_11SpecNewImpl13spec_new_impl (;157;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    i32.const 0
    local.set 4
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.const 1
          i32.add
          local.tee 5
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          call $_RNvCsfLfy6EI15iL_7___rustc35___rust_no_alloc_shim_is_unstable_v2
          i32.const 1
          local.set 4
          local.get 5
          i32.const 1
          call $_RNvCsfLfy6EI15iL_7___rustc12___rust_alloc
          local.tee 6
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            i32.const 0
            local.set 7
            i32.const 0
            local.set 4
            br 3 (;@1;)
          end
          block ;; label = @4
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            local.get 1
            local.get 2
            memory.copy
          end
          block ;; label = @4
            local.get 2
            i32.const 8
            i32.lt_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 8
            i32.add
            i32.const 0
            local.get 1
            local.get 2
            call $_RNvNtNtCsdHhIpgkcIfN_4core5slice6memchr14memchr_aligned
            local.get 3
            i32.load offset=12
            local.set 7
            local.get 3
            i32.load offset=8
            local.set 4
            br 3 (;@1;)
          end
          block ;; label = @4
            local.get 1
            i32.load8_u
            br_if 0 (;@4;)
            i32.const 1
            local.set 4
            i32.const 0
            local.set 7
            br 3 (;@1;)
          end
          i32.const 1
          local.set 4
          local.get 2
          i32.const 1
          i32.eq
          br_if 1 (;@2;)
          block ;; label = @4
            local.get 1
            i32.load8_u offset=1
            br_if 0 (;@4;)
            i32.const 1
            local.set 7
            br 3 (;@1;)
          end
          i32.const 2
          local.set 7
          local.get 2
          i32.const 2
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          i32.load8_u offset=2
          i32.eqz
          br_if 2 (;@1;)
          i32.const 3
          local.set 7
          local.get 2
          i32.const 3
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          i32.load8_u offset=3
          i32.eqz
          br_if 2 (;@1;)
          i32.const 4
          local.set 7
          local.get 2
          i32.const 4
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          i32.load8_u offset=4
          i32.eqz
          br_if 2 (;@1;)
          i32.const 5
          local.set 7
          local.get 2
          i32.const 5
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          i32.load8_u offset=5
          i32.eqz
          br_if 2 (;@1;)
          local.get 2
          local.set 7
          i32.const 0
          local.set 4
          local.get 2
          i32.const 6
          i32.eq
          br_if 2 (;@1;)
          local.get 2
          i32.const 6
          local.get 1
          i32.load8_u offset=6
          local.tee 4
          select
          local.set 7
          local.get 4
          i32.eqz
          local.set 4
          br 2 (;@1;)
        end
        local.get 4
        local.get 5
        call $_RNvNtCsi9YzqDQQz2q_5alloc7raw_vec12handle_error
        unreachable
      end
      local.get 2
      local.set 7
      i32.const 0
      local.set 4
    end
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.const 1
        i32.ne
        br_if 0 (;@2;)
        local.get 0
        local.get 2
        i32.store offset=8
        local.get 0
        local.get 6
        i32.store offset=4
        local.get 0
        local.get 5
        i32.store
        local.get 0
        local.get 7
        i32.store offset=12
        br 1 (;@1;)
      end
      local.get 3
      local.get 2
      i32.store offset=28
      local.get 3
      local.get 6
      i32.store offset=24
      local.get 3
      local.get 5
      i32.store offset=20
      local.get 3
      local.get 3
      i32.const 20
      i32.add
      call $_RNvMs_NtNtCsi9YzqDQQz2q_5alloc3ffi5c_strNtB4_7CString19__from_vec_unchecked
      local.get 0
      local.get 3
      i64.load
      i64.store offset=4 align=4
      local.get 0
      i32.const -2147483648
      i32.store
    end
    local.get 3
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core9panicking19assert_failed_inner (;158;) (type 13) (param i32 i32 i32 i32 i32 i32 i32 i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 8
    global.set $__stack_pointer
    local.get 8
    local.get 2
    i32.store offset=4
    local.get 8
    local.get 1
    i32.store
    local.get 8
    local.get 4
    i32.store offset=12
    local.get 8
    local.get 3
    i32.store offset=8
    local.get 8
    local.get 0
    i32.const 255
    i32.and
    i32.const 2
    i32.shl
    local.tee 2
    i32.load offset=1055820
    i32.store offset=20
    local.get 8
    local.get 2
    i32.load offset=1055808
    i32.store offset=16
    block ;; label = @1
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 8
      local.get 6
      i32.store offset=28
      local.get 8
      local.get 5
      i32.store offset=24
      local.get 8
      i32.const 62
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 9
      local.get 8
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=56
      local.get 8
      local.get 9
      local.get 8
      i64.extend_i32_u
      i64.or
      i64.store offset=48
      local.get 8
      i32.const 63
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 8
      i32.const 24
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=40
      local.get 8
      i32.const 64
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 8
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=32
      i32.const 1048902
      local.get 8
      i32.const 32
      i32.add
      local.get 7
      call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
      unreachable
    end
    local.get 8
    i32.const 62
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.tee 9
    local.get 8
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=48
    local.get 8
    local.get 9
    local.get 8
    i64.extend_i32_u
    i64.or
    i64.store offset=40
    local.get 8
    i32.const 64
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 8
    i32.const 16
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=32
    i32.const 1048847
    local.get 8
    i32.const 32
    i32.add
    local.get 7
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core9panicking5panic (;159;) (type 9) (param i32 i32 i32)
    local.get 0
    local.get 1
    i32.const 1
    i32.shl
    i32.const 1
    i32.or
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail (;160;) (type 3) (param i32 i32 i32 i32)
    (local i32 i64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 0
          local.get 2
          i32.gt_u
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.gt_u
          br_if 1 (;@2;)
          i32.const 9
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.set 5
          local.get 0
          local.get 1
          i32.le_u
          br_if 2 (;@1;)
          local.get 4
          local.get 0
          i32.store offset=8
          local.get 4
          local.get 1
          i32.store offset=12
          local.get 4
          local.get 5
          local.get 4
          i32.const 12
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=24
          local.get 4
          local.get 5
          local.get 4
          i32.const 8
          i32.add
          i64.extend_i32_u
          i64.or
          i64.store offset=16
          i32.const 1048695
          local.get 4
          i32.const 16
          i32.add
          local.get 3
          call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
          unreachable
        end
        local.get 4
        local.get 0
        i32.store offset=8
        local.get 4
        local.get 2
        i32.store offset=12
        local.get 4
        i32.const 9
        i64.extend_i32_u
        i64.const 32
        i64.shl
        local.tee 5
        local.get 4
        i32.const 12
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=24
        local.get 4
        local.get 5
        local.get 4
        i32.const 8
        i32.add
        i64.extend_i32_u
        i64.or
        i64.store offset=16
        i32.const 1048735
        local.get 4
        i32.const 16
        i32.add
        local.get 3
        call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
        unreachable
      end
      local.get 4
      local.get 1
      i32.store offset=8
      local.get 4
      local.get 2
      i32.store offset=12
      local.get 4
      i32.const 9
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.tee 5
      local.get 4
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=24
      local.get 4
      local.get 5
      local.get 4
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.or
      i64.store offset=16
      i32.const 1048792
      local.get 4
      i32.const 16
      i32.add
      local.get 3
      call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
      unreachable
    end
    local.get 4
    local.get 1
    i32.store offset=8
    local.get 4
    local.get 2
    i32.store offset=12
    local.get 4
    local.get 5
    local.get 4
    i32.const 12
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    local.get 4
    local.get 5
    local.get 4
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    i32.const 1048792
    local.get 4
    i32.const 16
    i32.add
    local.get 3
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt (;161;) (type 9) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=16
    local.get 3
    local.get 0
    i32.store offset=12
    local.get 3
    i32.const 1
    i32.store16 offset=28
    local.get 3
    local.get 2
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 12
    i32.add
    i32.store offset=20
    local.get 3
    i32.const 20
    i32.add
    call $_RNvCsfLfy6EI15iL_7___rustc17rust_begin_unwind
    unreachable
  )
  (func $_RNvXsd_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3impyNtB9_7Display3fmt (;162;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i64 i64 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 20
    local.set 3
    local.get 0
    i64.load
    local.tee 4
    local.set 5
    block ;; label = @1
      local.get 4
      i64.const 1000
      i64.lt_u
      br_if 0 (;@1;)
      i32.const 20
      local.set 3
      local.get 4
      local.set 5
      loop ;; label = @2
        local.get 2
        i32.const 12
        i32.add
        local.get 3
        i32.add
        local.tee 0
        i32.const -4
        i32.add
        local.get 5
        local.tee 6
        local.get 6
        i64.const 10000
        i64.div_u
        local.tee 5
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.load16_u offset=1055262 align=1
        i32.store16 align=1
        local.get 0
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1055262 align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 6
        i64.const 9999999
        i64.gt_u
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      local.get 5
      i64.const 9
      i64.le_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 5
      i32.wrap_i64
      local.tee 0
      local.get 0
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 0
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1055262 align=1
      i32.store16 align=1
      local.get 0
      i64.extend_i32_u
      local.set 5
    end
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i64.eqz
        br_if 0 (;@2;)
        local.get 5
        i64.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 12
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 5
      i32.wrap_i64
      i32.const 1
      i32.shl
      i32.load8_u offset=1055263
      i32.store8
    end
    local.get 1
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 12
    i32.add
    local.get 3
    i32.add
    i32.const 20
    local.get 3
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_RNvXs1i_NtCsdHhIpgkcIfN_4core3fmtReNtB6_7Display3fmtB8_ (;163;) (type 5) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter3pad
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core3fmt5write (;164;) (type 7) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 2
          i32.load8_u
          local.tee 5
          br_if 1 (;@2;)
          i32.const 0
          local.set 5
          br 2 (;@1;)
        end
        local.get 0
        local.get 2
        local.get 3
        i32.const 1
        i32.shr_u
        local.get 1
        i32.load offset=12
        call_indirect (type 4)
        local.set 5
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=12
      local.set 6
      i32.const 0
      local.set 7
      loop ;; label = @2
        local.get 2
        i32.const 1
        i32.add
        local.set 8
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 5
                  i32.extend8_s
                  i32.const -1
                  i32.gt_s
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 255
                  i32.and
                  local.tee 9
                  i32.const 128
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 9
                  i32.const 192
                  i32.ne
                  br_if 3 (;@4;)
                  local.get 4
                  local.get 1
                  i32.store offset=4
                  local.get 4
                  local.get 0
                  i32.store
                  local.get 4
                  i64.const 1610612768
                  i64.store offset=8 align=4
                  local.get 3
                  local.get 7
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 5
                  i32.load
                  local.get 4
                  local.get 5
                  i32.load offset=4
                  call_indirect (type 5)
                  i32.eqz
                  br_if 2 (;@5;)
                  i32.const 1
                  local.set 5
                  br 6 (;@1;)
                end
                block ;; label = @7
                  local.get 0
                  local.get 8
                  local.get 5
                  i32.const 255
                  i32.and
                  local.tee 5
                  local.get 6
                  call_indirect (type 4)
                  br_if 0 (;@7;)
                  local.get 8
                  local.get 5
                  i32.add
                  local.set 2
                  br 4 (;@3;)
                end
                i32.const 1
                local.set 5
                br 5 (;@1;)
              end
              block ;; label = @6
                local.get 0
                local.get 2
                i32.const 3
                i32.add
                local.tee 5
                local.get 2
                i32.load16_u offset=1 align=1
                local.tee 2
                local.get 6
                call_indirect (type 4)
                br_if 0 (;@6;)
                local.get 5
                local.get 2
                i32.add
                local.set 2
                br 3 (;@3;)
              end
              i32.const 1
              local.set 5
              br 4 (;@1;)
            end
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 8
            local.set 2
            br 1 (;@3;)
          end
          i32.const 1610612768
          local.set 10
          block ;; label = @4
            local.get 5
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 5
            i32.add
            local.set 8
            local.get 2
            i32.load offset=1 align=1
            local.set 10
          end
          i32.const 0
          local.set 9
          block ;; label = @4
            block ;; label = @5
              local.get 5
              i32.const 2
              i32.and
              br_if 0 (;@5;)
              i32.const 0
              local.set 11
              local.get 8
              local.set 2
              br 1 (;@4;)
            end
            local.get 8
            i32.const 2
            i32.add
            local.set 2
            local.get 8
            i32.load16_u align=1
            local.set 11
          end
          block ;; label = @4
            block ;; label = @5
              local.get 5
              i32.const 4
              i32.and
              br_if 0 (;@5;)
              local.get 2
              local.set 8
              br 1 (;@4;)
            end
            local.get 2
            i32.const 2
            i32.add
            local.set 8
            local.get 2
            i32.load16_u align=1
            local.set 9
          end
          block ;; label = @4
            block ;; label = @5
              local.get 5
              i32.const 8
              i32.and
              br_if 0 (;@5;)
              local.get 8
              local.set 2
              br 1 (;@4;)
            end
            local.get 8
            i32.const 2
            i32.add
            local.set 2
            local.get 8
            i32.load16_u align=1
            local.set 7
          end
          block ;; label = @4
            local.get 5
            i32.const 16
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 11
            i32.const 65535
            i32.and
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
            local.set 11
          end
          block ;; label = @4
            local.get 5
            i32.const 32
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            local.get 9
            i32.const 65535
            i32.and
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
            local.set 9
          end
          local.get 4
          local.get 9
          i32.store16 offset=14
          local.get 4
          local.get 11
          i32.store16 offset=12
          local.get 4
          local.get 10
          i32.store offset=8
          local.get 4
          local.get 1
          i32.store offset=4
          local.get 4
          local.get 0
          i32.store
          block ;; label = @4
            local.get 3
            local.get 7
            i32.const 3
            i32.shl
            i32.add
            local.tee 5
            i32.load
            local.get 4
            local.get 5
            i32.load offset=4
            call_indirect (type 5)
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1
            local.set 5
            br 3 (;@1;)
          end
          local.get 7
          i32.const 1
          i32.add
          local.set 7
        end
        local.get 2
        i32.load8_u
        local.tee 5
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 5
    end
    local.get 4
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 5
  )
  (func $_RNvMNtCsdHhIpgkcIfN_4core3stre9from_utf8 (;165;) (type 9) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call $_RNvNtNtCsdHhIpgkcIfN_4core3str8converts9from_utf8
  )
  (func $_RNvNtNtCsdHhIpgkcIfN_4core3str8converts9from_utf8 (;166;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i64 i32)
    block ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.get 2
      i32.const -7
      i32.add
      local.tee 3
      local.get 3
      local.get 2
      i32.gt_u
      select
      local.set 4
      local.get 1
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.get 1
      i32.sub
      local.set 5
      i32.const 0
      local.set 3
      loop ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                local.get 3
                i32.add
                i32.load8_u
                local.tee 6
                i32.extend8_s
                local.tee 7
                i32.const 0
                i32.lt_s
                br_if 0 (;@6;)
                local.get 5
                local.get 3
                i32.sub
                i32.const 3
                i32.and
                br_if 1 (;@5;)
                local.get 3
                local.get 4
                i32.ge_u
                br_if 2 (;@4;)
                loop ;; label = @7
                  local.get 1
                  local.get 3
                  i32.add
                  local.tee 6
                  i32.const 4
                  i32.add
                  i32.load
                  local.get 6
                  i32.load
                  i32.or
                  i32.const -2139062144
                  i32.and
                  br_if 3 (;@4;)
                  local.get 3
                  i32.const 8
                  i32.add
                  local.tee 3
                  local.get 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  br 3 (;@4;)
                end
              end
              i64.const 1103806595072
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                local.get 6
                                i32.load8_u offset=1055462
                                i32.const -2
                                i32.add
                                br_table 0 (;@14;) 1 (;@13;) 2 (;@12;) 7 (;@7;)
                              end
                              local.get 3
                              i32.const 1
                              i32.add
                              local.tee 6
                              local.get 2
                              i32.lt_u
                              br_if 2 (;@11;)
                              i64.const 0
                              local.set 8
                              br 6 (;@7;)
                            end
                            local.get 3
                            i32.const 1
                            i32.add
                            local.tee 9
                            local.get 2
                            i32.lt_u
                            br_if 2 (;@10;)
                            i64.const 0
                            local.set 8
                            br 5 (;@7;)
                          end
                          local.get 3
                          i32.const 1
                          i32.add
                          local.tee 9
                          local.get 2
                          i32.lt_u
                          br_if 2 (;@9;)
                          i64.const 0
                          local.set 8
                          br 4 (;@7;)
                        end
                        local.get 1
                        local.get 6
                        i32.add
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        br_if 3 (;@7;)
                        br 4 (;@6;)
                      end
                      local.get 1
                      local.get 9
                      i32.add
                      i32.load8_s
                      local.set 9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 6
                            i32.const -224
                            i32.add
                            br_table 0 (;@12;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 2 (;@10;) 1 (;@11;) 2 (;@10;)
                          end
                          local.get 9
                          i32.const -32
                          i32.and
                          i32.const -96
                          i32.eq
                          br_if 3 (;@8;)
                          br 4 (;@7;)
                        end
                        local.get 9
                        i32.const -97
                        i32.gt_s
                        br_if 3 (;@7;)
                        br 2 (;@8;)
                      end
                      block ;; label = @10
                        local.get 7
                        i32.const 31
                        i32.add
                        i32.const 255
                        i32.and
                        i32.const 12
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 7
                        i32.const -2
                        i32.and
                        i32.const -18
                        i32.ne
                        br_if 3 (;@7;)
                        local.get 9
                        i32.const -64
                        i32.lt_s
                        br_if 2 (;@8;)
                        br 3 (;@7;)
                      end
                      local.get 9
                      i32.const -64
                      i32.lt_s
                      br_if 1 (;@8;)
                      br 2 (;@7;)
                    end
                    local.get 1
                    local.get 9
                    i32.add
                    i32.load8_s
                    local.set 9
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 6
                            i32.const -240
                            i32.add
                            br_table 1 (;@11;) 0 (;@12;) 0 (;@12;) 0 (;@12;) 2 (;@10;) 0 (;@12;)
                          end
                          local.get 7
                          i32.const 15
                          i32.add
                          i32.const 255
                          i32.and
                          i32.const 2
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 9
                          i32.const -64
                          i32.lt_s
                          br_if 2 (;@9;)
                          br 4 (;@7;)
                        end
                        local.get 9
                        i32.const 112
                        i32.add
                        i32.const 255
                        i32.and
                        i32.const 48
                        i32.lt_u
                        br_if 1 (;@9;)
                        br 3 (;@7;)
                      end
                      local.get 9
                      i32.const -113
                      i32.gt_s
                      br_if 2 (;@7;)
                    end
                    block ;; label = @9
                      local.get 3
                      i32.const 2
                      i32.add
                      local.tee 6
                      local.get 2
                      i32.lt_u
                      br_if 0 (;@9;)
                      i64.const 0
                      local.set 8
                      br 2 (;@7;)
                    end
                    block ;; label = @9
                      local.get 1
                      local.get 6
                      i32.add
                      i32.load8_s
                      i32.const -65
                      i32.le_s
                      br_if 0 (;@9;)
                      i64.const 2203318222848
                      local.set 8
                      br 2 (;@7;)
                    end
                    i64.const 0
                    local.set 8
                    local.get 3
                    i32.const 3
                    i32.add
                    local.tee 6
                    local.get 2
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 1
                    local.get 6
                    i32.add
                    i32.load8_s
                    i32.const -64
                    i32.lt_s
                    br_if 2 (;@6;)
                    i64.const 3302829850624
                    local.set 8
                    br 1 (;@7;)
                  end
                  i64.const 0
                  local.set 8
                  local.get 3
                  i32.const 2
                  i32.add
                  local.tee 6
                  local.get 2
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 6
                  i32.add
                  i32.load8_s
                  i32.const -65
                  i32.le_s
                  br_if 1 (;@6;)
                  i64.const 2203318222848
                  local.set 8
                end
                local.get 0
                local.get 8
                local.get 3
                i64.extend_i32_u
                i64.or
                i64.store offset=4 align=4
                local.get 0
                i32.const 1
                i32.store
                return
              end
              local.get 6
              i32.const 1
              i32.add
              local.set 3
              br 2 (;@3;)
            end
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            br 1 (;@3;)
          end
          local.get 3
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          loop ;; label = @4
            local.get 1
            local.get 3
            i32.add
            i32.load8_s
            i32.const 0
            i32.lt_s
            br_if 1 (;@3;)
            local.get 2
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            i32.ne
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 3
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_RNvXs0_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str (;167;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=4
    local.set 3
    local.get 0
    i32.load
    local.set 4
    local.get 0
    i32.load offset=8
    local.set 5
    i32.const 0
    local.set 6
    i32.const 0
    local.set 7
    i32.const 0
    local.set 8
    i32.const 0
    local.set 9
    block ;; label = @1
      loop ;; label = @2
        local.get 9
        i32.const 1
        i32.and
        br_if 1 (;@1;)
        block ;; label = @3
          block ;; label = @4
            local.get 2
            local.get 8
            i32.lt_u
            br_if 0 (;@4;)
            loop ;; label = @5
              local.get 1
              local.get 8
              i32.add
              local.set 9
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 2
                          local.get 8
                          i32.sub
                          local.tee 10
                          i32.const 7
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 8
                          i32.ne
                          br_if 1 (;@10;)
                          local.get 2
                          local.set 8
                          br 7 (;@4;)
                        end
                        local.get 9
                        i32.const 3
                        i32.add
                        i32.const -4
                        i32.and
                        local.tee 0
                        local.get 9
                        i32.eq
                        br_if 1 (;@9;)
                        local.get 0
                        local.get 9
                        i32.sub
                        local.set 0
                        i32.const 0
                        local.set 11
                        loop ;; label = @11
                          local.get 9
                          local.get 11
                          i32.add
                          i32.load8_u
                          i32.const 10
                          i32.eq
                          br_if 5 (;@6;)
                          local.get 0
                          local.get 11
                          i32.const 1
                          i32.add
                          local.tee 11
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        local.get 0
                        local.get 10
                        i32.const -8
                        i32.add
                        local.tee 12
                        i32.gt_u
                        br_if 3 (;@7;)
                        br 2 (;@8;)
                      end
                      i32.const 0
                      local.set 11
                      loop ;; label = @10
                        local.get 9
                        local.get 11
                        i32.add
                        i32.load8_u
                        i32.const 10
                        i32.eq
                        br_if 4 (;@6;)
                        local.get 10
                        local.get 11
                        i32.const 1
                        i32.add
                        local.tee 11
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      local.get 2
                      local.set 8
                      br 5 (;@4;)
                    end
                    local.get 10
                    i32.const -8
                    i32.add
                    local.set 12
                    i32.const 0
                    local.set 0
                  end
                  loop ;; label = @8
                    i32.const 16843008
                    local.get 9
                    local.get 0
                    i32.add
                    local.tee 11
                    i32.load
                    local.tee 13
                    i32.const 168430090
                    i32.xor
                    i32.sub
                    local.get 13
                    i32.or
                    i32.const 16843008
                    local.get 11
                    i32.const 4
                    i32.add
                    i32.load
                    local.tee 11
                    i32.const 168430090
                    i32.xor
                    i32.sub
                    local.get 11
                    i32.or
                    i32.and
                    i32.const -2139062144
                    i32.and
                    i32.const -2139062144
                    i32.ne
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 8
                    i32.add
                    local.tee 0
                    local.get 12
                    i32.le_u
                    br_if 0 (;@8;)
                  end
                end
                block ;; label = @7
                  local.get 10
                  local.get 0
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 2
                  local.set 8
                  br 3 (;@4;)
                end
                loop ;; label = @7
                  block ;; label = @8
                    local.get 9
                    local.get 0
                    i32.add
                    i32.load8_u
                    i32.const 10
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 0
                    local.set 11
                    br 2 (;@6;)
                  end
                  local.get 10
                  local.get 0
                  i32.const 1
                  i32.add
                  local.tee 0
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 2
                local.set 8
                br 2 (;@4;)
              end
              local.get 8
              local.get 11
              i32.add
              local.tee 0
              i32.const 1
              i32.add
              local.set 8
              block ;; label = @6
                local.get 0
                local.get 2
                i32.ge_u
                br_if 0 (;@6;)
                local.get 9
                local.get 11
                i32.add
                i32.load8_u
                i32.const 10
                i32.ne
                br_if 0 (;@6;)
                i32.const 0
                local.set 9
                local.get 8
                local.set 10
                local.get 8
                local.set 0
                br 3 (;@3;)
              end
              local.get 2
              local.get 8
              i32.ge_u
              br_if 0 (;@5;)
            end
          end
          local.get 2
          local.get 7
          i32.eq
          br_if 2 (;@1;)
          i32.const 1
          local.set 9
          local.get 7
          local.set 10
          local.get 2
          local.set 0
        end
        block ;; label = @3
          block ;; label = @4
            local.get 5
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 1055752
            i32.const 4
            local.get 3
            i32.load offset=12
            call_indirect (type 4)
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 7
          i32.sub
          local.set 13
          i32.const 0
          local.set 11
          block ;; label = @4
            local.get 0
            local.get 7
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            local.get 0
            i32.add
            i32.const -1
            i32.add
            i32.load8_u
            i32.const 10
            i32.eq
            local.set 11
          end
          local.get 1
          local.get 7
          i32.add
          local.set 0
          local.get 5
          local.get 11
          i32.store8
          local.get 10
          local.set 7
          local.get 4
          local.get 0
          local.get 13
          local.get 3
          i32.load offset=12
          call_indirect (type 4)
          i32.eqz
          br_if 1 (;@2;)
        end
      end
      i32.const 1
      local.set 6
    end
    local.get 6
  )
  (func $_RNvMs1_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_11DebugStruct5field (;168;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block ;; label = @1
      local.get 0
      i32.load8_u offset=4
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u offset=5
      local.set 7
      block ;; label = @2
        local.get 0
        i32.load
        local.tee 8
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 8
        i32.load
        i32.const 1055167
        i32.const 1055164
        local.get 7
        i32.const 1
        i32.and
        local.tee 7
        select
        i32.const 2
        i32.const 3
        local.get 7
        select
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 8
        i32.load
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 8
        i32.load
        i32.const 1055169
        i32.const 2
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 3
        local.get 8
        local.get 4
        i32.load offset=12
        call_indirect (type 5)
        local.set 6
        br 1 (;@1;)
      end
      i32.const 1
      local.set 6
      block ;; label = @2
        local.get 7
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 8
        i32.load
        i32.const 1055171
        i32.const 3
        local.get 8
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
      end
      i32.const 1
      local.set 6
      local.get 5
      i32.const 1
      i32.store8 offset=15
      local.get 5
      i32.const 1055184
      i32.store offset=20
      local.get 5
      local.get 8
      i64.load align=4
      i64.store align=4
      local.get 5
      local.get 8
      i64.load offset=8 align=4
      i64.store offset=24 align=4
      local.get 5
      local.get 5
      i32.const 15
      i32.add
      i32.store offset=8
      local.get 5
      local.get 5
      i32.store offset=16
      local.get 5
      local.get 1
      local.get 2
      call $_RNvXs0_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str
      br_if 0 (;@1;)
      local.get 5
      i32.const 1055169
      i32.const 2
      call $_RNvXs0_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write9write_str
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 3
        local.get 5
        i32.const 16
        i32.add
        local.get 4
        i32.load offset=12
        call_indirect (type 5)
        i32.eqz
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        br 1 (;@1;)
      end
      local.get 5
      i32.load offset=16
      i32.const 1055174
      i32.const 2
      local.get 5
      i32.load offset=20
      i32.load offset=12
      call_indirect (type 4)
      local.set 6
    end
    local.get 0
    i32.const 1
    i32.store8 offset=5
    local.get 0
    local.get 6
    i32.store8 offset=4
    local.get 5
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvMs3_NtNtCsdHhIpgkcIfN_4core3ffi5c_strNtB5_4CStr19from_bytes_with_nul (;169;) (type 9) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i32.const 7
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.eqz
                  br_if 5 (;@2;)
                  local.get 1
                  i32.load8_u
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 3
                  br 6 (;@1;)
                end
                local.get 1
                i32.const 3
                i32.add
                i32.const -4
                i32.and
                local.tee 4
                local.get 1
                i32.eq
                br_if 1 (;@5;)
                local.get 4
                local.get 1
                i32.sub
                local.set 4
                i32.const 0
                local.set 3
                loop ;; label = @7
                  local.get 1
                  local.get 3
                  i32.add
                  i32.load8_u
                  i32.eqz
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 3
                  i32.const 1
                  i32.add
                  local.tee 3
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 4
                local.get 2
                i32.const -8
                i32.add
                local.tee 5
                i32.gt_u
                br_if 3 (;@3;)
                br 2 (;@4;)
              end
              i32.const 1
              local.set 3
              local.get 2
              i32.const 1
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=1
              i32.eqz
              br_if 4 (;@1;)
              i32.const 2
              local.set 3
              local.get 2
              i32.const 2
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=2
              i32.eqz
              br_if 4 (;@1;)
              i32.const 3
              local.set 3
              local.get 2
              i32.const 3
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=3
              i32.eqz
              br_if 4 (;@1;)
              i32.const 4
              local.set 3
              local.get 2
              i32.const 4
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=4
              i32.eqz
              br_if 4 (;@1;)
              i32.const 5
              local.set 3
              local.get 2
              i32.const 5
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=5
              i32.eqz
              br_if 4 (;@1;)
              i32.const 6
              local.set 3
              local.get 2
              i32.const 6
              i32.eq
              br_if 3 (;@2;)
              local.get 1
              i32.load8_u offset=6
              br_if 3 (;@2;)
              br 4 (;@1;)
            end
            local.get 2
            i32.const -8
            i32.add
            local.set 5
            i32.const 0
            local.set 4
          end
          loop ;; label = @4
            i32.const 16843008
            local.get 1
            local.get 4
            i32.add
            local.tee 3
            i32.load
            local.tee 6
            i32.sub
            local.get 6
            i32.or
            i32.const 16843008
            local.get 3
            i32.const 4
            i32.add
            i32.load
            local.tee 3
            i32.sub
            local.get 3
            i32.or
            i32.and
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.ne
            br_if 1 (;@3;)
            local.get 4
            i32.const 8
            i32.add
            local.tee 4
            local.get 5
            i32.le_u
            br_if 0 (;@4;)
          end
        end
        local.get 2
        local.get 4
        i32.eq
        br_if 0 (;@2;)
        loop ;; label = @3
          block ;; label = @4
            local.get 1
            local.get 4
            i32.add
            i32.load8_u
            br_if 0 (;@4;)
            local.get 4
            local.set 3
            br 3 (;@1;)
          end
          local.get 2
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 0
      i32.const 1
      i32.store offset=4
      local.get 0
      i32.const 1
      i32.store
      return
    end
    block ;; label = @1
      local.get 3
      i32.const 1
      i32.add
      local.get 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 3
      i32.store offset=8
      local.get 0
      i32.const 0
      i32.store offset=4
      local.get 0
      i32.const 1
      i32.store
      return
    end
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    i32.const 0
    i32.store
  )
  (func $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral (;170;) (type 14) (param i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    i32.const 43
    i32.const 1114112
    local.get 0
    i32.load offset=8
    local.tee 6
    i32.const 2097152
    i32.and
    local.tee 7
    select
    local.set 8
    local.get 7
    i32.const 21
    i32.shr_u
    i32.const 1
    local.get 1
    select
    local.get 5
    i32.add
    local.set 9
    block ;; label = @1
      block ;; label = @2
        local.get 6
        i32.const 8388608
        i32.and
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 16
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 3
          call $_RNvNtNtCsdHhIpgkcIfN_4core3str5count14do_count_chars
          local.set 7
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 3
          br_if 0 (;@3;)
          i32.const 0
          local.set 7
          br 1 (;@2;)
        end
        local.get 3
        i32.const 3
        i32.and
        local.set 10
        i32.const 0
        local.set 11
        i32.const 0
        local.set 7
        block ;; label = @3
          local.get 3
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          i32.const 12
          i32.and
          local.set 12
          i32.const 0
          local.set 11
          i32.const 0
          local.set 7
          loop ;; label = @4
            local.get 7
            local.get 2
            local.get 11
            i32.add
            local.tee 13
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 13
            i32.const 1
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 13
            i32.const 2
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.get 13
            i32.const 3
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 7
            local.get 12
            local.get 11
            i32.const 4
            i32.add
            local.tee 11
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 10
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 2
        local.get 11
        i32.add
        local.set 13
        loop ;; label = @3
          local.get 7
          local.get 13
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 7
          local.get 13
          i32.const 1
          i32.add
          local.set 13
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          br_if 0 (;@3;)
        end
      end
      local.get 7
      local.get 9
      i32.add
      local.set 9
    end
    local.get 8
    i32.const 45
    local.get 1
    select
    local.set 12
    block ;; label = @1
      block ;; label = @2
        local.get 9
        local.get 0
        i32.load16_u offset=12
        local.tee 1
        i32.ge_u
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 6
              i32.const 16777216
              i32.and
              br_if 0 (;@5;)
              local.get 1
              local.get 9
              i32.sub
              local.set 8
              i32.const 0
              local.set 7
              i32.const 0
              local.set 1
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 6
                    i32.const 29
                    i32.shr_u
                    i32.const 3
                    i32.and
                    br_table 2 (;@6;) 0 (;@8;) 1 (;@7;) 0 (;@8;) 2 (;@6;)
                  end
                  local.get 8
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 8
                i32.const 65534
                i32.and
                i32.const 1
                i32.shr_u
                local.set 1
              end
              local.get 6
              i32.const 2097151
              i32.and
              local.set 9
              local.get 0
              i32.load offset=4
              local.set 11
              local.get 0
              i32.load
              local.set 10
              loop ;; label = @6
                local.get 7
                i32.const 65535
                i32.and
                local.get 1
                i32.const 65535
                i32.and
                i32.ge_u
                br_if 2 (;@4;)
                i32.const 1
                local.set 13
                local.get 7
                i32.const 1
                i32.add
                local.set 7
                local.get 10
                local.get 9
                local.get 11
                i32.load offset=16
                call_indirect (type 5)
                i32.eqz
                br_if 0 (;@6;)
                br 5 (;@1;)
              end
            end
            local.get 0
            local.get 0
            i64.load offset=8 align=4
            local.tee 14
            i32.wrap_i64
            i32.const -1612709888
            i32.and
            i32.const 536870960
            i32.or
            i32.store offset=8
            i32.const 1
            local.set 13
            local.get 0
            i32.load
            local.tee 10
            local.get 0
            i32.load offset=4
            local.tee 11
            local.get 12
            local.get 2
            local.get 3
            call $_RNvNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB7_9Formatter12pad_integral12write_prefix
            br_if 3 (;@1;)
            i32.const 0
            local.set 7
            local.get 1
            local.get 9
            i32.sub
            i32.const 65535
            i32.and
            local.set 2
            loop ;; label = @5
              local.get 7
              i32.const 65535
              i32.and
              local.get 2
              i32.ge_u
              br_if 2 (;@3;)
              i32.const 1
              local.set 13
              local.get 7
              i32.const 1
              i32.add
              local.set 7
              local.get 10
              i32.const 48
              local.get 11
              i32.load offset=16
              call_indirect (type 5)
              i32.eqz
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          i32.const 1
          local.set 13
          local.get 10
          local.get 11
          local.get 12
          local.get 2
          local.get 3
          call $_RNvNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB7_9Formatter12pad_integral12write_prefix
          br_if 2 (;@1;)
          local.get 10
          local.get 4
          local.get 5
          local.get 11
          i32.load offset=12
          call_indirect (type 4)
          br_if 2 (;@1;)
          i32.const 0
          local.set 7
          local.get 8
          local.get 1
          i32.sub
          i32.const 65535
          i32.and
          local.set 0
          loop ;; label = @4
            local.get 7
            i32.const 65535
            i32.and
            local.tee 2
            local.get 0
            i32.lt_u
            local.set 13
            local.get 2
            local.get 0
            i32.ge_u
            br_if 3 (;@1;)
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 10
            local.get 9
            local.get 11
            i32.load offset=16
            call_indirect (type 5)
            i32.eqz
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        i32.const 1
        local.set 13
        local.get 10
        local.get 4
        local.get 5
        local.get 11
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        local.get 0
        local.get 14
        i64.store offset=8 align=4
        i32.const 0
        return
      end
      i32.const 1
      local.set 13
      local.get 0
      i32.load
      local.tee 7
      local.get 0
      i32.load offset=4
      local.tee 10
      local.get 12
      local.get 2
      local.get 3
      call $_RNvNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB7_9Formatter12pad_integral12write_prefix
      br_if 0 (;@1;)
      local.get 7
      local.get 4
      local.get 5
      local.get 10
      i32.load offset=12
      call_indirect (type 4)
      local.set 13
    end
    local.get 13
  )
  (func $_RNvNtNtCsdHhIpgkcIfN_4core3str5count14do_count_chars (;171;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        local.get 0
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.tee 2
        local.get 0
        i32.sub
        local.tee 3
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        local.get 3
        i32.sub
        local.tee 4
        i32.const 2
        i32.shr_u
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.const 3
        i32.and
        local.set 6
        i32.const 0
        local.set 7
        i32.const 0
        local.set 1
        block ;; label = @3
          local.get 2
          local.get 0
          i32.eq
          br_if 0 (;@3;)
          i32.const 0
          local.set 8
          i32.const 0
          local.set 1
          block ;; label = @4
            local.get 0
            local.get 2
            i32.sub
            local.tee 9
            i32.const -4
            i32.gt_u
            br_if 0 (;@4;)
            i32.const 0
            local.set 8
            i32.const 0
            local.set 1
            loop ;; label = @5
              local.get 1
              local.get 0
              local.get 8
              i32.add
              local.tee 2
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 1
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 2
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.get 2
              i32.const 3
              i32.add
              i32.load8_s
              i32.const -65
              i32.gt_s
              i32.add
              local.set 1
              local.get 8
              i32.const 4
              i32.add
              local.tee 8
              br_if 0 (;@5;)
            end
          end
          local.get 0
          local.get 8
          i32.add
          local.set 2
          loop ;; label = @4
            local.get 1
            local.get 2
            i32.load8_s
            i32.const -65
            i32.gt_s
            i32.add
            local.set 1
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            local.get 9
            i32.const 1
            i32.add
            local.tee 9
            br_if 0 (;@4;)
          end
        end
        local.get 0
        local.get 3
        i32.add
        local.set 9
        block ;; label = @3
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          local.get 9
          local.get 4
          i32.const 2147483644
          i32.and
          i32.add
          local.tee 2
          i32.load8_s
          i32.const -65
          i32.gt_s
          local.set 7
          local.get 6
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 7
          local.get 2
          i32.load8_s offset=1
          i32.const -65
          i32.gt_s
          i32.add
          local.set 7
          local.get 6
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 7
          local.get 2
          i32.load8_s offset=2
          i32.const -65
          i32.gt_s
          i32.add
          local.set 7
        end
        local.get 7
        local.get 1
        i32.add
        local.set 8
        loop ;; label = @3
          local.get 9
          local.set 3
          local.get 5
          i32.eqz
          br_if 2 (;@1;)
          local.get 5
          i32.const 192
          local.get 5
          i32.const 192
          i32.lt_u
          select
          local.tee 7
          i32.const 3
          i32.and
          local.set 6
          block ;; label = @4
            block ;; label = @5
              local.get 7
              i32.const 2
              i32.shl
              local.tee 4
              i32.const 1008
              i32.and
              local.tee 1
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              br 1 (;@4;)
            end
            local.get 3
            local.get 1
            i32.add
            local.set 0
            i32.const 0
            local.set 2
            local.get 3
            local.set 1
            loop ;; label = @5
              local.get 1
              i32.const 12
              i32.add
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.const 8
              i32.add
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 1
              i32.load
              local.tee 9
              i32.const -1
              i32.xor
              i32.const 7
              i32.shr_u
              local.get 9
              i32.const 6
              i32.shr_u
              i32.or
              i32.const 16843009
              i32.and
              local.get 2
              i32.add
              i32.add
              i32.add
              i32.add
              local.set 2
              local.get 1
              i32.const 16
              i32.add
              local.tee 1
              local.get 0
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 5
          local.get 7
          i32.sub
          local.set 5
          local.get 3
          local.get 4
          i32.add
          local.set 9
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 16711935
          i32.and
          local.get 2
          i32.const 16711935
          i32.and
          i32.add
          i32.const 65537
          i32.mul
          i32.const 16
          i32.shr_u
          local.get 8
          i32.add
          local.set 8
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 3
        local.get 7
        i32.const 252
        i32.and
        i32.const 2
        i32.shl
        i32.add
        local.tee 2
        i32.load
        local.tee 1
        i32.const -1
        i32.xor
        i32.const 7
        i32.shr_u
        local.get 1
        i32.const 6
        i32.shr_u
        i32.or
        i32.const 16843009
        i32.and
        local.set 1
        block ;; label = @3
          local.get 6
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=4
          local.tee 9
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 9
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
          local.get 6
          i32.const 2
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=8
          local.tee 2
          i32.const -1
          i32.xor
          i32.const 7
          i32.shr_u
          local.get 2
          i32.const 6
          i32.shr_u
          i32.or
          i32.const 16843009
          i32.and
          local.get 1
          i32.add
          local.set 1
        end
        local.get 1
        i32.const 8
        i32.shr_u
        i32.const 459007
        i32.and
        local.get 1
        i32.const 16711935
        i32.and
        i32.add
        i32.const 65537
        i32.mul
        i32.const 16
        i32.shr_u
        local.get 8
        i32.add
        local.set 8
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 1
      i32.const 3
      i32.and
      local.set 2
      i32.const 0
      local.set 9
      i32.const 0
      local.set 8
      block ;; label = @2
        local.get 1
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const -4
        i32.and
        local.set 5
        i32.const 0
        local.set 8
        i32.const 0
        local.set 9
        loop ;; label = @3
          local.get 8
          local.get 0
          local.get 9
          i32.add
          local.tee 1
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 1
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 2
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.get 1
          i32.const 3
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          i32.add
          local.set 8
          local.get 5
          local.get 9
          i32.const 4
          i32.add
          local.tee 9
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 0
      local.get 9
      i32.add
      local.set 1
      loop ;; label = @2
        local.get 8
        local.get 1
        i32.load8_s
        i32.const -65
        i32.gt_s
        i32.add
        local.set 8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    local.get 8
  )
  (func $_RNvNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB7_9Formatter12pad_integral12write_prefix (;172;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
    block ;; label = @1
      local.get 2
      i32.const 1114112
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      local.get 1
      i32.load offset=16
      call_indirect (type 5)
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    block ;; label = @1
      local.get 3
      br_if 0 (;@1;)
      i32.const 0
      return
    end
    local.get 0
    local.get 3
    local.get 4
    local.get 1
    i32.load offset=12
    call_indirect (type 4)
  )
  (func $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish (;173;) (type 12) (param i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    i32.const 1
    local.set 6
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 7
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      local.tee 8
      i32.load offset=12
      local.tee 9
      call_indirect (type 4)
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load8_u offset=10
          i32.const 128
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 6
          local.get 7
          i32.const 1055176
          i32.const 1
          local.get 9
          call_indirect (type 4)
          br_if 2 (;@1;)
          local.get 3
          local.get 0
          local.get 4
          i32.load offset=12
          call_indirect (type 5)
          i32.eqz
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 7
        i32.const 1055177
        i32.const 2
        local.get 9
        call_indirect (type 4)
        br_if 1 (;@1;)
        i32.const 1
        local.set 6
        local.get 5
        i32.const 1
        i32.store8 offset=15
        local.get 5
        local.get 8
        i32.store offset=4
        local.get 5
        local.get 7
        i32.store
        local.get 5
        i32.const 1055184
        i32.store offset=20
        local.get 5
        local.get 0
        i64.load offset=8 align=4
        i64.store offset=24 align=4
        local.get 5
        local.get 5
        i32.const 15
        i32.add
        i32.store offset=8
        local.get 5
        local.get 5
        i32.store offset=16
        local.get 3
        local.get 5
        i32.const 16
        i32.add
        local.get 4
        i32.load offset=12
        call_indirect (type 5)
        br_if 1 (;@1;)
        local.get 5
        i32.load offset=16
        i32.const 1055174
        i32.const 2
        local.get 5
        i32.load offset=20
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
      end
      block ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        local.get 0
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 6
        local.get 0
        i32.load
        i32.const 1055181
        i32.const 1
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
      end
      local.get 0
      i32.load
      i32.const 1055180
      i32.const 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 4)
      local.set 6
    end
    local.get 5
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 6
  )
  (func $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish (;174;) (type 15) (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 11
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 4)
    local.set 2
    local.get 11
    i32.const 0
    i32.store8 offset=13
    local.get 11
    local.get 2
    i32.store8 offset=12
    local.get 11
    local.get 0
    i32.store offset=8
    local.get 11
    i32.const 8
    i32.add
    local.get 3
    local.get 4
    local.get 5
    local.get 6
    call $_RNvMs1_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_11DebugStruct5field
    local.get 7
    local.get 8
    local.get 9
    local.get 10
    call $_RNvMs1_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_11DebugStruct5field
    local.set 10
    local.get 11
    i32.load8_u offset=13
    local.tee 2
    local.get 11
    i32.load8_u offset=12
    local.tee 1
    i32.or
    local.set 0
    block ;; label = @1
      local.get 2
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 10
        i32.load
        local.tee 0
        i32.load8_u offset=10
        i32.const 128
        i32.and
        br_if 0 (;@2;)
        local.get 0
        i32.load
        i32.const 1055182
        i32.const 2
        local.get 0
        i32.load offset=4
        i32.load offset=12
        call_indirect (type 4)
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      i32.const 1055179
      i32.const 1
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 4)
      local.set 0
    end
    local.get 11
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
    i32.const 1
    i32.and
  )
  (func $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter3pad (;175;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.const 402653184
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const 268435456
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load16_u offset=14
                  local.tee 4
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 2
                  br 2 (;@5;)
                end
                block ;; label = @7
                  local.get 2
                  i32.const 16
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 2
                  call $_RNvNtNtCsdHhIpgkcIfN_4core3str5count14do_count_chars
                  local.set 5
                  br 4 (;@3;)
                end
                block ;; label = @7
                  local.get 2
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 5
                  br 4 (;@3;)
                end
                local.get 2
                i32.const 3
                i32.and
                local.set 6
                i32.const 0
                local.set 7
                i32.const 0
                local.set 5
                block ;; label = @7
                  local.get 2
                  i32.const 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 12
                  i32.and
                  local.set 4
                  i32.const 0
                  local.set 5
                  i32.const 0
                  local.set 7
                  loop ;; label = @8
                    local.get 5
                    local.get 1
                    local.get 7
                    i32.add
                    local.tee 8
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 8
                    i32.const 1
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 8
                    i32.const 2
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 8
                    i32.const 3
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 5
                    local.get 4
                    local.get 7
                    i32.const 4
                    i32.add
                    local.tee 7
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 6
                  i32.eqz
                  br_if 4 (;@3;)
                end
                local.get 1
                local.get 7
                i32.add
                local.set 8
                loop ;; label = @7
                  local.get 5
                  local.get 8
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  i32.add
                  local.set 5
                  local.get 8
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 6
                  i32.const -1
                  i32.add
                  local.tee 6
                  br_if 0 (;@7;)
                  br 4 (;@3;)
                end
              end
              local.get 1
              local.get 2
              i32.add
              local.set 7
              i32.const 0
              local.set 2
              local.get 1
              local.set 8
              local.get 4
              local.set 6
              loop ;; label = @6
                local.get 8
                local.tee 5
                local.get 7
                i32.eq
                br_if 2 (;@4;)
                block ;; label = @7
                  block ;; label = @8
                    local.get 5
                    i32.load8_s
                    local.tee 8
                    i32.const -1
                    i32.le_s
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 1
                    i32.add
                    local.set 8
                    br 1 (;@7;)
                  end
                  block ;; label = @8
                    local.get 8
                    i32.const -32
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 5
                    i32.const 2
                    i32.add
                    local.set 8
                    br 1 (;@7;)
                  end
                  local.get 5
                  i32.const 4
                  i32.const 3
                  local.get 8
                  i32.const -17
                  i32.gt_u
                  select
                  i32.add
                  local.set 8
                end
                local.get 8
                local.get 5
                i32.sub
                local.get 2
                i32.add
                local.set 2
                local.get 6
                i32.const -1
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            local.set 6
          end
          local.get 4
          local.get 6
          i32.sub
          local.set 5
        end
        local.get 5
        local.get 0
        i32.load16_u offset=12
        local.tee 8
        i32.ge_u
        br_if 0 (;@2;)
        local.get 8
        local.get 5
        i32.sub
        local.set 9
        i32.const 0
        local.set 5
        i32.const 0
        local.set 4
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.const 29
              i32.shr_u
              i32.const 3
              i32.and
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;) 2 (;@3;)
            end
            local.get 9
            local.set 4
            br 1 (;@3;)
          end
          local.get 9
          i32.const 65534
          i32.and
          i32.const 1
          i32.shr_u
          local.set 4
        end
        local.get 3
        i32.const 2097151
        i32.and
        local.set 7
        local.get 0
        i32.load offset=4
        local.set 6
        local.get 0
        i32.load
        local.set 0
        block ;; label = @3
          loop ;; label = @4
            local.get 5
            i32.const 65535
            i32.and
            local.get 4
            i32.const 65535
            i32.and
            i32.ge_u
            br_if 1 (;@3;)
            i32.const 1
            local.set 8
            local.get 5
            i32.const 1
            i32.add
            local.set 5
            local.get 0
            local.get 7
            local.get 6
            i32.load offset=16
            call_indirect (type 5)
            br_if 3 (;@1;)
            br 0 (;@4;)
          end
        end
        i32.const 1
        local.set 8
        local.get 0
        local.get 1
        local.get 2
        local.get 6
        i32.load offset=12
        call_indirect (type 4)
        br_if 1 (;@1;)
        i32.const 0
        local.set 5
        local.get 9
        local.get 4
        i32.sub
        i32.const 65535
        i32.and
        local.set 2
        loop ;; label = @3
          local.get 5
          i32.const 65535
          i32.and
          local.tee 4
          local.get 2
          i32.lt_u
          local.set 8
          local.get 4
          local.get 2
          i32.ge_u
          br_if 2 (;@1;)
          local.get 5
          i32.const 1
          i32.add
          local.set 5
          local.get 0
          local.get 7
          local.get 6
          i32.load offset=16
          call_indirect (type 5)
          br_if 2 (;@1;)
          br 0 (;@3;)
        end
      end
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 4)
      local.set 8
    end
    local.get 8
  )
  (func $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter9write_str (;176;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 4)
  )
  (func $_RNvXs8_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3impmNtB9_7Display3fmt (;177;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 10
    local.set 3
    local.get 0
    i32.load
    local.tee 4
    local.set 5
    block ;; label = @1
      local.get 4
      i32.const 1000
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 10
      local.set 3
      local.get 4
      local.set 5
      loop ;; label = @2
        local.get 2
        i32.const 6
        i32.add
        local.get 3
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 5
        local.tee 0
        local.get 0
        i32.const 10000
        i32.div_u
        local.tee 5
        i32.const 10000
        i32.mul
        i32.sub
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.load16_u offset=1055262 align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1055262 align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 0
        i32.const 9999999
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      block ;; label = @2
        local.get 5
        i32.const 9
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 5
      local.get 5
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 0
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1055262 align=1
      i32.store16 align=1
    end
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 0
      i32.const 1
      i32.shl
      i32.load8_u offset=1055263
      i32.store8
    end
    local.get 1
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 6
    i32.add
    local.get 3
    i32.add
    i32.const 10
    local.get 3
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core6option13unwrap_failed (;178;) (type 1) (param i32)
    i32.const 1055208
    i32.const 43
    local.get 0
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking5panic
    unreachable
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core4cell22panic_already_borrowed (;179;) (type 1) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 65
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 1
    i32.const 15
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store
    i32.const 1048985
    local.get 1
    local.get 0
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvXsr_NtCsdHhIpgkcIfN_4core4cellNtB5_14BorrowMutErrorNtNtB7_3fmt7Display3fmt (;180;) (type 5) (param i32 i32) (result i32)
    local.get 1
    i32.const 1055783
    i32.const 24
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter3pad
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core6option13expect_failed (;181;) (type 9) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    local.get 1
    i32.store offset=4
    local.get 3
    local.get 0
    i32.store
    local.get 3
    i32.const 64
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 3
    i64.extend_i32_u
    i64.or
    i64.store offset=8
    i32.const 1048985
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvNtCsdHhIpgkcIfN_4core6result13unwrap_failed (;182;) (type 11) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.get 1
    i32.store offset=4
    local.get 5
    local.get 0
    i32.store
    local.get 5
    local.get 3
    i32.store offset=12
    local.get 5
    local.get 2
    i32.store offset=8
    local.get 5
    i32.const 62
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 5
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.or
    i64.store offset=24
    local.get 5
    i32.const 64
    i64.extend_i32_u
    i64.const 32
    i64.shl
    local.get 5
    i64.extend_i32_u
    i64.or
    i64.store offset=16
    i32.const 1048981
    local.get 5
    i32.const 16
    i32.add
    local.get 4
    call $_RNvNtCsdHhIpgkcIfN_4core9panicking9panic_fmt
    unreachable
  )
  (func $_RNvXs1g_NtCsdHhIpgkcIfN_4core3fmtRDNtB6_5DebugEL_Bx_3fmtB8_ (;183;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 5)
  )
  (func $_RNvXs8_NtCsdHhIpgkcIfN_4core3fmtNtB5_9ArgumentsNtB5_7Display3fmt (;184;) (type 5) (param i32 i32) (result i32)
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (func $_RNvXs6_NtNtCsdHhIpgkcIfN_4core3fmt3numjNtB7_8LowerHex3fmt (;185;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 3
    i32.const 0
    local.set 0
    loop ;; label = @1
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.add
      i32.const 7
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.load8_u offset=1055148
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1055756
    i32.const 2
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.add
    i32.const 8
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvNtNtCsdHhIpgkcIfN_4core5slice6memchr14memchr_aligned (;186;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 3
            i32.add
            i32.const -4
            i32.and
            local.tee 4
            local.get 2
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            i32.const -8
            i32.add
            local.set 5
            i32.const 0
            local.set 4
            br 1 (;@3;)
          end
          local.get 3
          local.get 4
          local.get 2
          i32.sub
          local.tee 4
          local.get 3
          local.get 4
          i32.lt_u
          select
          local.set 4
          block ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            local.set 6
            local.get 1
            i32.const 255
            i32.and
            local.set 7
            i32.const 1
            local.set 8
            loop ;; label = @5
              local.get 2
              local.get 6
              i32.add
              i32.load8_u
              local.get 7
              i32.eq
              br_if 4 (;@1;)
              local.get 4
              local.get 6
              i32.const 1
              i32.add
              local.tee 6
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 4
          local.get 3
          i32.const -8
          i32.add
          local.tee 5
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.set 6
        loop ;; label = @3
          i32.const 16843008
          local.get 2
          local.get 4
          i32.add
          local.tee 7
          i32.load
          local.get 6
          i32.xor
          local.tee 8
          i32.sub
          local.get 8
          i32.or
          i32.const 16843008
          local.get 7
          i32.const 4
          i32.add
          i32.load
          local.get 6
          i32.xor
          local.tee 7
          i32.sub
          local.get 7
          i32.or
          i32.and
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.ne
          br_if 1 (;@2;)
          local.get 4
          i32.const 8
          i32.add
          local.tee 4
          local.get 5
          i32.le_u
          br_if 0 (;@3;)
        end
      end
      block ;; label = @2
        local.get 3
        local.get 4
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        local.set 6
        i32.const 1
        local.set 8
        loop ;; label = @3
          block ;; label = @4
            local.get 2
            local.get 4
            i32.add
            i32.load8_u
            local.get 6
            i32.ne
            br_if 0 (;@4;)
            local.get 4
            local.set 6
            br 3 (;@1;)
          end
          local.get 3
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          i32.ne
          br_if 0 (;@3;)
        end
      end
      i32.const 0
      local.set 8
    end
    local.get 0
    local.get 6
    i32.store offset=4
    local.get 0
    local.get 8
    i32.store
  )
  (func $_RNvNtNtCsdHhIpgkcIfN_4core5slice6memchr7memrchr (;187;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 3
    local.set 4
    local.get 3
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                local.get 2
                i32.const 3
                i32.add
                i32.const -4
                i32.and
                local.get 2
                i32.sub
                local.tee 6
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                local.get 3
                local.get 6
                i32.sub
                i32.const 7
                i32.and
                local.tee 7
                i32.sub
                local.set 4
                local.get 3
                local.get 7
                i32.lt_u
                br_if 1 (;@5;)
                local.get 6
                local.set 5
              end
              i32.const 0
              local.get 4
              i32.sub
              local.set 8
              local.get 2
              i32.const -1
              i32.add
              local.set 9
              local.get 1
              i32.const 255
              i32.and
              local.set 10
              local.get 3
              local.set 6
              loop ;; label = @6
                local.get 8
                local.get 6
                i32.add
                i32.eqz
                br_if 2 (;@4;)
                local.get 9
                local.get 6
                i32.add
                local.set 7
                local.get 6
                i32.const -1
                i32.add
                local.set 6
                local.get 7
                i32.load8_u
                local.get 10
                i32.ne
                br_if 0 (;@6;)
                br 3 (;@3;)
              end
            end
            local.get 4
            local.get 3
            local.get 3
            i32.const 1055736
            call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
            unreachable
          end
          local.get 1
          i32.const 255
          i32.and
          i32.const 16843009
          i32.mul
          local.set 7
          block ;; label = @4
            loop ;; label = @5
              local.get 4
              local.tee 6
              local.get 5
              i32.le_u
              br_if 1 (;@4;)
              local.get 6
              i32.const -8
              i32.add
              local.set 4
              i32.const 16843008
              local.get 2
              local.get 6
              i32.add
              local.tee 8
              i32.const -8
              i32.add
              i32.load
              local.get 7
              i32.xor
              local.tee 9
              i32.sub
              local.get 9
              i32.or
              i32.const 16843008
              local.get 8
              i32.const -4
              i32.add
              i32.load
              local.get 7
              i32.xor
              local.tee 8
              i32.sub
              local.get 8
              i32.or
              i32.and
              i32.const -2139062144
              i32.and
              i32.const -2139062144
              i32.eq
              br_if 0 (;@5;)
            end
          end
          local.get 6
          local.get 3
          i32.gt_u
          br_if 2 (;@1;)
          local.get 2
          i32.const -1
          i32.add
          local.set 4
          local.get 1
          i32.const 255
          i32.and
          local.set 8
          loop ;; label = @4
            block ;; label = @5
              local.get 6
              br_if 0 (;@5;)
              i32.const 0
              local.set 7
              br 3 (;@2;)
            end
            local.get 4
            local.get 6
            i32.add
            local.set 7
            local.get 6
            i32.const -1
            i32.add
            local.set 6
            local.get 7
            i32.load8_u
            local.get 8
            i32.ne
            br_if 0 (;@4;)
          end
        end
        i32.const 1
        local.set 7
      end
      local.get 0
      local.get 6
      i32.store offset=4
      local.get 0
      local.get 7
      i32.store
      return
    end
    i32.const 0
    local.get 6
    local.get 3
    i32.const 1055720
    call $_RNvNtNtCsdHhIpgkcIfN_4core5slice5index16slice_index_fail
    unreachable
  )
  (func $_RNvXNtNtNtCsdHhIpgkcIfN_4core3fmt3num3imphNtB6_7Display3fmt (;188;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 3
    local.set 3
    local.get 0
    i32.load8_u
    local.tee 0
    local.set 4
    block ;; label = @1
      local.get 0
      i32.const 10
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 1
      local.set 3
      local.get 2
      local.get 0
      local.get 0
      i32.const 100
      i32.div_u
      local.tee 4
      i32.const 100
      i32.mul
      i32.sub
      i32.const 255
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1055262 align=1
      i32.store16 offset=14 align=1
    end
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 13
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 4
      i32.const 1
      i32.shl
      i32.load8_u offset=1055263
      i32.store8
    end
    local.get 1
    i32.const 1
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 13
    i32.add
    local.get 3
    i32.add
    i32.const 3
    local.get 3
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_RNvXs0_NtNtCsdHhIpgkcIfN_4core3fmt8buildersNtB5_10PadAdapterNtB7_5Write10write_char (;189;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.set 2
    local.get 0
    i32.load
    local.set 3
    block ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 0
      i32.load8_u
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 1055752
      i32.const 4
      local.get 2
      i32.load offset=12
      call_indirect (type 4)
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    local.get 0
    local.get 1
    i32.const 10
    i32.eq
    i32.store8
    local.get 3
    local.get 1
    local.get 2
    i32.load offset=16
    call_indirect (type 5)
  )
  (func $_RNvXsg_NtNtCsdHhIpgkcIfN_4core3fmt3numhNtB7_8UpperHex3fmt (;190;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop ;; label = @1
      local.get 2
      i32.const 14
      i32.add
      local.get 0
      i32.add
      i32.const 1
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.const 1055758
      i32.add
      i32.load8_u
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1055756
    i32.const 2
    local.get 2
    i32.const 14
    i32.add
    local.get 0
    i32.add
    i32.const 2
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvXs9_NtNtNtCsdHhIpgkcIfN_4core3fmt3num3implNtB9_7Display3fmt (;191;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 10
    local.set 3
    block ;; label = @1
      local.get 0
      i32.load
      local.tee 4
      local.get 4
      i32.const 31
      i32.shr_s
      local.tee 0
      i32.xor
      local.get 0
      i32.sub
      local.tee 5
      i32.const 1000
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 10
      local.set 3
      loop ;; label = @2
        local.get 2
        i32.const 6
        i32.add
        local.get 3
        i32.add
        local.tee 6
        i32.const -4
        i32.add
        local.get 5
        local.tee 0
        local.get 0
        i32.const 10000
        i32.div_u
        local.tee 5
        i32.const 10000
        i32.mul
        i32.sub
        local.tee 7
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 8
        i32.const 1
        i32.shl
        i32.load16_u offset=1055262 align=1
        i32.store16 align=1
        local.get 6
        i32.const -2
        i32.add
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1055262 align=1
        i32.store16 align=1
        local.get 3
        i32.const -4
        i32.add
        local.set 3
        local.get 0
        i32.const 9999999
        i32.gt_u
        br_if 0 (;@2;)
      end
    end
    block ;; label = @1
      block ;; label = @2
        local.get 5
        i32.const 9
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -2
      i32.add
      local.tee 3
      i32.add
      local.get 5
      local.get 5
      i32.const 65535
      i32.and
      i32.const 100
      i32.div_u
      local.tee 0
      i32.const 100
      i32.mul
      i32.sub
      i32.const 65535
      i32.and
      i32.const 1
      i32.shl
      i32.load16_u offset=1055262 align=1
      i32.store16 align=1
    end
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const 6
      i32.add
      local.get 3
      i32.const -1
      i32.add
      local.tee 3
      i32.add
      local.get 0
      i32.const 1
      i32.shl
      i32.load8_u offset=1055263
      i32.store8
    end
    local.get 1
    local.get 4
    i32.const -1
    i32.xor
    i32.const 31
    i32.shr_u
    i32.const 1
    i32.const 0
    local.get 2
    i32.const 6
    i32.add
    local.get 3
    i32.add
    i32.const 10
    local.get 3
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 3
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 3
  )
  (func $_RNvXse_NtNtCsdHhIpgkcIfN_4core3fmt3numhNtB7_8LowerHex3fmt (;192;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load8_u
    local.set 3
    i32.const 0
    local.set 0
    loop ;; label = @1
      local.get 2
      i32.const 14
      i32.add
      local.get 0
      i32.add
      i32.const 1
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.const 1055148
      i32.add
      i32.load8_u
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      i32.const 15
      i32.and
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1055756
    i32.const 2
    local.get 2
    i32.const 14
    i32.add
    local.get 0
    i32.add
    i32.const 2
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvXs8_NtNtCsdHhIpgkcIfN_4core3fmt3numjNtB7_8UpperHex3fmt (;193;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    local.get 0
    i32.load
    local.set 3
    i32.const 0
    local.set 0
    loop ;; label = @1
      local.get 2
      i32.const 8
      i32.add
      local.get 0
      i32.add
      i32.const 7
      i32.add
      local.get 3
      i32.const 15
      i32.and
      i32.load8_u offset=1055758
      i32.store8
      local.get 0
      i32.const -1
      i32.add
      local.set 0
      local.get 3
      i32.const 4
      i32.shr_u
      local.tee 3
      br_if 0 (;@1;)
    end
    local.get 1
    i32.const 1
    i32.const 1055756
    i32.const 2
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.add
    i32.const 8
    i32.add
    i32.const 0
    local.get 0
    i32.sub
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter12pad_integral
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 0
  )
  (func $_RNvXsg_NtCsdHhIpgkcIfN_4core3fmtbNtB5_7Display3fmt (;194;) (type 5) (param i32 i32) (result i32)
    block ;; label = @1
      local.get 0
      i32.load8_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1055774
      i32.const 5
      call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter3pad
      return
    end
    local.get 1
    i32.const 1055779
    i32.const 4
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter3pad
  )
  (func $_RNvXsi_NtCsdHhIpgkcIfN_4core3fmteNtB5_7Display3fmt (;195;) (type 4) (param i32 i32 i32) (result i32)
    local.get 2
    local.get 0
    local.get 1
    call $_RNvMsa_NtCsdHhIpgkcIfN_4core3fmtNtB5_9Formatter3pad
  )
  (func $_RNvYNtNtNtCsdHhIpgkcIfN_4core3fmt8builders10PadAdapterNtB6_5Write9write_fmtB8_ (;196;) (type 4) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1055184
    local.get 1
    local.get 2
    call $_RNvNtCsdHhIpgkcIfN_4core3fmt5write
  )
  (data $.rodata (;0;) (i32.const 1048576) "Hello from target!\0ainternal error: entered unreachable code: This should trigger the wrap_unreachable mechanism\c0\01:\c0\01:\c0\00\16slice index starts at \c0\0d but ends at \c0\00\12range start index \c0\22 out of range for slice of length \c0\00\10range end index \c0\22 out of range for slice of length \c0\00\10assertion `left \c0\17 right` failed\0a  left: \c0\09\0a right: \c0\00\10assertion `left \c0\10 right` failed: \c0\09\0a  left: \c0\09\0a right: \c0\00\13failed printing to \c0\02: \c0\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/slice/index.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/rt.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sys/sync/mutex/no_threads.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/slice/memchr.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs\00examples/test_wasm/test_unreachable_target/src/main.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/buffered/linewritershim.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sync/reentrant_lock.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sys/io/error/wasi.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sync/once.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/mod.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/alloc/src/raw_vec/mod.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/thread/id.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sys/io/io_slice/iovec.rs\00/\00\c0\0b (os error \c0\01)\00\15memory allocation of \c0G bytes failed\0askipping backtrace printing to avoid potential recursion\0a\005fatal runtime error: failed to initiate panic, error \c0\0b, aborting\0a\00\15memory allocation of \c0\0e bytes failed\0a\00\0cpanicked at \c0\02:\0a\c03\0athread panicked while processing panic. aborting.\0a\00\09\0athread '\c0\03' (\c0\0e) panicked at \c0\02:\0a\c0\01\0a\00\19aborting due to panic at \c0\02:\0a\c0\01\0a\00\00\00\00+\03\10\006\00\00\00\05\00\00\00\05\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\02\00\00\00\03\00\00\00\03\00\00\00\b9\04\10\00L\00\00\00\db\00\00\00\14\00\00\00Once instance has previously been poisonedone-time initialization may not be performed recursively\00\00\00\00\00\00\04\00\00\00\04\00\00\00\0f\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\10\00\00\00\11\00\00\00\0c\00\00\00\04\00\00\00\12\00\00\00\13\00\00\00\14\00\00\00a formatting trait implementation returned an error when the underlying stream did not\00\00\06\05\10\00I\00\00\00\88\02\00\00\11\00\00\00\11\00\00\00\0c\00\00\00\04\00\00\00\15\00\00\00\16\00\00\00\17\00\00\00\11\00\00\00\0c\00\00\00\04\00\00\00\18\00\00\00\19\00\00\00\1a\00\00\00\11\00\00\00\0c\00\00\00\04\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\df\02\10\00K\00\00\00\8d\04\00\00\09\00\00\00file name contained an unexpected NUL byte\00\00L\09\10\00*\00\00\00\14\00\00\00\00\00\00\00\02\00\00\00x\09\10\00\b9\04\10\00L\00\00\00\9f\00\00\002\00\00\00main<unnamed>\00\00\00l\04\10\00L\00\00\00\16\01\00\00.\00\00\00\1e\00\00\00\0c\00\00\00\04\00\00\00\1f\00\00\00 \00\00\00!\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00\00\00\00\00\08\00\00\00\04\00\00\00\22\00\00\00#\00\00\00$\00\00\00%\00\00\00&\00\00\00\10\00\00\00\04\00\00\00'\00\00\00(\00\00\00)\00\00\00*\00\00\00called `Result::unwrap()` on an `Err` valuemid > lenm]\cb\d6,P\ebcxA\a6Wq\1b\8b\b9\e4\fd\9e\8f\ba8\e8\93\b7\8acNw\af%\80fatal runtime error: rwlock locked for writing, aborting\0aRUST_BACKTRACEentity not foundpermission deniedconnection refusedconnection resethost unreachablenetwork unreachableconnection abortednot connectedaddress in useaddress not availablenetwork downbroken pipeentity already existsoperation would blocknot a directoryis a directorydirectory not emptyread-only filesystem or storage mediumfilesystem loop or indirection limit (e.g. symlink loop)stale network file handleinvalid input parameterinvalid datatimed outwrite zerono storage spaceseek on unseekable filequota exceededfile too largeresource busyexecutable file busydeadlockcross-device link or renametoo many linksinvalid filenameargument list too longoperation interruptedunsupportedunexpected end of fileout of memoryin progressother erroruncategorized errorcannot recursively acquire mutex1\02\10\00\5c\00\00\00\13\00\00\00\09\00\00\00lock count overflow in reentrant mutex\00\00\c0\03\10\00V\00\00\00#\01\00\00-\00\00\00advancing io slices beyond their length\00\06\05\10\00I\00\00\00Z\06\00\00\0d\00\00\00advancing IoSlice beyond its length\00\ee\05\10\00X\00\00\00\1f\00\00\00\0d\00\00\00\06\05\10\00I\00\00\00X\06\00\00 \00\00\00failed to write the buffered data\00\00\00\cc\0e\10\00!\00\00\00\17\00\00\00\00\00\00\00\eb\01\10\00E\00\00\00\8d\00\00\00\0d\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00+\00\00\00,\00\00\00-\00\00\00.\00\00\00/\00\00\000\00\00\00\0a\00\00\001\00\00\00\0c\00\00\00\04\00\00\002\00\00\003\00\00\004\00\00\005\00\00\006\00\00\007\00\00\008\00\00\00Box<dyn Any>thread caused non-unwinding panic. aborting.\0a\00\00\00\9b\01\10\00O\00\00\00\fc\03\00\003\00\00\00stdout\00\00\df\02\10\00K\00\00\00\e3\02\00\00\13\00\00\00failed to write whole buffer\c4\0f\10\00\1c\00\00\00\17\00\00\00\00\00\00\00\02\00\00\00\e0\0f\10\00stack backtrace:\0anote: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0a\00\00\00\00\00\00\00\08\00\00\00\04\00\00\009\00\00\00\17\04\10\00T\00\00\00N\00\00\006\00\00\00strerror_r failure\00\00\17\04\10\00T\00\00\00L\00\00\00\0d\00\00\00failed to generate unique thread ID: bitspace exhausted\00\a1\05\10\00L\00\00\00&\00\00\00\0d\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00:\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00;\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00<\00\00\00Utf8Errorvalid_up_toerror_lenNoneSome\00\00\00b\03\10\00]\00\00\00\16\01\00\00)\00\00\00\1e\00\00\00\0c\00\00\00\04\00\00\00=\00\00\00\df\02\10\00K\00\00\00\5c\03\00\00\14\00\00\00\06\05\10\00I\00\00\00Y\07\00\00$\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\0e\00\00\00\0e\00\00\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00\00\00\0d\00\00\00\0b\00\00\00\0b\00\00\00\13\00\00\00\fb\0a\10\00\0b\0b\10\00\1c\0b\10\00.\0b\10\00>\0b\10\00N\0b\10\00a\0b\10\00s\0b\10\00\80\0b\10\00\8e\0b\10\00\a3\0b\10\00\af\0b\10\00\ba\0b\10\00\cf\0b\10\00\e4\0b\10\00\f3\0b\10\00\01\0c\10\00\14\0c\10\00:\0c\10\00r\0c\10\00\8b\0c\10\00\a2\0c\10\00\ae\0c\10\00\b7\0c\10\00\c1\0c\10\00\d1\0c\10\00\e8\0c\10\00\f6\0c\10\00\04\0d\10\00\11\0d\10\00%\0d\10\00-\0d\10\00H\0d\10\00V\0d\10\00f\0d\10\00|\0d\10\00\91\0d\10\00\9c\0d\10\00\b2\0d\10\00\bf\0d\10\00\ca\0d\10\00\d5\0d\10\00Success\00Illegal byte sequence\00Domain error\00Result not representable\00Not a tty\00Permission denied\00Operation not permitted\00No such file or directory\00No such process\00File exists\00Value too large for data type\00No space left on device\00Out of memory\00Resource busy\00Interrupted system call\00Resource temporarily unavailable\00Invalid seek\00Cross-device link\00Read-only file system\00Directory not empty\00Connection reset by peer\00Operation timed out\00Connection refused\00Host is unreachable\00Address in use\00Broken pipe\00I/O error\00No such device or address\00No such device\00Not a directory\00Is a directory\00Text file busy\00Exec format error\00Invalid argument\00Argument list too long\00Symbolic link loop\00Filename too long\00Too many open files in system\00No file descriptors available\00Bad file descriptor\00No child process\00Bad address\00File too large\00Too many links\00No locks available\00Resource deadlock would occur\00State not recoverable\00Previous owner died\00Operation canceled\00Function not implemented\00No message of desired type\00Identifier removed\00Link has been severed\00Protocol error\00Bad message\00Not a socket\00Destination address required\00Message too large\00Protocol wrong type for socket\00Protocol not available\00Protocol not supported\00Not supported\00Address family not supported by protocol\00Address not available\00Network is down\00Network unreachable\00Connection reset by network\00Connection aborted\00No buffer space available\00Socket is connected\00Socket not connected\00Operation already in progress\00Operation in progress\00Stale file handle\00Quota exceeded\00Multihop attempted\00Capabilities insufficient\00\00\00\00\00\00\00\00\00\00\00u\02N\00\d6\01\e2\04\b9\04\18\01\8e\05\ed\02\16\04\f2\00\97\03\01\038\05\af\01\82\01O\03/\04\1e\00\d4\05\a2\00\12\03\1e\03\c2\01\de\03\08\00\ac\05\00\01d\02\f1\01e\054\02\8c\02\cf\02-\03L\04\e3\05\9f\02\f8\04\1c\05\08\05\b1\02K\05\15\02x\00R\02<\03\f1\03\e4\00\c3\03}\04\cc\00\aa\03y\05$\02n\01m\03\22\04\ab\04D\00\fb\01\ae\00\83\03`\00\e5\01\07\04\94\04^\04+\00X\019\01\92\00\c2\05\9b\01C\02F\01\f6\05capacity overflow\00P\05\10\00P\00\00\00\1c\00\00\00\05\00\00\000123456789abcdef { , :  {\0a,\0a((\0a}), }\00\00\00\00\0c\00\00\00\04\00\00\00B\00\00\00C\00\00\00D\00\00\00called `Option::unwrap()` on a `None` value==!=matches00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00\00\00\8e\02\10\00P\00\00\00\a0\00\00\00\09\00\00\00\8e\02\10\00P\00\00\00\84\00\00\00\1e\00\00\00    0x0123456789ABCDEFfalsetrueRefCell already borrowed\00\13\1a\10\00\15\1a\10\00\17\1a\10\00\02\00\00\00\02\00\00\00\07\00\00\00")
  (data $.data (;1;) (i32.const 1055832) "\01\00\00\00\ff\ff\ff\ffG\06\10\00\00\00\02\00")
  (@custom "\u{8}l" (after data) "anguage\02\03C11\00\04Rust\00\0cprocessed-by\02\05clang_21.1.4-wasi-sdk (https://github.com/llvm/llvm-project 222fc11f2b8f25f6a0f4976272ef1bb7bf49521d)\05rustc\1d1.95.0 (59807616e 2026-04-14)")
  (@custom "target_features" (after data) "\09+\0bbulk-memory+\0fbulk-memory-opt+\16call-indirect-overlong+\0eextended-const+\0amultivalue+\0fmutable-globals+\13nontrapping-fptoint+\0freference-types+\08sign-ext")
  (@producers
    (processed-by "wasi-virt-layer" "0.4.12")
  )
)
