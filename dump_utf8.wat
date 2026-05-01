(module
  (type (;0;) (func))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32 i32 i32) (result i32)))
  (type (;8;) (func (param i32 i32 i32 i32)))
  (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32)))
  (import "wasip1-vfs:host/virtual-file-system-wasip1-core" "[static]wasip1.fd-write-import" (func (;0;) (type 9)))
  (import "wasip1-vfs:host/virtual-file-system-wasip1-core" "[static]wasip1.proc-exit-import" (func (;1;) (type 2)))
  (table (;0;) 54 54 funcref)
  (memory (;0;) 19)
  (global (;0;) (mut i32) i32.const 1048576)
  (global (;1;) (mut i32) i32.const 1114112)
  (global (;2;) (mut i32) i32.const 1179648)
  (export "memory" (memory 0))
  (export "__wasip1_vfs_c_target_proc_exit" (func 69))
  (export "main" (func 40))
  (export "__wasip1_vfs_self_proc_exit" (func 69))
  (export "cabi_realloc" (func 76))
  (export "cabi_realloc_wit_bindgen_0_54_0" (func 58))
  (export "__wasip1_vfs_c_target__start" (func 30))
  (start 87)
  (elem (;0;) (i32.const 1) func 86 76 20 73 39 67 19 23 70 14 59 49 29 16 77 43 21 78 22 31 79 72 47 24 80 75 54 74 81 63 57 33 38 83 53 48 85 82 34 11 72 41 17 85 82 44 18 71 65 64 67 62 73)
  (func (;2;) (type 3) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 10
    global.set 0
    i32.const 1052716
    i32.load
    local.tee 7
    i32.eqz
    if ;; label = @1
      i32.const 1053164
      i32.load
      local.tee 3
      i32.eqz
      if ;; label = @2
        i32.const 1053176
        i64.const -1
        i64.store align=4
        i32.const 1053168
        i64.const 281474976776192
        i64.store align=4
        i32.const 1053164
        local.get 10
        i32.const 8
        i32.add
        i32.const -16
        i32.and
        i32.const 1431655768
        i32.xor
        local.tee 3
        i32.store
        i32.const 1053184
        i32.const 0
        i32.store
        i32.const 1053136
        i32.const 0
        i32.store
      else
      end
      i32.const 1053140
      i32.const 1053200
      i32.store
      i32.const 1052708
      i32.const 1053200
      i32.store
      i32.const 1052728
      local.get 3
      i32.store
      i32.const 1052724
      i32.const -1
      i32.store
      i32.const 1053144
      i32.const 60912
      i32.store
      i32.const 1053128
      i32.const 60912
      i32.store
      i32.const 1053124
      i32.const 60912
      i32.store
      loop ;; label = @2
        local.get 1
        i32.const 1052752
        i32.add
        local.get 1
        i32.const 1052740
        i32.add
        local.tee 2
        i32.store
        local.get 2
        local.get 1
        i32.const 1052732
        i32.add
        local.tee 5
        i32.store
        local.get 1
        i32.const 1052744
        i32.add
        local.get 5
        i32.store
        local.get 1
        i32.const 1052760
        i32.add
        local.get 1
        i32.const 1052748
        i32.add
        local.tee 5
        i32.store
        local.get 5
        local.get 2
        i32.store
        local.get 1
        i32.const 1052768
        i32.add
        local.get 1
        i32.const 1052756
        i32.add
        local.tee 2
        i32.store
        local.get 2
        local.get 5
        i32.store
        local.get 1
        i32.const 1052764
        i32.add
        local.get 2
        i32.store
        local.get 1
        i32.const 32
        i32.add
        local.tee 1
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 1114060
      i32.const 56
      i32.store
      i32.const 1052720
      i32.const 1053180
      i32.load
      i32.store
      i32.const 1052716
      i32.const 1053208
      local.tee 7
      i32.store
      i32.const 1052704
      i32.const 60848
      i32.store
      i32.const 1053212
      i32.const 60849
      i32.store
    else
    end
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
                            local.get 0
                            i32.const 236
                            i32.le_u
                            if ;; label = @13
                              i32.const 1052692
                              i32.load
                              local.tee 4
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
                              local.tee 6
                              i32.const 3
                              i32.shr_u
                              local.tee 0
                              i32.shr_u
                              local.tee 1
                              i32.const 3
                              i32.and
                              if ;; label = @14
                                block ;; label = @15
                                  local.get 1
                                  i32.const 1
                                  i32.and
                                  local.get 0
                                  i32.or
                                  i32.const 1
                                  i32.xor
                                  local.tee 2
                                  i32.const 3
                                  i32.shl
                                  local.tee 0
                                  i32.const 1052732
                                  i32.add
                                  local.tee 1
                                  local.get 0
                                  i32.load offset=1052740
                                  local.tee 0
                                  i32.load offset=8
                                  local.tee 5
                                  i32.eq
                                  if ;; label = @16
                                    i32.const 1052692
                                    local.get 4
                                    i32.const -2
                                    local.get 2
                                    i32.rotl
                                    i32.and
                                    i32.store
                                    br 1 (;@15;)
                                  else
                                  end
                                  local.get 1
                                  local.get 5
                                  i32.store offset=8
                                  local.get 5
                                  local.get 1
                                  i32.store offset=12
                                end
                                local.get 0
                                i32.const 8
                                i32.add
                                local.set 1
                                local.get 0
                                local.get 2
                                i32.const 3
                                i32.shl
                                local.tee 2
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 0
                                local.get 2
                                i32.add
                                local.tee 0
                                local.get 0
                                i32.load offset=4
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                br 13 (;@1;)
                              else
                              end
                              local.get 6
                              i32.const 1052700
                              i32.load
                              local.tee 8
                              i32.le_u
                              br_if 1 (;@12;)
                              local.get 1
                              if ;; label = @14
                                block ;; label = @15
                                  i32.const 2
                                  local.get 0
                                  i32.shl
                                  local.tee 2
                                  i32.const 0
                                  local.get 2
                                  i32.sub
                                  i32.or
                                  local.get 1
                                  local.get 0
                                  i32.shl
                                  i32.and
                                  i32.ctz
                                  local.tee 1
                                  i32.const 3
                                  i32.shl
                                  local.tee 0
                                  i32.const 1052732
                                  i32.add
                                  local.tee 2
                                  local.get 0
                                  i32.load offset=1052740
                                  local.tee 0
                                  i32.load offset=8
                                  local.tee 5
                                  i32.eq
                                  if ;; label = @16
                                    i32.const 1052692
                                    local.get 4
                                    i32.const -2
                                    local.get 1
                                    i32.rotl
                                    i32.and
                                    local.tee 4
                                    i32.store
                                    br 1 (;@15;)
                                  else
                                  end
                                  local.get 2
                                  local.get 5
                                  i32.store offset=8
                                  local.get 5
                                  local.get 2
                                  i32.store offset=12
                                end
                                local.get 0
                                local.get 6
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                local.get 0
                                local.get 1
                                i32.const 3
                                i32.shl
                                local.tee 1
                                i32.add
                                local.get 1
                                local.get 6
                                i32.sub
                                local.tee 5
                                i32.store
                                local.get 0
                                local.get 6
                                i32.add
                                local.tee 3
                                local.get 5
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 8
                                if ;; label = @15
                                  local.get 8
                                  i32.const -8
                                  i32.and
                                  i32.const 1052732
                                  i32.add
                                  local.set 1
                                  i32.const 1052712
                                  i32.load
                                  local.set 2
                                  block (result i32) ;; label = @16
                                    local.get 4
                                    i32.const 1
                                    local.get 8
                                    i32.const 3
                                    i32.shr_u
                                    i32.shl
                                    local.tee 7
                                    i32.and
                                    i32.eqz
                                    if ;; label = @17
                                      i32.const 1052692
                                      local.get 4
                                      local.get 7
                                      i32.or
                                      i32.store
                                      local.get 1
                                      br 1 (;@16;)
                                    else
                                    end
                                    local.get 1
                                    i32.load offset=8
                                  end
                                  local.tee 4
                                  local.get 2
                                  i32.store offset=12
                                  local.get 1
                                  local.get 2
                                  i32.store offset=8
                                  local.get 2
                                  local.get 1
                                  i32.store offset=12
                                  local.get 2
                                  local.get 4
                                  i32.store offset=8
                                else
                                end
                                local.get 0
                                i32.const 8
                                i32.add
                                local.set 1
                                i32.const 1052712
                                local.get 3
                                i32.store
                                i32.const 1052700
                                local.get 5
                                i32.store
                                br 13 (;@1;)
                              else
                              end
                              i32.const 1052696
                              i32.load
                              local.tee 11
                              i32.eqz
                              br_if 1 (;@12;)
                              local.get 11
                              i32.ctz
                              i32.const 2
                              i32.shl
                              i32.load offset=1052996
                              local.tee 2
                              i32.load offset=4
                              i32.const -8
                              i32.and
                              local.get 6
                              i32.sub
                              local.set 3
                              local.get 2
                              local.set 0
                              loop ;; label = @14
                                block ;; label = @15
                                  local.get 0
                                  i32.load offset=16
                                  local.tee 1
                                  i32.eqz
                                  if ;; label = @16
                                    local.get 0
                                    i32.load offset=20
                                    local.tee 1
                                    i32.eqz
                                    br_if 1 (;@15;)
                                  else
                                  end
                                  local.get 1
                                  i32.load offset=4
                                  i32.const -8
                                  i32.and
                                  local.get 6
                                  i32.sub
                                  local.tee 0
                                  local.get 3
                                  local.get 0
                                  local.get 3
                                  i32.lt_u
                                  local.tee 0
                                  select
                                  local.set 3
                                  local.get 1
                                  local.get 2
                                  local.get 0
                                  select
                                  local.set 2
                                  local.get 1
                                  local.set 0
                                  br 1 (;@14;)
                                end
                              end
                              local.get 2
                              i32.load offset=24
                              local.set 9
                              local.get 2
                              local.get 2
                              i32.load offset=12
                              local.tee 1
                              i32.ne
                              if ;; label = @14
                                local.get 2
                                i32.load offset=8
                                local.tee 0
                                local.get 1
                                i32.store offset=12
                                local.get 1
                                local.get 0
                                i32.store offset=8
                                br 12 (;@2;)
                              else
                              end
                              local.get 2
                              i32.load offset=20
                              local.tee 0
                              if (result i32) ;; label = @14
                                local.get 2
                                i32.const 20
                                i32.add
                              else
                                local.get 2
                                i32.load offset=16
                                local.tee 0
                                i32.eqz
                                br_if 3 (;@11;)
                                local.get 2
                                i32.const 16
                                i32.add
                              end
                              local.set 5
                              loop ;; label = @14
                                local.get 5
                                local.set 7
                                local.get 0
                                local.tee 1
                                i32.const 20
                                i32.add
                                local.set 5
                                local.get 1
                                i32.load offset=20
                                local.tee 0
                                br_if 0 (;@14;)
                                local.get 1
                                i32.const 16
                                i32.add
                                local.set 5
                                local.get 1
                                i32.load offset=16
                                local.tee 0
                                br_if 0 (;@14;)
                              end
                              local.get 7
                              i32.const 0
                              i32.store
                              br 11 (;@2;)
                            else
                            end
                            i32.const -1
                            local.set 6
                            local.get 0
                            i32.const -65
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 19
                            i32.add
                            local.tee 1
                            i32.const -16
                            i32.and
                            local.set 6
                            i32.const 1052696
                            i32.load
                            local.tee 8
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 31
                            local.set 9
                            i32.const 0
                            local.get 6
                            i32.sub
                            local.set 3
                            local.get 0
                            i32.const 16777196
                            i32.le_u
                            if ;; label = @13
                              local.get 6
                              i32.const 38
                              local.get 1
                              i32.const 8
                              i32.shr_u
                              i32.clz
                              local.tee 0
                              i32.sub
                              i32.shr_u
                              i32.const 1
                              i32.and
                              local.get 0
                              i32.const 1
                              i32.shl
                              i32.sub
                              i32.const 62
                              i32.add
                              local.set 9
                            else
                            end
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 9
                                  i32.const 2
                                  i32.shl
                                  i32.load offset=1052996
                                  local.tee 0
                                  i32.eqz
                                  if ;; label = @16
                                    i32.const 0
                                    local.set 1
                                    i32.const 0
                                    local.set 5
                                    br 1 (;@15;)
                                  else
                                  end
                                  i32.const 0
                                  local.set 1
                                  local.get 6
                                  i32.const 25
                                  local.get 9
                                  i32.const 1
                                  i32.shr_u
                                  i32.sub
                                  i32.const 0
                                  local.get 9
                                  i32.const 31
                                  i32.ne
                                  select
                                  i32.shl
                                  local.set 2
                                  i32.const 0
                                  local.set 5
                                  loop ;; label = @16
                                    block ;; label = @17
                                      local.get 0
                                      i32.load offset=4
                                      i32.const -8
                                      i32.and
                                      local.get 6
                                      i32.sub
                                      local.tee 4
                                      local.get 3
                                      i32.ge_u
                                      br_if 0 (;@17;)
                                      local.get 0
                                      local.set 5
                                      local.get 4
                                      local.tee 3
                                      br_if 0 (;@17;)
                                      i32.const 0
                                      local.set 3
                                      local.get 0
                                      local.set 1
                                      br 3 (;@14;)
                                    end
                                    local.get 1
                                    local.get 0
                                    i32.load offset=20
                                    local.tee 4
                                    local.get 4
                                    local.get 0
                                    local.get 2
                                    i32.const 29
                                    i32.shr_u
                                    i32.const 4
                                    i32.and
                                    i32.add
                                    i32.load offset=16
                                    local.tee 0
                                    i32.eq
                                    select
                                    local.get 1
                                    local.get 4
                                    select
                                    local.set 1
                                    local.get 2
                                    i32.const 1
                                    i32.shl
                                    local.set 2
                                    local.get 0
                                    br_if 0 (;@16;)
                                  end
                                end
                                local.get 1
                                local.get 5
                                i32.or
                                i32.eqz
                                if ;; label = @15
                                  i32.const 0
                                  local.set 5
                                  i32.const 2
                                  local.get 9
                                  i32.shl
                                  local.tee 0
                                  i32.const 0
                                  local.get 0
                                  i32.sub
                                  i32.or
                                  local.get 8
                                  i32.and
                                  local.tee 0
                                  i32.eqz
                                  br_if 3 (;@12;)
                                  local.get 0
                                  i32.ctz
                                  i32.const 2
                                  i32.shl
                                  i32.load offset=1052996
                                  local.set 1
                                else
                                end
                                local.get 1
                                i32.eqz
                                br_if 1 (;@13;)
                              end
                              loop ;; label = @14
                                local.get 1
                                i32.load offset=4
                                i32.const -8
                                i32.and
                                local.get 6
                                i32.sub
                                local.tee 2
                                local.get 3
                                i32.lt_u
                                local.set 0
                                local.get 2
                                local.get 3
                                local.get 0
                                select
                                local.set 3
                                local.get 1
                                local.get 5
                                local.get 0
                                select
                                local.set 5
                                local.get 1
                                i32.load offset=16
                                local.tee 0
                                if (result i32) ;; label = @15
                                  local.get 0
                                else
                                  local.get 1
                                  i32.load offset=20
                                end
                                local.tee 1
                                br_if 0 (;@14;)
                              end
                            end
                            local.get 5
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 3
                            i32.const 1052700
                            i32.load
                            local.get 6
                            i32.sub
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 5
                            i32.load offset=24
                            local.set 7
                            local.get 5
                            local.get 5
                            i32.load offset=12
                            local.tee 1
                            i32.ne
                            if ;; label = @13
                              local.get 5
                              i32.load offset=8
                              local.tee 0
                              local.get 1
                              i32.store offset=12
                              local.get 1
                              local.get 0
                              i32.store offset=8
                              br 10 (;@3;)
                            else
                            end
                            local.get 5
                            i32.load offset=20
                            local.tee 0
                            if (result i32) ;; label = @13
                              local.get 5
                              i32.const 20
                              i32.add
                            else
                              local.get 5
                              i32.load offset=16
                              local.tee 0
                              i32.eqz
                              br_if 3 (;@10;)
                              local.get 5
                              i32.const 16
                              i32.add
                            end
                            local.set 2
                            loop ;; label = @13
                              local.get 2
                              local.set 4
                              local.get 0
                              local.tee 1
                              i32.const 20
                              i32.add
                              local.set 2
                              local.get 1
                              i32.load offset=20
                              local.tee 0
                              br_if 0 (;@13;)
                              local.get 1
                              i32.const 16
                              i32.add
                              local.set 2
                              local.get 1
                              i32.load offset=16
                              local.tee 0
                              br_if 0 (;@13;)
                            end
                            local.get 4
                            i32.const 0
                            i32.store
                            br 9 (;@3;)
                          end
                          local.get 6
                          i32.const 1052700
                          i32.load
                          local.tee 5
                          i32.le_u
                          if ;; label = @12
                            i32.const 1052712
                            i32.load
                            local.set 1
                            block ;; label = @13
                              local.get 5
                              local.get 6
                              i32.sub
                              local.tee 0
                              i32.const 16
                              i32.ge_u
                              if ;; label = @14
                                local.get 1
                                local.get 6
                                i32.add
                                local.tee 2
                                local.get 0
                                i32.const 1
                                i32.or
                                i32.store offset=4
                                local.get 1
                                local.get 5
                                i32.add
                                local.get 0
                                i32.store
                                local.get 1
                                local.get 6
                                i32.const 3
                                i32.or
                                i32.store offset=4
                                br 1 (;@13;)
                              else
                              end
                              local.get 1
                              local.get 5
                              i32.const 3
                              i32.or
                              i32.store offset=4
                              local.get 1
                              local.get 5
                              i32.add
                              local.tee 0
                              local.get 0
                              i32.load offset=4
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              i32.const 0
                              local.set 2
                              i32.const 0
                              local.set 0
                            end
                            i32.const 1052700
                            local.get 0
                            i32.store
                            i32.const 1052712
                            local.get 2
                            i32.store
                            local.get 1
                            i32.const 8
                            i32.add
                            local.set 1
                            br 11 (;@1;)
                          else
                          end
                          local.get 6
                          i32.const 1052704
                          i32.load
                          local.tee 2
                          i32.lt_u
                          if ;; label = @12
                            local.get 6
                            local.get 7
                            i32.add
                            local.tee 0
                            local.get 2
                            local.get 6
                            i32.sub
                            local.tee 1
                            i32.const 1
                            i32.or
                            i32.store offset=4
                            i32.const 1052716
                            local.get 0
                            i32.store
                            i32.const 1052704
                            local.get 1
                            i32.store
                            local.get 7
                            local.get 6
                            i32.const 3
                            i32.or
                            i32.store offset=4
                            local.get 7
                            i32.const 8
                            i32.add
                            local.set 1
                            br 11 (;@1;)
                          else
                          end
                          i32.const 0
                          local.set 1
                          local.get 6
                          local.get 6
                          i32.const 71
                          i32.add
                          local.tee 5
                          block (result i32) ;; label = @12
                            i32.const 1053164
                            i32.load
                            if ;; label = @13
                              i32.const 1053172
                              i32.load
                              br 1 (;@12;)
                            else
                            end
                            i32.const 1053176
                            i64.const -1
                            i64.store align=4
                            i32.const 1053168
                            i64.const 281474976776192
                            i64.store align=4
                            i32.const 1053164
                            local.get 10
                            i32.const 12
                            i32.add
                            i32.const -16
                            i32.and
                            i32.const 1431655768
                            i32.xor
                            i32.store
                            i32.const 1053184
                            i32.const 0
                            i32.store
                            i32.const 1053136
                            i32.const 0
                            i32.store
                            i32.const 65536
                          end
                          local.tee 0
                          i32.add
                          local.tee 4
                          i32.const 0
                          local.get 0
                          i32.sub
                          local.tee 3
                          i32.and
                          local.tee 0
                          i32.ge_u
                          if ;; label = @12
                            i32.const 1052684
                            i32.const 48
                            i32.store
                            br 11 (;@1;)
                          else
                          end
                          block ;; label = @12
                            i32.const 1053132
                            i32.load
                            local.tee 1
                            i32.eqz
                            br_if 0 (;@12;)
                            i32.const 1053124
                            i32.load
                            local.tee 8
                            local.get 0
                            i32.add
                            local.tee 9
                            local.get 8
                            i32.gt_u
                            local.get 1
                            local.get 9
                            i32.ge_u
                            i32.and
                            br_if 0 (;@12;)
                            i32.const 0
                            local.set 1
                            i32.const 1052684
                            i32.const 48
                            i32.store
                            br 11 (;@1;)
                          end
                          i32.const 1053136
                          i32.load8_u
                          i32.const 4
                          i32.and
                          br_if 4 (;@7;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 7
                              if ;; label = @14
                                i32.const 1053140
                                local.set 1
                                loop ;; label = @15
                                  local.get 1
                                  i32.load
                                  local.tee 8
                                  local.get 7
                                  i32.le_u
                                  if ;; label = @16
                                    local.get 7
                                    local.get 8
                                    local.get 1
                                    i32.load offset=4
                                    i32.add
                                    i32.lt_u
                                    br_if 3 (;@13;)
                                  else
                                  end
                                  local.get 1
                                  i32.load offset=8
                                  local.tee 1
                                  br_if 0 (;@15;)
                                end
                              else
                              end
                              i32.const 0
                              call 37
                              local.tee 2
                              i32.const -1
                              i32.eq
                              br_if 5 (;@8;)
                              local.get 0
                              local.set 4
                              i32.const 1053168
                              i32.load
                              local.tee 1
                              i32.const 1
                              i32.sub
                              local.tee 3
                              local.get 2
                              i32.and
                              if ;; label = @14
                                local.get 0
                                local.get 2
                                i32.sub
                                local.get 2
                                local.get 3
                                i32.add
                                i32.const 0
                                local.get 1
                                i32.sub
                                i32.and
                                i32.add
                                local.set 4
                              else
                              end
                              local.get 4
                              local.get 6
                              i32.le_u
                              local.get 4
                              i32.const 2147483646
                              i32.gt_u
                              i32.or
                              br_if 5 (;@8;)
                              i32.const 1053132
                              i32.load
                              local.tee 1
                              if ;; label = @14
                                i32.const 1053124
                                i32.load
                                local.tee 3
                                local.get 4
                                i32.add
                                local.tee 7
                                local.get 3
                                i32.le_u
                                local.get 1
                                local.get 7
                                i32.lt_u
                                i32.or
                                br_if 6 (;@8;)
                              else
                              end
                              local.get 4
                              call 37
                              local.tee 1
                              local.get 2
                              i32.ne
                              br_if 1 (;@12;)
                              br 7 (;@6;)
                            end
                            local.get 4
                            local.get 2
                            i32.sub
                            local.get 3
                            i32.and
                            local.tee 4
                            i32.const 2147483646
                            i32.gt_u
                            br_if 4 (;@8;)
                            local.get 4
                            call 37
                            local.tee 2
                            local.get 1
                            i32.load
                            local.get 1
                            i32.load offset=4
                            i32.add
                            i32.eq
                            br_if 3 (;@9;)
                            local.get 2
                            local.set 1
                          end
                          local.get 1
                          i32.const -1
                          i32.eq
                          local.get 4
                          local.get 6
                          i32.const 72
                          i32.add
                          i32.ge_u
                          i32.or
                          i32.eqz
                          if ;; label = @12
                            i32.const 1053172
                            i32.load
                            local.tee 2
                            local.get 5
                            local.get 4
                            i32.sub
                            i32.add
                            i32.const 0
                            local.get 2
                            i32.sub
                            i32.and
                            local.tee 2
                            i32.const 2147483646
                            i32.gt_u
                            if ;; label = @13
                              local.get 1
                              local.set 2
                              br 7 (;@6;)
                            else
                            end
                            local.get 2
                            call 37
                            i32.const -1
                            i32.ne
                            if ;; label = @13
                              local.get 2
                              local.get 4
                              i32.add
                              local.set 4
                              local.get 1
                              local.set 2
                              br 7 (;@6;)
                            else
                            end
                            i32.const 0
                            local.get 4
                            i32.sub
                            call 37
                            drop
                            br 4 (;@8;)
                          else
                          end
                          local.get 1
                          local.tee 2
                          i32.const -1
                          i32.ne
                          br_if 5 (;@6;)
                          br 3 (;@8;)
                        end
                        i32.const 0
                        local.set 1
                        br 8 (;@2;)
                      end
                      i32.const 0
                      local.set 1
                      br 6 (;@3;)
                    end
                    local.get 2
                    i32.const -1
                    i32.ne
                    br_if 2 (;@6;)
                  end
                  i32.const 1053136
                  i32.const 1053136
                  i32.load
                  i32.const 4
                  i32.or
                  i32.store
                end
                local.get 0
                i32.const 2147483646
                i32.gt_u
                br_if 1 (;@5;)
                local.get 0
                call 37
                local.tee 2
                i32.const -1
                i32.eq
                i32.const 0
                call 37
                local.tee 0
                i32.const -1
                i32.eq
                i32.or
                local.get 0
                local.get 2
                i32.le_u
                i32.or
                br_if 1 (;@5;)
                local.get 0
                local.get 2
                i32.sub
                local.tee 4
                local.get 6
                i32.const 56
                i32.add
                i32.le_u
                br_if 1 (;@5;)
              end
              i32.const 1053124
              i32.const 1053124
              i32.load
              local.get 4
              i32.add
              local.tee 0
              i32.store
              i32.const 1053128
              i32.load
              local.get 0
              i32.lt_u
              if ;; label = @6
                i32.const 1053128
                local.get 0
                i32.store
              else
              end
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    i32.const 1052716
                    i32.load
                    local.tee 3
                    if ;; label = @9
                      i32.const 1053140
                      local.set 1
                      loop ;; label = @10
                        local.get 2
                        local.get 1
                        i32.load
                        local.tee 0
                        local.get 1
                        i32.load offset=4
                        local.tee 5
                        i32.add
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 1
                        i32.load offset=8
                        local.tee 1
                        br_if 0 (;@10;)
                      end
                      br 2 (;@7;)
                    else
                    end
                    i32.const 1052708
                    i32.load
                    local.tee 0
                    i32.const 0
                    local.get 0
                    local.get 2
                    i32.le_u
                    select
                    i32.eqz
                    if ;; label = @9
                      i32.const 1052708
                      local.get 2
                      i32.store
                    else
                    end
                    i32.const 0
                    local.set 1
                    i32.const 1053144
                    local.get 4
                    i32.store
                    i32.const 1053140
                    local.get 2
                    i32.store
                    i32.const 1052724
                    i32.const -1
                    i32.store
                    i32.const 1052728
                    i32.const 1053164
                    i32.load
                    i32.store
                    i32.const 1053152
                    i32.const 0
                    i32.store
                    loop ;; label = @9
                      local.get 1
                      i32.const 1052752
                      i32.add
                      local.get 1
                      i32.const 1052740
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 0
                      local.get 1
                      i32.const 1052732
                      i32.add
                      local.tee 5
                      i32.store
                      local.get 1
                      i32.const 1052744
                      i32.add
                      local.get 5
                      i32.store
                      local.get 1
                      i32.const 1052760
                      i32.add
                      local.get 1
                      i32.const 1052748
                      i32.add
                      local.tee 5
                      i32.store
                      local.get 5
                      local.get 0
                      i32.store
                      local.get 1
                      i32.const 1052768
                      i32.add
                      local.get 1
                      i32.const 1052756
                      i32.add
                      local.tee 0
                      i32.store
                      local.get 0
                      local.get 5
                      i32.store
                      local.get 1
                      i32.const 1052764
                      i32.add
                      local.get 0
                      i32.store
                      local.get 1
                      i32.const 32
                      i32.add
                      local.tee 1
                      i32.const 256
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 2
                    i32.const -8
                    local.get 2
                    i32.sub
                    i32.const 15
                    i32.and
                    local.tee 0
                    i32.add
                    local.tee 1
                    local.get 4
                    i32.const 56
                    i32.sub
                    local.tee 5
                    local.get 0
                    i32.sub
                    local.tee 0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    i32.const 1052720
                    i32.const 1053180
                    i32.load
                    i32.store
                    i32.const 1052704
                    local.get 0
                    i32.store
                    i32.const 1052716
                    local.get 1
                    i32.store
                    local.get 2
                    local.get 5
                    i32.add
                    i32.const 56
                    i32.store offset=4
                    br 2 (;@6;)
                  end
                  local.get 2
                  local.get 3
                  i32.le_u
                  local.get 0
                  local.get 3
                  i32.gt_u
                  i32.or
                  br_if 0 (;@7;)
                  local.get 1
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
                  local.tee 2
                  i32.const 1052704
                  i32.load
                  local.get 4
                  i32.add
                  local.tee 7
                  local.get 0
                  i32.sub
                  local.tee 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 1
                  local.get 4
                  local.get 5
                  i32.add
                  i32.store offset=4
                  i32.const 1052720
                  i32.const 1053180
                  i32.load
                  i32.store
                  i32.const 1052704
                  local.get 0
                  i32.store
                  i32.const 1052716
                  local.get 2
                  i32.store
                  local.get 3
                  local.get 7
                  i32.add
                  i32.const 56
                  i32.store offset=4
                  br 1 (;@6;)
                end
                i32.const 1052708
                i32.load
                local.get 2
                i32.gt_u
                if ;; label = @7
                  i32.const 1052708
                  local.get 2
                  i32.store
                else
                end
                local.get 2
                local.get 4
                i32.add
                local.set 5
                i32.const 1053140
                local.set 1
                block ;; label = @7
                  loop ;; label = @8
                    local.get 5
                    local.get 1
                    i32.load
                    local.tee 0
                    i32.ne
                    if ;; label = @9
                      local.get 1
                      i32.load offset=8
                      local.tee 1
                      br_if 1 (;@8;)
                      br 2 (;@7;)
                    else
                    end
                  end
                  local.get 1
                  i32.load8_u offset=12
                  i32.const 8
                  i32.and
                  i32.eqz
                  br_if 3 (;@4;)
                end
                i32.const 1053140
                local.set 1
                loop ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load
                    local.tee 0
                    local.get 3
                    i32.le_u
                    if ;; label = @9
                      local.get 3
                      local.get 0
                      local.get 1
                      i32.load offset=4
                      i32.add
                      local.tee 5
                      i32.lt_u
                      br_if 1 (;@8;)
                    else
                    end
                    local.get 1
                    i32.load offset=8
                    local.set 1
                    br 1 (;@7;)
                  end
                end
                local.get 2
                i32.const -8
                local.get 2
                i32.sub
                i32.const 15
                i32.and
                local.tee 0
                i32.add
                local.tee 1
                local.get 4
                i32.const 56
                i32.sub
                local.tee 7
                local.get 0
                i32.sub
                local.tee 8
                i32.const 1
                i32.or
                i32.store offset=4
                local.get 2
                local.get 7
                i32.add
                i32.const 56
                i32.store offset=4
                local.get 3
                local.get 5
                i32.const 55
                local.get 5
                i32.sub
                i32.const 15
                i32.and
                i32.add
                i32.const 63
                i32.sub
                local.tee 0
                local.get 0
                local.get 3
                i32.const 16
                i32.add
                i32.lt_u
                select
                local.tee 0
                i32.const 35
                i32.store offset=4
                i32.const 1052720
                i32.const 1053180
                i32.load
                i32.store
                i32.const 1052704
                local.get 8
                i32.store
                i32.const 1052716
                local.get 1
                i32.store
                local.get 0
                i32.const 16
                i32.add
                i32.const 1053148
                i64.load align=4
                i64.store align=4
                local.get 0
                i32.const 1053140
                i64.load align=4
                i64.store offset=8 align=4
                i32.const 1053148
                local.get 0
                i32.const 8
                i32.add
                i32.store
                i32.const 1053144
                local.get 4
                i32.store
                i32.const 1053140
                local.get 2
                i32.store
                i32.const 1053152
                i32.const 0
                i32.store
                local.get 0
                i32.const 36
                i32.add
                local.set 1
                loop ;; label = @7
                  local.get 1
                  i32.const 7
                  i32.store
                  local.get 1
                  i32.const 4
                  i32.add
                  local.tee 1
                  local.get 5
                  i32.lt_u
                  br_if 0 (;@7;)
                end
                local.get 0
                local.get 3
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                local.get 0
                i32.load offset=4
                i32.const -2
                i32.and
                i32.store offset=4
                local.get 0
                local.get 0
                local.get 3
                i32.sub
                local.tee 2
                i32.store
                local.get 3
                local.get 2
                i32.const 1
                i32.or
                i32.store offset=4
                block (result i32) ;; label = @7
                  local.get 2
                  i32.const 255
                  i32.le_u
                  if ;; label = @8
                    local.get 2
                    i32.const -8
                    i32.and
                    i32.const 1052732
                    i32.add
                    local.set 1
                    block (result i32) ;; label = @9
                      i32.const 1052692
                      i32.load
                      local.tee 0
                      i32.const 1
                      local.get 2
                      i32.const 3
                      i32.shr_u
                      i32.shl
                      local.tee 2
                      i32.and
                      i32.eqz
                      if ;; label = @10
                        i32.const 1052692
                        local.get 0
                        local.get 2
                        i32.or
                        i32.store
                        local.get 1
                        br 1 (;@9;)
                      else
                      end
                      local.get 1
                      i32.load offset=8
                    end
                    local.tee 0
                    local.get 3
                    i32.store offset=12
                    local.get 1
                    local.get 3
                    i32.store offset=8
                    i32.const 8
                    local.set 5
                    i32.const 12
                    br 1 (;@7;)
                  else
                  end
                  i32.const 31
                  local.set 1
                  local.get 2
                  i32.const 16777215
                  i32.le_u
                  if ;; label = @8
                    local.get 2
                    i32.const 38
                    local.get 2
                    i32.const 8
                    i32.shr_u
                    i32.clz
                    local.tee 0
                    i32.sub
                    i32.shr_u
                    i32.const 1
                    i32.and
                    local.get 0
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 62
                    i32.add
                    local.set 1
                  else
                  end
                  local.get 3
                  local.get 1
                  i32.store offset=28
                  local.get 3
                  i64.const 0
                  i64.store offset=16 align=4
                  local.get 1
                  i32.const 2
                  i32.shl
                  i32.const 1052996
                  i32.add
                  local.set 0
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 1052696
                      i32.load
                      local.tee 5
                      i32.const 1
                      local.get 1
                      i32.shl
                      local.tee 4
                      i32.and
                      i32.eqz
                      if ;; label = @10
                        local.get 0
                        local.get 3
                        i32.store
                        i32.const 1052696
                        local.get 4
                        local.get 5
                        i32.or
                        i32.store
                        br 1 (;@9;)
                      else
                      end
                      local.get 2
                      i32.const 25
                      local.get 1
                      i32.const 1
                      i32.shr_u
                      i32.sub
                      i32.const 0
                      local.get 1
                      i32.const 31
                      i32.ne
                      select
                      i32.shl
                      local.set 1
                      local.get 0
                      i32.load
                      local.set 5
                      loop ;; label = @10
                        local.get 5
                        local.tee 0
                        i32.load offset=4
                        i32.const -8
                        i32.and
                        local.get 2
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 1
                        i32.const 29
                        i32.shr_u
                        local.set 5
                        local.get 1
                        i32.const 1
                        i32.shl
                        local.set 1
                        local.get 0
                        local.get 5
                        i32.const 4
                        i32.and
                        i32.add
                        local.tee 4
                        i32.load offset=16
                        local.tee 5
                        br_if 0 (;@10;)
                      end
                      local.get 4
                      i32.const 16
                      i32.add
                      local.get 3
                      i32.store
                    end
                    local.get 3
                    local.get 0
                    i32.store offset=24
                    i32.const 12
                    local.set 5
                    local.get 3
                    local.tee 0
                    local.set 1
                    i32.const 8
                    br 1 (;@7;)
                  end
                  local.get 0
                  i32.load offset=8
                  local.set 1
                  local.get 0
                  local.get 3
                  i32.store offset=8
                  local.get 1
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 1
                  i32.store offset=8
                  i32.const 0
                  local.set 1
                  i32.const 12
                  local.set 5
                  i32.const 24
                end
                local.get 3
                local.get 5
                i32.add
                local.get 0
                i32.store
                local.get 3
                i32.add
                local.get 1
                i32.store
              end
              i32.const 1052704
              i32.load
              local.tee 1
              local.get 6
              i32.le_u
              br_if 0 (;@5;)
              i32.const 1052716
              i32.load
              local.tee 0
              local.get 6
              i32.add
              local.tee 2
              local.get 1
              local.get 6
              i32.sub
              local.tee 1
              i32.const 1
              i32.or
              i32.store offset=4
              i32.const 1052704
              local.get 1
              i32.store
              i32.const 1052716
              local.get 2
              i32.store
              local.get 0
              local.get 6
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 0
              i32.const 8
              i32.add
              local.set 1
              br 4 (;@1;)
            end
            i32.const 0
            local.set 1
            i32.const 1052684
            i32.const 48
            i32.store
            br 3 (;@1;)
          end
          local.get 1
          local.get 2
          i32.store
          local.get 1
          local.get 1
          i32.load offset=4
          local.get 4
          i32.add
          i32.store offset=4
          local.get 2
          i32.const -8
          local.get 2
          i32.sub
          i32.const 15
          i32.and
          i32.add
          local.tee 8
          local.get 6
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          i32.const -8
          local.get 0
          i32.sub
          i32.const 15
          i32.and
          i32.add
          local.tee 4
          local.get 6
          local.get 8
          i32.add
          local.tee 3
          i32.sub
          local.set 7
          block ;; label = @4
            i32.const 1052716
            i32.load
            local.get 4
            i32.eq
            if ;; label = @5
              i32.const 1052716
              local.get 3
              i32.store
              i32.const 1052704
              i32.const 1052704
              i32.load
              local.get 7
              i32.add
              local.tee 0
              i32.store
              local.get 3
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              br 1 (;@4;)
            else
            end
            i32.const 1052712
            i32.load
            local.get 4
            i32.eq
            if ;; label = @5
              i32.const 1052712
              local.get 3
              i32.store
              i32.const 1052700
              i32.const 1052700
              i32.load
              local.get 7
              i32.add
              local.tee 0
              i32.store
              local.get 3
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              local.get 3
              i32.add
              local.get 0
              i32.store
              br 1 (;@4;)
            else
            end
            local.get 4
            i32.load offset=4
            local.tee 2
            i32.const 3
            i32.and
            i32.const 1
            i32.eq
            if ;; label = @5
              local.get 2
              i32.const -8
              i32.and
              local.set 9
              local.get 4
              i32.load offset=12
              local.set 1
              block ;; label = @6
                local.get 2
                i32.const 255
                i32.le_u
                if ;; label = @7
                  local.get 4
                  i32.load offset=8
                  local.tee 0
                  local.get 1
                  i32.eq
                  if ;; label = @8
                    i32.const 1052692
                    i32.const 1052692
                    i32.load
                    i32.const -2
                    local.get 2
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store
                    br 2 (;@6;)
                  else
                  end
                  local.get 1
                  local.get 0
                  i32.store offset=8
                  local.get 0
                  local.get 1
                  i32.store offset=12
                  br 1 (;@6;)
                else
                end
                local.get 4
                i32.load offset=24
                local.set 6
                block ;; label = @7
                  local.get 1
                  local.get 4
                  i32.ne
                  if ;; label = @8
                    local.get 4
                    i32.load offset=8
                    local.tee 0
                    local.get 1
                    i32.store offset=12
                    local.get 1
                    local.get 0
                    i32.store offset=8
                    br 1 (;@7;)
                  else
                  end
                  block ;; label = @8
                    local.get 4
                    i32.load offset=20
                    local.tee 2
                    if (result i32) ;; label = @9
                      local.get 4
                      i32.const 20
                      i32.add
                    else
                      local.get 4
                      i32.load offset=16
                      local.tee 2
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 4
                      i32.const 16
                      i32.add
                    end
                    local.set 0
                    loop ;; label = @9
                      local.get 0
                      local.set 5
                      local.get 2
                      local.tee 1
                      i32.const 20
                      i32.add
                      local.set 0
                      local.get 1
                      i32.load offset=20
                      local.tee 2
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 16
                      i32.add
                      local.set 0
                      local.get 1
                      i32.load offset=16
                      local.tee 2
                      br_if 0 (;@9;)
                    end
                    local.get 5
                    i32.const 0
                    i32.store
                    br 1 (;@7;)
                  end
                  i32.const 0
                  local.set 1
                end
                local.get 6
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 4
                  i32.load offset=28
                  local.tee 0
                  i32.const 2
                  i32.shl
                  local.tee 2
                  i32.load offset=1052996
                  local.get 4
                  i32.eq
                  if ;; label = @8
                    local.get 2
                    i32.const 1052996
                    i32.add
                    local.get 1
                    i32.store
                    local.get 1
                    br_if 1 (;@7;)
                    i32.const 1052696
                    i32.const 1052696
                    i32.load
                    i32.const -2
                    local.get 0
                    i32.rotl
                    i32.and
                    i32.store
                    br 2 (;@6;)
                  else
                  end
                  block ;; label = @8
                    local.get 4
                    local.get 6
                    i32.load offset=16
                    i32.eq
                    if ;; label = @9
                      local.get 6
                      local.get 1
                      i32.store offset=16
                      br 1 (;@8;)
                    else
                    end
                    local.get 6
                    local.get 1
                    i32.store offset=20
                  end
                  local.get 1
                  i32.eqz
                  br_if 1 (;@6;)
                end
                local.get 1
                local.get 6
                i32.store offset=24
                local.get 4
                i32.load offset=16
                local.tee 0
                if ;; label = @7
                  local.get 1
                  local.get 0
                  i32.store offset=16
                  local.get 0
                  local.get 1
                  i32.store offset=24
                else
                end
                local.get 4
                i32.load offset=20
                local.tee 0
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                local.get 0
                i32.store offset=20
                local.get 0
                local.get 1
                i32.store offset=24
              end
              local.get 7
              local.get 9
              i32.add
              local.set 7
              local.get 4
              local.get 9
              i32.add
              local.tee 4
              i32.load offset=4
              local.set 2
            else
            end
            local.get 4
            local.get 2
            i32.const -2
            i32.and
            i32.store offset=4
            local.get 3
            local.get 7
            i32.add
            local.get 7
            i32.store
            local.get 3
            local.get 7
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 7
            i32.const 255
            i32.le_u
            if ;; label = @5
              local.get 7
              i32.const -8
              i32.and
              i32.const 1052732
              i32.add
              local.set 0
              block (result i32) ;; label = @6
                i32.const 1052692
                i32.load
                local.tee 1
                i32.const 1
                local.get 7
                i32.const 3
                i32.shr_u
                i32.shl
                local.tee 2
                i32.and
                i32.eqz
                if ;; label = @7
                  i32.const 1052692
                  local.get 1
                  local.get 2
                  i32.or
                  i32.store
                  local.get 0
                  br 1 (;@6;)
                else
                end
                local.get 0
                i32.load offset=8
              end
              local.tee 1
              local.get 3
              i32.store offset=12
              local.get 0
              local.get 3
              i32.store offset=8
              local.get 3
              local.get 0
              i32.store offset=12
              local.get 3
              local.get 1
              i32.store offset=8
              br 1 (;@4;)
            else
            end
            i32.const 31
            local.set 1
            local.get 7
            i32.const 16777215
            i32.le_u
            if ;; label = @5
              local.get 7
              i32.const 38
              local.get 7
              i32.const 8
              i32.shr_u
              i32.clz
              local.tee 0
              i32.sub
              i32.shr_u
              i32.const 1
              i32.and
              local.get 0
              i32.const 1
              i32.shl
              i32.sub
              i32.const 62
              i32.add
              local.set 1
            else
            end
            local.get 3
            local.get 1
            i32.store offset=28
            local.get 3
            i64.const 0
            i64.store offset=16 align=4
            local.get 1
            i32.const 2
            i32.shl
            i32.const 1052996
            i32.add
            local.set 0
            i32.const 1052696
            i32.load
            local.tee 2
            i32.const 1
            local.get 1
            i32.shl
            local.tee 5
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 0
              local.get 3
              i32.store
              i32.const 1052696
              local.get 2
              local.get 5
              i32.or
              i32.store
              local.get 3
              local.get 0
              i32.store offset=24
              local.get 3
              local.get 3
              i32.store offset=8
              local.get 3
              local.get 3
              i32.store offset=12
              br 1 (;@4;)
            else
            end
            local.get 7
            i32.const 25
            local.get 1
            i32.const 1
            i32.shr_u
            i32.sub
            i32.const 0
            local.get 1
            i32.const 31
            i32.ne
            select
            i32.shl
            local.set 1
            local.get 0
            i32.load
            local.set 0
            block ;; label = @5
              loop ;; label = @6
                local.get 0
                local.tee 2
                i32.load offset=4
                i32.const -8
                i32.and
                local.get 7
                i32.eq
                br_if 1 (;@5;)
                local.get 1
                i32.const 29
                i32.shr_u
                local.set 0
                local.get 1
                i32.const 1
                i32.shl
                local.set 1
                local.get 2
                local.get 0
                i32.const 4
                i32.and
                i32.add
                local.tee 5
                i32.load offset=16
                local.tee 0
                br_if 0 (;@6;)
              end
              local.get 5
              i32.const 16
              i32.add
              local.get 3
              i32.store
              local.get 3
              local.get 2
              i32.store offset=24
              local.get 3
              local.get 3
              i32.store offset=12
              local.get 3
              local.get 3
              i32.store offset=8
              br 1 (;@4;)
            end
            local.get 2
            i32.load offset=8
            local.tee 0
            local.get 3
            i32.store offset=12
            local.get 2
            local.get 3
            i32.store offset=8
            local.get 3
            i32.const 0
            i32.store offset=24
            local.get 3
            local.get 2
            i32.store offset=12
            local.get 3
            local.get 0
            i32.store offset=8
          end
          local.get 8
          i32.const 8
          i32.add
          local.set 1
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.load offset=28
            local.tee 0
            i32.const 2
            i32.shl
            local.tee 2
            i32.load offset=1052996
            local.get 5
            i32.eq
            if ;; label = @5
              local.get 2
              i32.const 1052996
              i32.add
              local.get 1
              i32.store
              local.get 1
              br_if 1 (;@4;)
              i32.const 1052696
              local.get 8
              i32.const -2
              local.get 0
              i32.rotl
              i32.and
              local.tee 8
              i32.store
              br 2 (;@3;)
            else
            end
            block ;; label = @5
              local.get 5
              local.get 7
              i32.load offset=16
              i32.eq
              if ;; label = @6
                local.get 7
                local.get 1
                i32.store offset=16
                br 1 (;@5;)
              else
              end
              local.get 7
              local.get 1
              i32.store offset=20
            end
            local.get 1
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 1
          local.get 7
          i32.store offset=24
          local.get 5
          i32.load offset=16
          local.tee 0
          if ;; label = @4
            local.get 1
            local.get 0
            i32.store offset=16
            local.get 0
            local.get 1
            i32.store offset=24
          else
          end
          local.get 5
          i32.load offset=20
          local.tee 0
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 0
          i32.store offset=20
          local.get 0
          local.get 1
          i32.store offset=24
        end
        block ;; label = @3
          local.get 3
          i32.const 15
          i32.le_u
          if ;; label = @4
            local.get 5
            local.get 3
            local.get 6
            i32.or
            local.tee 0
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 0
            local.get 5
            i32.add
            local.tee 0
            local.get 0
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            br 1 (;@3;)
          else
          end
          local.get 5
          local.get 6
          i32.add
          local.tee 4
          local.get 3
          i32.const 1
          i32.or
          i32.store offset=4
          local.get 5
          local.get 6
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 3
          local.get 4
          i32.add
          local.get 3
          i32.store
          local.get 3
          i32.const 255
          i32.le_u
          if ;; label = @4
            local.get 3
            i32.const -8
            i32.and
            i32.const 1052732
            i32.add
            local.set 0
            block (result i32) ;; label = @5
              i32.const 1052692
              i32.load
              local.tee 1
              i32.const 1
              local.get 3
              i32.const 3
              i32.shr_u
              i32.shl
              local.tee 2
              i32.and
              i32.eqz
              if ;; label = @6
                i32.const 1052692
                local.get 1
                local.get 2
                i32.or
                i32.store
                local.get 0
                br 1 (;@5;)
              else
              end
              local.get 0
              i32.load offset=8
            end
            local.tee 1
            local.get 4
            i32.store offset=12
            local.get 0
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 0
            i32.store offset=12
            local.get 4
            local.get 1
            i32.store offset=8
            br 1 (;@3;)
          else
          end
          i32.const 31
          local.set 1
          local.get 3
          i32.const 16777215
          i32.le_u
          if ;; label = @4
            local.get 3
            i32.const 38
            local.get 3
            i32.const 8
            i32.shr_u
            i32.clz
            local.tee 0
            i32.sub
            i32.shr_u
            i32.const 1
            i32.and
            local.get 0
            i32.const 1
            i32.shl
            i32.sub
            i32.const 62
            i32.add
            local.set 1
          else
          end
          local.get 4
          local.get 1
          i32.store offset=28
          local.get 4
          i64.const 0
          i64.store offset=16 align=4
          local.get 1
          i32.const 2
          i32.shl
          i32.const 1052996
          i32.add
          local.set 0
          local.get 8
          i32.const 1
          local.get 1
          i32.shl
          local.tee 2
          i32.and
          i32.eqz
          if ;; label = @4
            local.get 0
            local.get 4
            i32.store
            i32.const 1052696
            local.get 2
            local.get 8
            i32.or
            i32.store
            local.get 4
            local.get 0
            i32.store offset=24
            local.get 4
            local.get 4
            i32.store offset=8
            local.get 4
            local.get 4
            i32.store offset=12
            br 1 (;@3;)
          else
          end
          local.get 3
          i32.const 25
          local.get 1
          i32.const 1
          i32.shr_u
          i32.sub
          i32.const 0
          local.get 1
          i32.const 31
          i32.ne
          select
          i32.shl
          local.set 1
          local.get 0
          i32.load
          local.set 0
          block ;; label = @4
            loop ;; label = @5
              local.get 0
              local.tee 2
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 3
              i32.eq
              br_if 1 (;@4;)
              local.get 1
              i32.const 29
              i32.shr_u
              local.set 0
              local.get 1
              i32.const 1
              i32.shl
              local.set 1
              local.get 2
              local.get 0
              i32.const 4
              i32.and
              i32.add
              local.tee 7
              i32.load offset=16
              local.tee 0
              br_if 0 (;@5;)
            end
            local.get 7
            i32.const 16
            i32.add
            local.get 4
            i32.store
            local.get 4
            local.get 2
            i32.store offset=24
            local.get 4
            local.get 4
            i32.store offset=12
            local.get 4
            local.get 4
            i32.store offset=8
            br 1 (;@3;)
          end
          local.get 2
          i32.load offset=8
          local.tee 0
          local.get 4
          i32.store offset=12
          local.get 2
          local.get 4
          i32.store offset=8
          local.get 4
          i32.const 0
          i32.store offset=24
          local.get 4
          local.get 2
          i32.store offset=12
          local.get 4
          local.get 0
          i32.store offset=8
        end
        local.get 5
        i32.const 8
        i32.add
        local.set 1
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 9
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 2
          i32.load offset=28
          local.tee 0
          i32.const 2
          i32.shl
          local.tee 5
          i32.load offset=1052996
          local.get 2
          i32.eq
          if ;; label = @4
            local.get 5
            i32.const 1052996
            i32.add
            local.get 1
            i32.store
            local.get 1
            br_if 1 (;@3;)
            i32.const 1052696
            local.get 11
            i32.const -2
            local.get 0
            i32.rotl
            i32.and
            i32.store
            br 2 (;@2;)
          else
          end
          block ;; label = @4
            local.get 2
            local.get 9
            i32.load offset=16
            i32.eq
            if ;; label = @5
              local.get 9
              local.get 1
              i32.store offset=16
              br 1 (;@4;)
            else
            end
            local.get 9
            local.get 1
            i32.store offset=20
          end
          local.get 1
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 1
        local.get 9
        i32.store offset=24
        local.get 2
        i32.load offset=16
        local.tee 0
        if ;; label = @3
          local.get 1
          local.get 0
          i32.store offset=16
          local.get 0
          local.get 1
          i32.store offset=24
        else
        end
        local.get 2
        i32.load offset=20
        local.tee 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        i32.store offset=20
        local.get 0
        local.get 1
        i32.store offset=24
      end
      block ;; label = @2
        local.get 3
        i32.const 15
        i32.le_u
        if ;; label = @3
          local.get 2
          local.get 3
          local.get 6
          i32.or
          local.tee 0
          i32.const 3
          i32.or
          i32.store offset=4
          local.get 0
          local.get 2
          i32.add
          local.tee 0
          local.get 0
          i32.load offset=4
          i32.const 1
          i32.or
          i32.store offset=4
          br 1 (;@2;)
        else
        end
        local.get 2
        local.get 6
        i32.add
        local.tee 5
        local.get 3
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 2
        local.get 6
        i32.const 3
        i32.or
        i32.store offset=4
        local.get 3
        local.get 5
        i32.add
        local.get 3
        i32.store
        local.get 8
        if ;; label = @3
          local.get 8
          i32.const -8
          i32.and
          i32.const 1052732
          i32.add
          local.set 0
          i32.const 1052712
          i32.load
          local.set 1
          block (result i32) ;; label = @4
            i32.const 1
            local.get 8
            i32.const 3
            i32.shr_u
            i32.shl
            local.tee 7
            local.get 4
            i32.and
            i32.eqz
            if ;; label = @5
              i32.const 1052692
              local.get 4
              local.get 7
              i32.or
              i32.store
              local.get 0
              br 1 (;@4;)
            else
            end
            local.get 0
            i32.load offset=8
          end
          local.tee 4
          local.get 1
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          local.get 1
          local.get 0
          i32.store offset=12
          local.get 1
          local.get 4
          i32.store offset=8
        else
        end
        i32.const 1052712
        local.get 5
        i32.store
        i32.const 1052700
        local.get 3
        i32.store
      end
      local.get 2
      i32.const 8
      i32.add
      local.set 1
    end
    local.get 10
    i32.const 16
    i32.add
    global.set 0
    local.get 1
  )
  (func (;3;) (type 1) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 6
    global.set 0
    i32.const 3
    local.set 7
    block ;; label = @1
      i32.const 1052628
      i32.load8_u
      i32.const 1
      i32.sub
      local.tee 0
      i32.const 255
      i32.and
      i32.const 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 6
      i32.const 4
      i32.add
      local.set 8
      global.get 0
      i32.const 416
      i32.sub
      local.tee 4
      global.set 0
      local.get 4
      i32.const 20
      i32.add
      local.tee 3
      i32.const 1051297
      i32.const 14
      memory.copy
      local.get 3
      i32.const 14
      i32.add
      i32.const 0
      i32.store8
      local.get 4
      i32.const 404
      i32.add
      local.set 2
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              local.get 3
              i32.const 3
              i32.add
              i32.const -4
              i32.and
              local.tee 0
              i32.ne
              if ;; label = @6
                local.get 0
                local.get 3
                i32.sub
                local.set 0
                loop ;; label = @7
                  local.get 1
                  local.get 3
                  i32.add
                  i32.load8_u
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.add
                  local.tee 1
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 0
                i32.const 7
                i32.gt_u
                br_if 2 (;@4;)
                br 1 (;@5;)
              else
              end
              i32.const 0
              local.set 0
            end
            loop ;; label = @5
              i32.const 16843008
              local.get 0
              local.get 3
              i32.add
              local.tee 1
              i32.load
              local.tee 5
              i32.sub
              local.get 5
              i32.or
              i32.const 16843008
              local.get 1
              i32.const 4
              i32.add
              i32.load
              local.tee 1
              i32.sub
              local.get 1
              i32.or
              i32.and
              i32.const -2139062144
              i32.and
              i32.const -2139062144
              i32.ne
              br_if 1 (;@4;)
              local.get 0
              i32.const 8
              i32.add
              local.tee 0
              i32.const 7
              i32.le_u
              br_if 0 (;@5;)
            end
          end
          local.get 0
          i32.const 15
          i32.ne
          if ;; label = @4
            loop ;; label = @5
              local.get 0
              local.get 3
              i32.add
              i32.load8_u
              i32.eqz
              if ;; label = @6
                local.get 0
                local.set 1
                br 3 (;@3;)
              else
              end
              local.get 0
              i32.const 1
              i32.add
              local.tee 0
              i32.const 15
              i32.ne
              br_if 0 (;@5;)
            end
          else
          end
          local.get 2
          i32.const 1
          i32.store offset=4
          local.get 2
          i32.const 1
          i32.store
          br 1 (;@2;)
        end
        local.get 1
        i32.const 14
        i32.ne
        if ;; label = @3
          local.get 2
          local.get 1
          i32.store offset=8
          local.get 2
          i32.const 0
          i32.store offset=4
          local.get 2
          i32.const 1
          i32.store
          br 1 (;@2;)
        else
        end
        local.get 2
        i32.const 15
        i32.store offset=8
        local.get 2
        local.get 3
        i32.store offset=4
        local.get 2
        i32.const 0
        i32.store
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 4
            i32.load offset=404
            i32.const 1
            i32.eq
            if ;; label = @5
              local.get 4
              i32.const 1051008
              i64.load
              i64.store offset=12 align=4
              i32.const -2147483647
              local.set 0
              br 1 (;@4;)
            else
            end
            block (result i32) ;; label = @5
              local.get 4
              i32.load offset=408
              local.set 3
              i32.const 1052596
              i32.load
              i32.const -1
              i32.eq
              if ;; label = @6
                global.get 0
                i32.const 16
                i32.sub
                local.tee 1
                global.set 0
                local.get 1
                i32.const 8
                i32.add
                i32.const 8
                i32.store align=1
                local.get 1
                i32.const 12
                i32.add
                i32.const 1
                i32.store align=1
                block ;; label = @7
                  local.get 1
                  i32.load offset=12
                  local.tee 0
                  i32.eqz
                  if ;; label = @8
                    i32.const 1052688
                    local.set 0
                    br 1 (;@7;)
                  else
                  end
                  block ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 0
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      i32.load offset=8
                      call 2
                      local.tee 2
                      i32.eqz
                      br_if 0 (;@9;)
                      block ;; label = @10
                        block (result i32) ;; label = @11
                          i32.const 0
                          local.get 0
                          i32.eqz
                          br_if 0 (;@11;)
                          drop
                          local.get 0
                          i64.extend_i32_u
                          i64.const 2
                          i64.shl
                          local.tee 14
                          i32.wrap_i64
                          local.tee 5
                          local.get 0
                          i32.const 4
                          i32.or
                          i32.const 65536
                          i32.lt_u
                          br_if 0 (;@11;)
                          drop
                          i32.const -1
                          local.get 5
                          local.get 14
                          i64.const 32
                          i64.shr_u
                          i32.wrap_i64
                          select
                        end
                        local.tee 5
                        call 2
                        local.tee 0
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 4
                        i32.sub
                        i32.load8_u
                        i32.const 3
                        i32.and
                        i32.eqz
                        local.get 5
                        i32.eqz
                        i32.or
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const 0
                        local.get 5
                        memory.fill
                      end
                      local.get 0
                      br_if 1 (;@8;)
                      local.get 2
                      call 5
                    end
                    i32.const 70
                    call 1
                    unreachable
                  end
                  local.get 0
                  local.get 2
                  i32.store align=1
                  local.get 2
                  i32.const 1050071
                  i32.load align=1
                  i32.store offset=3 align=1
                  local.get 2
                  i32.const 1050068
                  i32.load align=1
                  i32.store align=1
                  local.get 2
                  i32.const 0
                  i32.store8 offset=7
                end
                i32.const 1052596
                local.get 0
                i32.store
                local.get 1
                i32.const 16
                i32.add
                global.set 0
              else
              end
              i32.const 0
              local.get 3
              block (result i32) ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    local.tee 0
                    i32.const 3
                    i32.and
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 0
                    i32.load8_u
                    local.tee 1
                    i32.eqz
                    br_if 2 (;@6;)
                    drop
                    local.get 0
                    local.get 1
                    i32.const 61
                    i32.eq
                    br_if 2 (;@6;)
                    drop
                    local.get 0
                    i32.const 1
                    i32.add
                    local.tee 1
                    i32.const 3
                    i32.and
                    i32.eqz
                    if ;; label = @9
                      local.get 1
                      local.set 0
                      br 1 (;@8;)
                    else
                    end
                    local.get 1
                    i32.load8_u
                    local.tee 2
                    i32.eqz
                    local.get 2
                    i32.const 61
                    i32.eq
                    i32.or
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 2
                    i32.add
                    local.tee 1
                    i32.const 3
                    i32.and
                    i32.eqz
                    if ;; label = @9
                      local.get 1
                      local.set 0
                      br 1 (;@8;)
                    else
                    end
                    local.get 1
                    i32.load8_u
                    local.tee 2
                    i32.eqz
                    local.get 2
                    i32.const 61
                    i32.eq
                    i32.or
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 3
                    i32.add
                    local.tee 1
                    i32.const 3
                    i32.and
                    i32.eqz
                    if ;; label = @9
                      local.get 1
                      local.set 0
                      br 1 (;@8;)
                    else
                    end
                    local.get 1
                    i32.load8_u
                    local.tee 2
                    i32.eqz
                    local.get 2
                    i32.const 61
                    i32.eq
                    i32.or
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 4
                    i32.add
                    local.set 0
                  end
                  block ;; label = @8
                    i32.const 16843008
                    local.get 0
                    i32.load
                    local.tee 1
                    i32.sub
                    local.get 1
                    i32.or
                    i32.const -2139062144
                    i32.and
                    i32.const -2139062144
                    i32.ne
                    br_if 0 (;@8;)
                    loop ;; label = @9
                      i32.const 16843008
                      local.get 1
                      i32.const 1027423549
                      i32.xor
                      local.tee 1
                      i32.sub
                      local.get 1
                      i32.or
                      i32.const -2139062144
                      i32.and
                      i32.const -2139062144
                      i32.ne
                      br_if 1 (;@8;)
                      i32.const 16843008
                      local.get 0
                      i32.const 4
                      i32.add
                      local.tee 0
                      i32.load
                      local.tee 1
                      i32.sub
                      local.get 1
                      i32.or
                      i32.const -2139062144
                      i32.and
                      i32.const -2139062144
                      i32.eq
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 0
                  i32.const 1
                  i32.sub
                  local.set 1
                  loop ;; label = @8
                    local.get 1
                    i32.const 1
                    i32.add
                    local.tee 1
                    i32.load8_u
                    local.tee 0
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 61
                    i32.ne
                    br_if 0 (;@8;)
                  end
                end
                local.get 1
              end
              local.tee 0
              i32.eq
              br_if 0 (;@5;)
              drop
              block ;; label = @6
                local.get 3
                local.get 0
                local.get 3
                i32.sub
                local.tee 5
                i32.add
                i32.load8_u
                br_if 0 (;@6;)
                i32.const 1052596
                i32.load
                local.tee 1
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.load
                local.tee 0
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.const 4
                i32.add
                local.set 9
                loop ;; label = @7
                  block ;; label = @8
                    block (result i32) ;; label = @9
                      local.get 0
                      local.set 1
                      i32.const 0
                      local.get 5
                      i32.eqz
                      br_if 0 (;@9;)
                      drop
                      block ;; label = @10
                        local.get 3
                        i32.load8_u
                        local.tee 2
                        i32.eqz
                        if ;; label = @11
                          i32.const 0
                          local.set 2
                          br 1 (;@10;)
                        else
                        end
                        local.get 3
                        i32.const 1
                        i32.add
                        local.set 10
                        local.get 5
                        i32.const 1
                        i32.sub
                        local.set 11
                        block ;; label = @11
                          loop ;; label = @12
                            local.get 11
                            i32.eqz
                            local.get 2
                            local.get 1
                            i32.load8_u
                            local.tee 12
                            i32.ne
                            local.get 12
                            i32.eqz
                            i32.or
                            i32.or
                            br_if 1 (;@11;)
                            local.get 11
                            i32.const 1
                            i32.sub
                            local.set 11
                            local.get 1
                            i32.const 1
                            i32.add
                            local.set 1
                            local.get 10
                            i32.load8_u
                            local.set 2
                            local.get 10
                            i32.const 1
                            i32.add
                            local.set 10
                            local.get 2
                            br_if 0 (;@12;)
                          end
                          i32.const 0
                          local.set 2
                        end
                      end
                      local.get 2
                      local.get 1
                      i32.load8_u
                      i32.sub
                    end
                    i32.eqz
                    if ;; label = @9
                      local.get 0
                      local.get 5
                      i32.add
                      local.tee 0
                      i32.load8_u
                      i32.const 61
                      i32.eq
                      br_if 1 (;@8;)
                    else
                    end
                    local.get 9
                    i32.load
                    local.set 0
                    local.get 9
                    i32.const 4
                    i32.add
                    local.set 9
                    local.get 0
                    br_if 1 (;@7;)
                    br 2 (;@6;)
                  end
                end
                local.get 0
                i32.const 1
                i32.add
                local.set 13
              end
              local.get 13
            end
            local.tee 3
            i32.eqz
            if ;; label = @5
              i32.const -2147483648
              local.set 0
              br 1 (;@4;)
            else
            end
            block ;; label = @5
              local.get 3
              call 28
              local.tee 0
              i32.eqz
              if ;; label = @6
                i32.const 1
                local.set 1
                br 1 (;@5;)
              else
              end
              local.get 0
              i32.const 1
              call 42
              local.tee 1
              i32.eqz
              br_if 2 (;@3;)
              local.get 0
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 3
              local.get 0
              memory.copy
            end
            local.get 4
            local.get 0
            i32.store offset=16
            local.get 4
            local.get 1
            i32.store offset=12
          end
          block ;; label = @4
            local.get 0
            i32.const -2147483647
            i32.ne
            if ;; label = @5
              local.get 8
              local.get 4
              i64.load offset=12 align=4
              i64.store offset=4 align=4
              local.get 8
              local.get 0
              i32.store
              br 1 (;@4;)
            else
            end
            local.get 4
            i32.load8_u offset=12
            i32.const 3
            i32.eq
            if ;; label = @5
              local.get 4
              i32.load offset=16
              local.tee 0
              i32.load
              local.set 1
              local.get 0
              i32.const 4
              i32.add
              i32.load
              local.tee 3
              i32.load
              local.tee 2
              if ;; label = @6
                local.get 1
                local.get 2
                call_indirect (type 2)
              else
              end
              local.get 3
              i32.load offset=4
              if ;; label = @6
                local.get 3
                i32.load offset=8
                drop
                local.get 1
                call 5
              else
              end
              local.get 0
              call 5
            else
            end
            local.get 8
            i32.const -2147483648
            i32.store
          end
          local.get 4
          i32.const 416
          i32.add
          global.set 0
          br 1 (;@2;)
        end
        i32.const 1
        local.get 0
        call 61
        unreachable
      end
      block ;; label = @2
        local.get 6
        i32.load offset=4
        local.tee 3
        i32.const -2147483648
        i32.eq
        if ;; label = @3
          i32.const 2
          local.set 0
          br 1 (;@2;)
        else
        end
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 6
                i32.load offset=12
                i32.const 1
                i32.sub
                br_table 0 (;@6;) 2 (;@4;) 2 (;@4;) 1 (;@5;) 2 (;@4;)
              end
              local.get 6
              i32.load offset=8
              local.tee 1
              i32.load8_u
              i32.const 48
              i32.ne
              br_if 1 (;@4;)
              i32.const 2
              local.set 0
              local.get 3
              br_if 2 (;@3;)
              br 3 (;@2;)
            end
            local.get 6
            i32.load offset=8
            local.tee 1
            i32.load align=1
            i32.const 1819047270
            i32.ne
            br_if 0 (;@4;)
            i32.const 2
            local.set 7
            i32.const 1
            local.set 0
            local.get 3
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          i32.const 1
          local.set 7
          i32.const 0
          local.set 0
          local.get 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 6
          i32.load offset=8
          local.set 1
        end
        local.get 1
        call 5
      end
      i32.const 1052628
      i32.const 1052628
      i32.load8_u
      local.tee 1
      local.get 7
      local.get 1
      select
      i32.store8
      local.get 1
      i32.eqz
      br_if 0 (;@1;)
      i32.const 3
      local.set 0
      local.get 1
      i32.const 4
      i32.ge_u
      br_if 0 (;@1;)
      i32.const 33619971
      local.get 1
      i32.const 3
      i32.shl
      i32.const 248
      i32.and
      i32.shr_u
      local.set 0
    end
    local.get 6
    i32.const 16
    i32.add
    global.set 0
    local.get 0
  )
  (func (;4;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 12
        i32.const 402653184
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 12
                i32.const 268435456
                i32.and
                if ;; label = @7
                  local.get 0
                  i32.load16_u offset=14
                  local.tee 7
                  br_if 1 (;@6;)
                  i32.const 0
                  local.set 2
                  br 2 (;@5;)
                else
                end
                local.get 2
                i32.const 16
                i32.ge_u
                if ;; label = @7
                  block (result i32) ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 2
                        local.get 1
                        i32.const 3
                        i32.add
                        i32.const -4
                        i32.and
                        local.tee 4
                        local.get 1
                        i32.sub
                        local.tee 8
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 2
                        local.get 8
                        i32.sub
                        local.tee 7
                        i32.const 2
                        i32.shr_u
                        local.tee 11
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 4
                        i32.ne
                        if ;; label = @11
                          local.get 1
                          local.get 4
                          i32.sub
                          local.tee 4
                          i32.const -4
                          i32.le_u
                          if ;; label = @12
                            loop ;; label = @13
                              local.get 3
                              local.get 1
                              local.get 6
                              i32.add
                              local.tee 10
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              i32.add
                              local.get 10
                              i32.const 1
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              i32.add
                              local.get 10
                              i32.const 2
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              i32.add
                              local.get 10
                              i32.const 3
                              i32.add
                              i32.load8_s
                              i32.const -65
                              i32.gt_s
                              i32.add
                              local.set 3
                              local.get 6
                              i32.const 4
                              i32.add
                              local.tee 6
                              br_if 0 (;@13;)
                            end
                          else
                          end
                          local.get 1
                          local.get 6
                          i32.add
                          local.set 9
                          loop ;; label = @12
                            local.get 3
                            local.get 9
                            i32.load8_s
                            i32.const -65
                            i32.gt_s
                            i32.add
                            local.set 3
                            local.get 9
                            i32.const 1
                            i32.add
                            local.set 9
                            local.get 4
                            i32.const 1
                            i32.add
                            local.tee 4
                            br_if 0 (;@12;)
                          end
                        else
                        end
                        local.get 1
                        local.get 8
                        i32.add
                        local.set 4
                        block ;; label = @11
                          local.get 7
                          i32.const 3
                          i32.and
                          local.tee 8
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 4
                          local.get 7
                          i32.const 2147483644
                          i32.and
                          i32.add
                          local.tee 7
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          local.set 5
                          local.get 8
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 5
                          local.get 7
                          i32.load8_s offset=1
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.set 5
                          local.get 8
                          i32.const 2
                          i32.eq
                          br_if 0 (;@11;)
                          local.get 5
                          local.get 7
                          i32.load8_s offset=2
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.set 5
                        end
                        local.get 3
                        local.get 5
                        i32.add
                        local.set 6
                        loop ;; label = @11
                          local.get 4
                          local.set 5
                          local.get 11
                          i32.eqz
                          br_if 2 (;@9;)
                          i32.const 192
                          local.get 11
                          local.get 11
                          i32.const 192
                          i32.ge_u
                          select
                          local.tee 13
                          i32.const 3
                          i32.and
                          local.set 10
                          block ;; label = @12
                            local.get 13
                            i32.const 2
                            i32.shl
                            local.tee 8
                            i32.const 1008
                            i32.and
                            local.tee 3
                            i32.eqz
                            if ;; label = @13
                              i32.const 0
                              local.set 9
                              br 1 (;@12;)
                            else
                            end
                            local.get 3
                            local.get 5
                            i32.add
                            local.set 7
                            i32.const 0
                            local.set 9
                            local.get 5
                            local.set 3
                            loop ;; label = @13
                              local.get 9
                              local.get 3
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.get 3
                              i32.const 4
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.get 3
                              i32.const 8
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.get 3
                              i32.const 12
                              i32.add
                              i32.load
                              local.tee 4
                              i32.const -1
                              i32.xor
                              i32.const 7
                              i32.shr_u
                              local.get 4
                              i32.const 6
                              i32.shr_u
                              i32.or
                              i32.const 16843009
                              i32.and
                              i32.add
                              local.set 9
                              local.get 3
                              i32.const 16
                              i32.add
                              local.tee 3
                              local.get 7
                              i32.ne
                              br_if 0 (;@13;)
                            end
                          end
                          local.get 11
                          local.get 13
                          i32.sub
                          local.set 11
                          local.get 5
                          local.get 8
                          i32.add
                          local.set 4
                          local.get 9
                          i32.const 8
                          i32.shr_u
                          i32.const 16711935
                          i32.and
                          local.get 9
                          i32.const 16711935
                          i32.and
                          i32.add
                          i32.const 65537
                          i32.mul
                          i32.const 16
                          i32.shr_u
                          local.get 6
                          i32.add
                          local.set 6
                          local.get 10
                          i32.eqz
                          br_if 0 (;@11;)
                        end
                        block (result i32) ;; label = @11
                          local.get 5
                          local.get 13
                          i32.const 252
                          i32.and
                          i32.const 2
                          i32.shl
                          i32.add
                          local.tee 4
                          i32.load
                          local.tee 3
                          i32.const -1
                          i32.xor
                          i32.const 7
                          i32.shr_u
                          local.get 3
                          i32.const 6
                          i32.shr_u
                          i32.or
                          i32.const 16843009
                          i32.and
                          local.tee 5
                          local.get 10
                          i32.const 1
                          i32.eq
                          br_if 0 (;@11;)
                          drop
                          local.get 5
                          local.get 4
                          i32.load offset=4
                          local.tee 3
                          i32.const -1
                          i32.xor
                          i32.const 7
                          i32.shr_u
                          local.get 3
                          i32.const 6
                          i32.shr_u
                          i32.or
                          i32.const 16843009
                          i32.and
                          i32.add
                          local.tee 5
                          local.get 10
                          i32.const 2
                          i32.eq
                          br_if 0 (;@11;)
                          drop
                          local.get 5
                          local.get 4
                          i32.load offset=8
                          local.tee 3
                          i32.const -1
                          i32.xor
                          i32.const 7
                          i32.shr_u
                          local.get 3
                          i32.const 6
                          i32.shr_u
                          i32.or
                          i32.const 16843009
                          i32.and
                          i32.add
                        end
                        local.tee 3
                        i32.const 8
                        i32.shr_u
                        i32.const 459007
                        i32.and
                        local.get 3
                        i32.const 16711935
                        i32.and
                        i32.add
                        i32.const 65537
                        i32.mul
                        i32.const 16
                        i32.shr_u
                        local.get 6
                        i32.add
                        local.set 6
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.get 2
                      i32.eqz
                      br_if 1 (;@8;)
                      drop
                      local.get 2
                      i32.const 3
                      i32.and
                      local.set 9
                      i32.const 0
                      local.set 4
                      local.get 2
                      i32.const 4
                      i32.ge_u
                      if ;; label = @10
                        local.get 2
                        i32.const -4
                        i32.and
                        local.set 3
                        loop ;; label = @11
                          local.get 6
                          local.get 1
                          local.get 4
                          i32.add
                          local.tee 5
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.get 5
                          i32.const 1
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.get 5
                          i32.const 2
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.get 5
                          i32.const 3
                          i32.add
                          i32.load8_s
                          i32.const -65
                          i32.gt_s
                          i32.add
                          local.set 6
                          local.get 3
                          local.get 4
                          i32.const 4
                          i32.add
                          local.tee 4
                          i32.ne
                          br_if 0 (;@11;)
                        end
                        local.get 9
                        i32.eqz
                        br_if 1 (;@9;)
                      else
                      end
                      local.get 1
                      local.get 4
                      i32.add
                      local.set 3
                      loop ;; label = @10
                        local.get 6
                        local.get 3
                        i32.load8_s
                        i32.const -65
                        i32.gt_s
                        i32.add
                        local.set 6
                        local.get 3
                        i32.const 1
                        i32.add
                        local.set 3
                        local.get 9
                        i32.const 1
                        i32.sub
                        local.tee 9
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 6
                  end
                  local.set 6
                  br 4 (;@3;)
                else
                end
                local.get 2
                i32.eqz
                br_if 3 (;@3;)
                local.get 2
                i32.const 3
                i32.and
                local.set 3
                local.get 2
                i32.const 4
                i32.ge_u
                if ;; label = @7
                  local.get 2
                  i32.const 12
                  i32.and
                  local.set 4
                  loop ;; label = @8
                    local.get 6
                    local.get 1
                    local.get 5
                    i32.add
                    local.tee 7
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 7
                    i32.const 1
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 7
                    i32.const 2
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.get 7
                    i32.const 3
                    i32.add
                    i32.load8_s
                    i32.const -65
                    i32.gt_s
                    i32.add
                    local.set 6
                    local.get 4
                    local.get 5
                    i32.const 4
                    i32.add
                    local.tee 5
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 3
                  i32.eqz
                  br_if 4 (;@3;)
                else
                end
                local.get 1
                local.get 5
                i32.add
                local.set 5
                loop ;; label = @7
                  local.get 6
                  local.get 5
                  i32.load8_s
                  i32.const -65
                  i32.gt_s
                  i32.add
                  local.set 6
                  local.get 5
                  i32.const 1
                  i32.add
                  local.set 5
                  local.get 3
                  i32.const 1
                  i32.sub
                  local.tee 3
                  br_if 0 (;@7;)
                end
                br 3 (;@3;)
              end
              local.get 1
              local.get 2
              i32.add
              local.set 8
              i32.const 0
              local.set 2
              local.get 1
              local.set 5
              local.get 7
              local.set 3
              loop ;; label = @6
                local.get 5
                local.tee 4
                local.get 8
                i32.eq
                br_if 2 (;@4;)
                local.get 2
                block (result i32) ;; label = @7
                  local.get 4
                  i32.const 1
                  i32.add
                  local.get 4
                  i32.load8_s
                  local.tee 5
                  i32.const 0
                  i32.ge_s
                  br_if 0 (;@7;)
                  drop
                  local.get 4
                  i32.const 2
                  i32.add
                  local.get 5
                  i32.const -32
                  i32.lt_u
                  br_if 0 (;@7;)
                  drop
                  local.get 4
                  i32.const 4
                  i32.const 3
                  local.get 5
                  i32.const -17
                  i32.gt_u
                  select
                  i32.add
                end
                local.tee 5
                local.get 4
                i32.sub
                i32.add
                local.set 2
                local.get 3
                i32.const 1
                i32.sub
                local.tee 3
                br_if 0 (;@6;)
              end
            end
            i32.const 0
            local.set 3
          end
          local.get 7
          local.get 3
          i32.sub
          local.set 6
        end
        local.get 6
        local.get 0
        i32.load16_u offset=12
        local.tee 3
        i32.ge_u
        br_if 0 (;@2;)
        local.get 3
        local.get 6
        i32.sub
        local.set 4
        i32.const 0
        local.set 6
        i32.const 0
        local.set 3
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 12
              i32.const 29
              i32.shr_u
              i32.const 3
              i32.and
              i32.const 1
              i32.sub
              br_table 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
            local.get 4
            local.set 3
            br 1 (;@3;)
          end
          local.get 4
          i32.const 65534
          i32.and
          i32.const 1
          i32.shr_u
          local.set 3
        end
        local.get 12
        i32.const 2097151
        i32.and
        local.set 7
        local.get 0
        i32.load offset=4
        local.set 10
        local.get 0
        i32.load
        local.set 8
        loop ;; label = @3
          local.get 6
          i32.const 65535
          i32.and
          local.get 3
          i32.const 65535
          i32.and
          i32.lt_u
          if ;; label = @4
            i32.const 1
            local.set 5
            local.get 6
            i32.const 1
            i32.add
            local.set 6
            local.get 8
            local.get 7
            local.get 10
            i32.load offset=16
            call_indirect (type 5)
            i32.eqz
            br_if 1 (;@3;)
            br 3 (;@1;)
          else
          end
        end
        i32.const 1
        local.set 5
        local.get 8
        local.get 1
        local.get 2
        local.get 10
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        i32.const 0
        local.set 6
        local.get 4
        local.get 3
        i32.sub
        i32.const 65535
        i32.and
        local.set 1
        loop ;; label = @3
          local.get 6
          i32.const 65535
          i32.and
          local.tee 0
          local.get 1
          i32.lt_u
          local.set 5
          local.get 0
          local.get 1
          i32.ge_u
          br_if 2 (;@1;)
          local.get 6
          i32.const 1
          i32.add
          local.set 6
          local.get 8
          local.get 7
          local.get 10
          i32.load offset=16
          call_indirect (type 5)
          i32.eqz
          br_if 0 (;@3;)
        end
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=4
      i32.load offset=12
      call_indirect (type 7)
      local.set 5
    end
    local.get 5
  )
  (func (;5;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.sub
      local.tee 3
      local.get 0
      i32.const 4
      i32.sub
      i32.load
      local.tee 1
      i32.const -8
      i32.and
      local.tee 0
      i32.add
      local.set 5
      block ;; label = @2
        local.get 1
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 1
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        local.get 3
        i32.load
        local.tee 1
        i32.sub
        local.tee 3
        i32.const 1052708
        i32.load
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.add
        local.set 0
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 1052712
              i32.load
              local.get 3
              i32.ne
              if ;; label = @6
                local.get 3
                i32.load offset=12
                local.set 2
                local.get 1
                i32.const 255
                i32.le_u
                if ;; label = @7
                  local.get 2
                  local.get 3
                  i32.load offset=8
                  local.tee 4
                  i32.ne
                  br_if 2 (;@5;)
                  i32.const 1052692
                  i32.const 1052692
                  i32.load
                  i32.const -2
                  local.get 1
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store
                  br 5 (;@2;)
                else
                end
                local.get 3
                i32.load offset=24
                local.set 6
                local.get 2
                local.get 3
                i32.ne
                if ;; label = @7
                  local.get 3
                  i32.load offset=8
                  local.tee 1
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 1
                  i32.store offset=8
                  br 4 (;@3;)
                else
                end
                local.get 3
                i32.load offset=20
                local.tee 1
                if (result i32) ;; label = @7
                  local.get 3
                  i32.const 20
                  i32.add
                else
                  local.get 3
                  i32.load offset=16
                  local.tee 1
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 3
                  i32.const 16
                  i32.add
                end
                local.set 4
                loop ;; label = @7
                  local.get 4
                  local.set 7
                  local.get 1
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 4
                  local.get 2
                  i32.load offset=20
                  local.tee 1
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 2
                  i32.load offset=16
                  local.tee 1
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 3 (;@3;)
              else
              end
              local.get 5
              i32.load offset=4
              local.tee 1
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              local.get 5
              local.get 1
              i32.const -2
              i32.and
              i32.store offset=4
              i32.const 1052700
              local.get 0
              i32.store
              local.get 5
              local.get 0
              i32.store
              local.get 3
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              return
            end
            local.get 2
            local.get 4
            i32.store offset=8
            local.get 4
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
          local.get 3
          i32.load offset=28
          local.tee 1
          i32.const 2
          i32.shl
          local.tee 4
          i32.load offset=1052996
          local.get 3
          i32.eq
          if ;; label = @4
            local.get 4
            i32.const 1052996
            i32.add
            local.get 2
            i32.store
            local.get 2
            br_if 1 (;@3;)
            i32.const 1052696
            i32.const 1052696
            i32.load
            i32.const -2
            local.get 1
            i32.rotl
            i32.and
            i32.store
            br 2 (;@2;)
          else
          end
          block ;; label = @4
            local.get 3
            local.get 6
            i32.load offset=16
            i32.eq
            if ;; label = @5
              local.get 6
              local.get 2
              i32.store offset=16
              br 1 (;@4;)
            else
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
        local.get 3
        i32.load offset=16
        local.tee 1
        if ;; label = @3
          local.get 2
          local.get 1
          i32.store offset=16
          local.get 1
          local.get 2
          i32.store offset=24
        else
        end
        local.get 3
        i32.load offset=20
        local.tee 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.store offset=20
        local.get 1
        local.get 2
        i32.store offset=24
      end
      local.get 3
      local.get 5
      i32.ge_u
      br_if 0 (;@1;)
      local.get 5
      i32.load offset=4
      local.tee 1
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 1
              i32.const 2
              i32.and
              i32.eqz
              if ;; label = @6
                i32.const 1052716
                i32.load
                local.get 5
                i32.eq
                if ;; label = @7
                  i32.const 1052716
                  local.get 3
                  i32.store
                  i32.const 1052704
                  i32.const 1052704
                  i32.load
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store
                  local.get 3
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 3
                  i32.const 1052712
                  i32.load
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 1052700
                  i32.const 0
                  i32.store
                  i32.const 1052712
                  i32.const 0
                  i32.store
                  return
                else
                end
                i32.const 1052712
                i32.load
                local.tee 8
                local.get 5
                i32.eq
                if ;; label = @7
                  i32.const 1052712
                  local.get 3
                  i32.store
                  i32.const 1052700
                  i32.const 1052700
                  i32.load
                  local.get 0
                  i32.add
                  local.tee 0
                  i32.store
                  local.get 3
                  local.get 0
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  local.get 3
                  i32.add
                  local.get 0
                  i32.store
                  return
                else
                end
                local.get 1
                i32.const -8
                i32.and
                local.get 0
                i32.add
                local.set 0
                local.get 5
                i32.load offset=12
                local.set 2
                local.get 1
                i32.const 255
                i32.le_u
                if ;; label = @7
                  local.get 5
                  i32.load offset=8
                  local.tee 4
                  local.get 2
                  i32.eq
                  if ;; label = @8
                    i32.const 1052692
                    i32.const 1052692
                    i32.load
                    i32.const -2
                    local.get 1
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store
                    br 5 (;@3;)
                  else
                  end
                  local.get 2
                  local.get 4
                  i32.store offset=8
                  local.get 4
                  local.get 2
                  i32.store offset=12
                  br 4 (;@3;)
                else
                end
                local.get 5
                i32.load offset=24
                local.set 6
                local.get 2
                local.get 5
                i32.ne
                if ;; label = @7
                  local.get 5
                  i32.load offset=8
                  local.tee 1
                  local.get 2
                  i32.store offset=12
                  local.get 2
                  local.get 1
                  i32.store offset=8
                  br 3 (;@4;)
                else
                end
                local.get 5
                i32.load offset=20
                local.tee 1
                if (result i32) ;; label = @7
                  local.get 5
                  i32.const 20
                  i32.add
                else
                  local.get 5
                  i32.load offset=16
                  local.tee 1
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 5
                  i32.const 16
                  i32.add
                end
                local.set 4
                loop ;; label = @7
                  local.get 4
                  local.set 7
                  local.get 1
                  local.tee 2
                  i32.const 20
                  i32.add
                  local.set 4
                  local.get 2
                  i32.load offset=20
                  local.tee 1
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 16
                  i32.add
                  local.set 4
                  local.get 2
                  i32.load offset=16
                  local.tee 1
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 2 (;@4;)
              else
              end
              local.get 5
              local.get 1
              i32.const -2
              i32.and
              i32.store offset=4
              local.get 0
              local.get 3
              i32.add
              local.get 0
              i32.store
              local.get 3
              local.get 0
              i32.const 1
              i32.or
              i32.store offset=4
              br 3 (;@2;)
            end
            i32.const 0
            local.set 2
          end
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.load offset=28
            local.tee 1
            i32.const 2
            i32.shl
            local.tee 4
            i32.load offset=1052996
            local.get 5
            i32.eq
            if ;; label = @5
              local.get 4
              i32.const 1052996
              i32.add
              local.get 2
              i32.store
              local.get 2
              br_if 1 (;@4;)
              i32.const 1052696
              i32.const 1052696
              i32.load
              i32.const -2
              local.get 1
              i32.rotl
              i32.and
              i32.store
              br 2 (;@3;)
            else
            end
            block ;; label = @5
              local.get 5
              local.get 6
              i32.load offset=16
              i32.eq
              if ;; label = @6
                local.get 6
                local.get 2
                i32.store offset=16
                br 1 (;@5;)
              else
              end
              local.get 6
              local.get 2
              i32.store offset=20
            end
            local.get 2
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 2
          local.get 6
          i32.store offset=24
          local.get 5
          i32.load offset=16
          local.tee 1
          if ;; label = @4
            local.get 2
            local.get 1
            i32.store offset=16
            local.get 1
            local.get 2
            i32.store offset=24
          else
          end
          local.get 5
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
        local.get 0
        local.get 3
        i32.add
        local.get 0
        i32.store
        local.get 3
        local.get 0
        i32.const 1
        i32.or
        i32.store offset=4
        local.get 3
        local.get 8
        i32.ne
        br_if 0 (;@2;)
        i32.const 1052700
        local.get 0
        i32.store
        return
      end
      local.get 0
      i32.const 255
      i32.le_u
      if ;; label = @2
        local.get 0
        i32.const -8
        i32.and
        i32.const 1052732
        i32.add
        local.set 1
        block (result i32) ;; label = @3
          i32.const 1052692
          i32.load
          local.tee 4
          i32.const 1
          local.get 0
          i32.const 3
          i32.shr_u
          i32.shl
          local.tee 0
          i32.and
          i32.eqz
          if ;; label = @4
            i32.const 1052692
            local.get 0
            local.get 4
            i32.or
            i32.store
            local.get 1
            br 1 (;@3;)
          else
          end
          local.get 1
          i32.load offset=8
        end
        local.tee 0
        local.get 3
        i32.store offset=12
        local.get 1
        local.get 3
        i32.store offset=8
        local.get 3
        local.get 1
        i32.store offset=12
        local.get 3
        local.get 0
        i32.store offset=8
        return
      else
      end
      i32.const 31
      local.set 2
      local.get 0
      i32.const 16777215
      i32.le_u
      if ;; label = @2
        local.get 0
        i32.const 38
        local.get 0
        i32.const 8
        i32.shr_u
        i32.clz
        local.tee 1
        i32.sub
        i32.shr_u
        i32.const 1
        i32.and
        local.get 1
        i32.const 1
        i32.shl
        i32.sub
        i32.const 62
        i32.add
        local.set 2
      else
      end
      local.get 3
      local.get 2
      i32.store offset=28
      local.get 3
      i64.const 0
      i64.store offset=16 align=4
      local.get 2
      i32.const 2
      i32.shl
      i32.const 1052996
      i32.add
      local.set 4
      block (result i32) ;; label = @2
        block ;; label = @3
          block (result i32) ;; label = @4
            i32.const 1052696
            i32.load
            local.tee 1
            i32.const 1
            local.get 2
            i32.shl
            local.tee 7
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 4
              local.get 3
              i32.store
              i32.const 1052696
              local.get 1
              local.get 7
              i32.or
              i32.store
              i32.const 24
              local.set 2
              i32.const 8
              br 1 (;@4;)
            else
            end
            local.get 0
            i32.const 25
            local.get 2
            i32.const 1
            i32.shr_u
            i32.sub
            i32.const 0
            local.get 2
            i32.const 31
            i32.ne
            select
            i32.shl
            local.set 2
            local.get 4
            i32.load
            local.set 4
            loop ;; label = @5
              local.get 4
              local.tee 1
              i32.load offset=4
              i32.const -8
              i32.and
              local.get 0
              i32.eq
              br_if 2 (;@3;)
              local.get 2
              i32.const 29
              i32.shr_u
              local.set 4
              local.get 2
              i32.const 1
              i32.shl
              local.set 2
              local.get 1
              local.get 4
              i32.const 4
              i32.and
              i32.add
              local.tee 7
              i32.load offset=16
              local.tee 4
              br_if 0 (;@5;)
            end
            local.get 7
            i32.const 16
            i32.add
            local.get 3
            i32.store
            i32.const 24
            local.set 2
            local.get 1
            local.set 4
            i32.const 8
          end
          local.set 0
          local.get 3
          local.tee 1
          br 1 (;@2;)
        end
        local.get 1
        i32.load offset=8
        local.tee 4
        local.get 3
        i32.store offset=12
        local.get 1
        local.get 3
        i32.store offset=8
        i32.const 24
        local.set 0
        i32.const 8
        local.set 2
        i32.const 0
      end
      local.set 7
      local.get 2
      local.get 3
      i32.add
      local.get 4
      i32.store
      local.get 3
      local.get 1
      i32.store offset=12
      local.get 0
      local.get 3
      i32.add
      local.get 7
      i32.store
      i32.const 1052724
      i32.const 1052724
      i32.load
      i32.const 1
      i32.sub
      local.tee 0
      i32.const -1
      local.get 0
      select
      i32.store
    end
  )
  (func (;6;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    local.get 1
    i32.add
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 2
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        local.get 2
        i32.const 2
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        local.get 0
        i32.load
        local.tee 2
        local.get 1
        i32.add
        local.set 1
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 0
              local.get 2
              i32.sub
              local.tee 0
              i32.const 1052712
              i32.load
              i32.ne
              if ;; label = @6
                local.get 0
                i32.load offset=12
                local.set 3
                local.get 2
                i32.const 255
                i32.le_u
                if ;; label = @7
                  local.get 3
                  local.get 0
                  i32.load offset=8
                  local.tee 4
                  i32.ne
                  br_if 2 (;@5;)
                  i32.const 1052692
                  i32.const 1052692
                  i32.load
                  i32.const -2
                  local.get 2
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store
                  br 5 (;@2;)
                else
                end
                local.get 0
                i32.load offset=24
                local.set 6
                local.get 0
                local.get 3
                i32.ne
                if ;; label = @7
                  local.get 0
                  i32.load offset=8
                  local.tee 2
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 2
                  i32.store offset=8
                  br 4 (;@3;)
                else
                end
                local.get 0
                i32.load offset=20
                local.tee 4
                if (result i32) ;; label = @7
                  local.get 0
                  i32.const 20
                  i32.add
                else
                  local.get 0
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 0
                  i32.const 16
                  i32.add
                end
                local.set 2
                loop ;; label = @7
                  local.get 2
                  local.set 7
                  local.get 4
                  local.tee 3
                  i32.const 20
                  i32.add
                  local.set 2
                  local.get 3
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 3
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 3 (;@3;)
              else
              end
              local.get 5
              i32.load offset=4
              local.tee 2
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if 3 (;@2;)
              local.get 5
              local.get 2
              i32.const -2
              i32.and
              i32.store offset=4
              i32.const 1052700
              local.get 1
              i32.store
              local.get 5
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
            local.get 4
            i32.store offset=8
            local.get 4
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
          local.get 0
          i32.load offset=28
          local.tee 2
          i32.const 2
          i32.shl
          local.tee 4
          i32.load offset=1052996
          local.get 0
          i32.eq
          if ;; label = @4
            local.get 4
            i32.const 1052996
            i32.add
            local.get 3
            i32.store
            local.get 3
            br_if 1 (;@3;)
            i32.const 1052696
            i32.const 1052696
            i32.load
            i32.const -2
            local.get 2
            i32.rotl
            i32.and
            i32.store
            br 2 (;@2;)
          else
          end
          block ;; label = @4
            local.get 0
            local.get 6
            i32.load offset=16
            i32.eq
            if ;; label = @5
              local.get 6
              local.get 3
              i32.store offset=16
              br 1 (;@4;)
            else
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
        local.get 0
        i32.load offset=16
        local.tee 2
        if ;; label = @3
          local.get 3
          local.get 2
          i32.store offset=16
          local.get 2
          local.get 3
          i32.store offset=24
        else
        end
        local.get 0
        i32.load offset=20
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 2
        i32.store offset=20
        local.get 2
        local.get 3
        i32.store offset=24
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 5
              i32.load offset=4
              local.tee 2
              i32.const 2
              i32.and
              i32.eqz
              if ;; label = @6
                i32.const 1052716
                i32.load
                local.get 5
                i32.eq
                if ;; label = @7
                  i32.const 1052716
                  local.get 0
                  i32.store
                  i32.const 1052704
                  i32.const 1052704
                  i32.load
                  local.get 1
                  i32.add
                  local.tee 1
                  i32.store
                  local.get 0
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 0
                  i32.const 1052712
                  i32.load
                  i32.ne
                  br_if 6 (;@1;)
                  i32.const 1052700
                  i32.const 0
                  i32.store
                  i32.const 1052712
                  i32.const 0
                  i32.store
                  return
                else
                end
                i32.const 1052712
                i32.load
                local.tee 8
                local.get 5
                i32.eq
                if ;; label = @7
                  i32.const 1052712
                  local.get 0
                  i32.store
                  i32.const 1052700
                  i32.const 1052700
                  i32.load
                  local.get 1
                  i32.add
                  local.tee 1
                  i32.store
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
                else
                end
                local.get 2
                i32.const -8
                i32.and
                local.get 1
                i32.add
                local.set 1
                local.get 5
                i32.load offset=12
                local.set 3
                local.get 2
                i32.const 255
                i32.le_u
                if ;; label = @7
                  local.get 5
                  i32.load offset=8
                  local.tee 4
                  local.get 3
                  i32.eq
                  if ;; label = @8
                    i32.const 1052692
                    i32.const 1052692
                    i32.load
                    i32.const -2
                    local.get 2
                    i32.const 3
                    i32.shr_u
                    i32.rotl
                    i32.and
                    i32.store
                    br 5 (;@3;)
                  else
                  end
                  local.get 3
                  local.get 4
                  i32.store offset=8
                  local.get 4
                  local.get 3
                  i32.store offset=12
                  br 4 (;@3;)
                else
                end
                local.get 5
                i32.load offset=24
                local.set 6
                local.get 3
                local.get 5
                i32.ne
                if ;; label = @7
                  local.get 5
                  i32.load offset=8
                  local.tee 2
                  local.get 3
                  i32.store offset=12
                  local.get 3
                  local.get 2
                  i32.store offset=8
                  br 3 (;@4;)
                else
                end
                local.get 5
                i32.load offset=20
                local.tee 4
                if (result i32) ;; label = @7
                  local.get 5
                  i32.const 20
                  i32.add
                else
                  local.get 5
                  i32.load offset=16
                  local.tee 4
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 5
                  i32.const 16
                  i32.add
                end
                local.set 2
                loop ;; label = @7
                  local.get 2
                  local.set 7
                  local.get 4
                  local.tee 3
                  i32.const 20
                  i32.add
                  local.set 2
                  local.get 3
                  i32.load offset=20
                  local.tee 4
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 16
                  i32.add
                  local.set 2
                  local.get 3
                  i32.load offset=16
                  local.tee 4
                  br_if 0 (;@7;)
                end
                local.get 7
                i32.const 0
                i32.store
                br 2 (;@4;)
              else
              end
              local.get 5
              local.get 2
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
          local.get 6
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.load offset=28
            local.tee 2
            i32.const 2
            i32.shl
            local.tee 4
            i32.load offset=1052996
            local.get 5
            i32.eq
            if ;; label = @5
              local.get 4
              i32.const 1052996
              i32.add
              local.get 3
              i32.store
              local.get 3
              br_if 1 (;@4;)
              i32.const 1052696
              i32.const 1052696
              i32.load
              i32.const -2
              local.get 2
              i32.rotl
              i32.and
              i32.store
              br 2 (;@3;)
            else
            end
            block ;; label = @5
              local.get 5
              local.get 6
              i32.load offset=16
              i32.eq
              if ;; label = @6
                local.get 6
                local.get 3
                i32.store offset=16
                br 1 (;@5;)
              else
              end
              local.get 6
              local.get 3
              i32.store offset=20
            end
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 3
          local.get 6
          i32.store offset=24
          local.get 5
          i32.load offset=16
          local.tee 2
          if ;; label = @4
            local.get 3
            local.get 2
            i32.store offset=16
            local.get 2
            local.get 3
            i32.store offset=24
          else
          end
          local.get 5
          i32.load offset=20
          local.tee 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 3
          local.get 2
          i32.store offset=20
          local.get 2
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
        local.get 8
        i32.ne
        br_if 0 (;@2;)
        i32.const 1052700
        local.get 1
        i32.store
        return
      end
      local.get 1
      i32.const 255
      i32.le_u
      if ;; label = @2
        local.get 1
        i32.const -8
        i32.and
        i32.const 1052732
        i32.add
        local.set 2
        block (result i32) ;; label = @3
          i32.const 1052692
          i32.load
          local.tee 3
          i32.const 1
          local.get 1
          i32.const 3
          i32.shr_u
          i32.shl
          local.tee 1
          i32.and
          i32.eqz
          if ;; label = @4
            i32.const 1052692
            local.get 1
            local.get 3
            i32.or
            i32.store
            local.get 2
            br 1 (;@3;)
          else
          end
          local.get 2
          i32.load offset=8
        end
        local.tee 1
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=12
        local.get 0
        local.get 1
        i32.store offset=8
        return
      else
      end
      i32.const 31
      local.set 3
      local.get 1
      i32.const 16777215
      i32.le_u
      if ;; label = @2
        local.get 1
        i32.const 38
        local.get 1
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
        local.set 3
      else
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
      i32.const 1052996
      i32.add
      local.set 2
      i32.const 1052696
      i32.load
      local.tee 4
      i32.const 1
      local.get 3
      i32.shl
      local.tee 7
      i32.and
      i32.eqz
      if ;; label = @2
        local.get 2
        local.get 0
        i32.store
        i32.const 1052696
        local.get 4
        local.get 7
        i32.or
        i32.store
        local.get 0
        local.get 2
        i32.store offset=24
        local.get 0
        local.get 0
        i32.store offset=8
        local.get 0
        local.get 0
        i32.store offset=12
        return
      else
      end
      local.get 1
      i32.const 25
      local.get 3
      i32.const 1
      i32.shr_u
      i32.sub
      i32.const 0
      local.get 3
      i32.const 31
      i32.ne
      select
      i32.shl
      local.set 3
      local.get 2
      i32.load
      local.set 2
      block ;; label = @2
        loop ;; label = @3
          local.get 2
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
          local.set 2
          local.get 3
          i32.const 1
          i32.shl
          local.set 3
          local.get 4
          local.get 2
          i32.const 4
          i32.and
          i32.add
          local.tee 7
          i32.load offset=16
          local.tee 2
          br_if 0 (;@3;)
        end
        local.get 7
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
  (func (;7;) (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 11
    global.set 0
    block ;; label = @1
      local.get 2
      i32.const 8
      i32.le_u
      local.get 2
      local.get 3
      i32.le_u
      i32.and
      i32.eqz
      if ;; label = @2
        local.get 11
        i32.const 0
        i32.store offset=12
        local.get 11
        i32.const 12
        i32.add
        i32.const 4
        local.get 2
        local.get 2
        i32.const 4
        i32.le_u
        select
        local.get 3
        call 10
        br_if 1 (;@1;)
        local.get 11
        i32.load offset=12
        local.tee 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        local.get 1
        local.get 1
        local.get 3
        i32.gt_u
        select
        local.tee 1
        if ;; label = @3
          local.get 2
          local.get 0
          local.get 1
          memory.copy
        else
        end
        local.get 0
        call 5
        local.get 2
        local.set 6
        br 1 (;@1;)
      else
      end
      block (result i32) ;; label = @2
        i32.const 0
        local.set 2
        local.get 0
        i32.eqz
        if ;; label = @3
          local.get 3
          call 2
          br 1 (;@2;)
        else
        end
        local.get 3
        i32.const -64
        i32.ge_u
        if ;; label = @3
          i32.const 1052684
          i32.const 48
          i32.store
          i32.const 0
          br 1 (;@2;)
        else
        end
        i32.const 16
        local.get 3
        i32.const 19
        i32.add
        i32.const -16
        i32.and
        local.get 3
        i32.const 11
        i32.lt_u
        select
        local.set 4
        local.get 0
        i32.const 4
        i32.sub
        local.tee 8
        i32.load
        local.tee 9
        i32.const -8
        i32.and
        local.set 1
        block ;; label = @3
          block ;; label = @4
            local.get 9
            i32.const 3
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 4
              i32.const 256
              i32.lt_u
              local.get 1
              local.get 4
              i32.le_u
              i32.or
              br_if 1 (;@4;)
              local.get 1
              local.get 4
              i32.sub
              i32.const 1053172
              i32.load
              i32.const 1
              i32.shl
              i32.le_u
              br_if 2 (;@3;)
              br 1 (;@4;)
            else
            end
            local.get 0
            i32.const 8
            i32.sub
            local.tee 7
            local.get 1
            i32.add
            local.set 5
            local.get 1
            local.get 4
            i32.ge_u
            if ;; label = @5
              local.get 1
              local.get 4
              i32.sub
              local.tee 1
              i32.const 16
              i32.lt_u
              br_if 2 (;@3;)
              local.get 8
              local.get 4
              local.get 9
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 4
              local.get 7
              i32.add
              local.tee 2
              local.get 1
              i32.const 3
              i32.or
              i32.store offset=4
              local.get 5
              local.get 5
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 2
              local.get 1
              call 6
              local.get 0
              br 3 (;@2;)
            else
            end
            i32.const 1052716
            i32.load
            local.get 5
            i32.eq
            if ;; label = @5
              i32.const 1052704
              i32.load
              local.get 1
              i32.add
              local.tee 1
              local.get 4
              i32.le_u
              br_if 1 (;@4;)
              local.get 8
              local.get 4
              local.get 9
              i32.const 1
              i32.and
              i32.or
              i32.const 2
              i32.or
              i32.store
              i32.const 1052716
              local.get 4
              local.get 7
              i32.add
              local.tee 2
              i32.store
              i32.const 1052704
              local.get 1
              local.get 4
              i32.sub
              local.tee 1
              i32.store
              local.get 2
              local.get 1
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              br 3 (;@2;)
            else
            end
            i32.const 1052712
            i32.load
            local.get 5
            i32.eq
            if ;; label = @5
              i32.const 1052700
              i32.load
              local.get 1
              i32.add
              local.tee 6
              local.get 4
              i32.lt_u
              br_if 1 (;@4;)
              block ;; label = @6
                local.get 6
                local.get 4
                i32.sub
                local.tee 1
                i32.const 16
                i32.ge_u
                if ;; label = @7
                  local.get 8
                  local.get 4
                  local.get 9
                  i32.const 1
                  i32.and
                  i32.or
                  i32.const 2
                  i32.or
                  i32.store
                  local.get 4
                  local.get 7
                  i32.add
                  local.tee 2
                  local.get 1
                  i32.const 1
                  i32.or
                  i32.store offset=4
                  local.get 6
                  local.get 7
                  i32.add
                  local.tee 3
                  local.get 1
                  i32.store
                  local.get 3
                  local.get 3
                  i32.load offset=4
                  i32.const -2
                  i32.and
                  i32.store offset=4
                  br 1 (;@6;)
                else
                end
                local.get 8
                local.get 9
                i32.const 1
                i32.and
                local.get 6
                i32.or
                i32.const 2
                i32.or
                i32.store
                local.get 6
                local.get 7
                i32.add
                local.tee 1
                local.get 1
                i32.load offset=4
                i32.const 1
                i32.or
                i32.store offset=4
                i32.const 0
                local.set 1
              end
              i32.const 1052712
              local.get 2
              i32.store
              i32.const 1052700
              local.get 1
              i32.store
              local.get 0
              br 3 (;@2;)
            else
            end
            local.get 5
            i32.load offset=4
            local.tee 2
            i32.const 2
            i32.and
            br_if 0 (;@4;)
            local.get 2
            i32.const -8
            i32.and
            local.get 1
            i32.add
            local.tee 12
            local.get 4
            i32.lt_u
            br_if 0 (;@4;)
            local.get 12
            local.get 4
            i32.sub
            local.set 13
            local.get 5
            i32.load offset=12
            local.set 1
            block ;; label = @5
              local.get 2
              i32.const 255
              i32.le_u
              if ;; label = @6
                local.get 5
                i32.load offset=8
                local.tee 3
                local.get 1
                i32.eq
                if ;; label = @7
                  i32.const 1052692
                  i32.const 1052692
                  i32.load
                  i32.const -2
                  local.get 2
                  i32.const 3
                  i32.shr_u
                  i32.rotl
                  i32.and
                  i32.store
                  br 2 (;@5;)
                else
                end
                local.get 1
                local.get 3
                i32.store offset=8
                local.get 3
                local.get 1
                i32.store offset=12
                br 1 (;@5;)
              else
              end
              local.get 5
              i32.load offset=24
              local.set 10
              block ;; label = @6
                local.get 1
                local.get 5
                i32.ne
                if ;; label = @7
                  local.get 5
                  i32.load offset=8
                  local.tee 2
                  local.get 1
                  i32.store offset=12
                  local.get 1
                  local.get 2
                  i32.store offset=8
                  br 1 (;@6;)
                else
                end
                block ;; label = @7
                  local.get 5
                  i32.load offset=20
                  local.tee 3
                  if (result i32) ;; label = @8
                    local.get 5
                    i32.const 20
                    i32.add
                  else
                    local.get 5
                    i32.load offset=16
                    local.tee 3
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 5
                    i32.const 16
                    i32.add
                  end
                  local.set 2
                  loop ;; label = @8
                    local.get 2
                    local.set 6
                    local.get 3
                    local.tee 1
                    i32.const 20
                    i32.add
                    local.set 2
                    local.get 1
                    i32.load offset=20
                    local.tee 3
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 16
                    i32.add
                    local.set 2
                    local.get 1
                    i32.load offset=16
                    local.tee 3
                    br_if 0 (;@8;)
                  end
                  local.get 6
                  i32.const 0
                  i32.store
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 1
              end
              local.get 10
              i32.eqz
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 5
                i32.load offset=28
                local.tee 2
                i32.const 2
                i32.shl
                local.tee 3
                i32.load offset=1052996
                local.get 5
                i32.eq
                if ;; label = @7
                  local.get 3
                  i32.const 1052996
                  i32.add
                  local.get 1
                  i32.store
                  local.get 1
                  br_if 1 (;@6;)
                  i32.const 1052696
                  i32.const 1052696
                  i32.load
                  i32.const -2
                  local.get 2
                  i32.rotl
                  i32.and
                  i32.store
                  br 2 (;@5;)
                else
                end
                block ;; label = @7
                  local.get 5
                  local.get 10
                  i32.load offset=16
                  i32.eq
                  if ;; label = @8
                    local.get 10
                    local.get 1
                    i32.store offset=16
                    br 1 (;@7;)
                  else
                  end
                  local.get 10
                  local.get 1
                  i32.store offset=20
                end
                local.get 1
                i32.eqz
                br_if 1 (;@5;)
              end
              local.get 1
              local.get 10
              i32.store offset=24
              local.get 5
              i32.load offset=16
              local.tee 2
              if ;; label = @6
                local.get 1
                local.get 2
                i32.store offset=16
                local.get 2
                local.get 1
                i32.store offset=24
              else
              end
              local.get 5
              i32.load offset=20
              local.tee 2
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 2
              i32.store offset=20
              local.get 2
              local.get 1
              i32.store offset=24
            end
            local.get 13
            i32.const 15
            i32.le_u
            if ;; label = @5
              local.get 8
              local.get 9
              i32.const 1
              i32.and
              local.get 12
              i32.or
              i32.const 2
              i32.or
              i32.store
              local.get 7
              local.get 12
              i32.add
              local.tee 1
              local.get 1
              i32.load offset=4
              i32.const 1
              i32.or
              i32.store offset=4
              local.get 0
              br 3 (;@2;)
            else
            end
            local.get 8
            local.get 4
            local.get 9
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store
            local.get 4
            local.get 7
            i32.add
            local.tee 1
            local.get 13
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 7
            local.get 12
            i32.add
            local.tee 2
            local.get 2
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 1
            local.get 13
            call 6
            local.get 0
            br 2 (;@2;)
          end
          i32.const 0
          local.get 3
          call 2
          local.tee 1
          i32.eqz
          br_if 1 (;@2;)
          drop
          i32.const -4
          i32.const -8
          local.get 8
          i32.load
          local.tee 2
          i32.const 3
          i32.and
          select
          local.get 2
          i32.const -8
          i32.and
          i32.add
          local.tee 2
          local.get 3
          local.get 2
          local.get 3
          i32.lt_u
          select
          local.tee 2
          if ;; label = @4
            local.get 1
            local.get 0
            local.get 2
            memory.copy
          else
          end
          local.get 0
          call 5
          local.get 1
          local.set 0
        end
        local.get 0
      end
      local.set 6
    end
    local.get 11
    i32.const 16
    i32.add
    global.set 0
    local.get 6
  )
  (func (;8;) (type 10) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 5
    global.set 0
    local.get 5
    local.get 1
    i32.store offset=32
    local.get 5
    local.get 0
    i32.store offset=28
    local.get 5
    local.get 2
    i32.store offset=36
    i32.const 1052660
    i32.const 1052660
    i32.load
    local.tee 6
    i32.const 1
    i32.add
    i32.store
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block (result i32) ;; label = @6
                i32.const 0
                local.get 6
                i32.const 0
                i32.lt_s
                br_if 0 (;@6;)
                drop
                i32.const 1
                i32.const 1052620
                i32.load8_u
                br_if 0 (;@6;)
                drop
                i32.const 1052620
                i32.const 1
                i32.store8
                i32.const 1052616
                i32.const 1052616
                i32.load
                i32.const 1
                i32.add
                i32.store
                i32.const 2
              end
              i32.const 255
              i32.and
              br_table 2 (;@3;) 1 (;@4;) 0 (;@5;) 1 (;@4;)
            end
            i32.const 1052632
            i32.load
            local.tee 6
            i32.const 0
            i32.ge_s
            br_if 2 (;@2;)
            local.get 5
            i32.const 56
            i32.add
            local.get 5
            i32.const 79
            i32.add
            i32.const 1051240
            i32.const 115
            call 70
            local.get 5
            i32.load8_u offset=56
            local.get 5
            i32.load offset=60
            call 50
            br 3 (;@1;)
          end
          local.get 5
          local.get 0
          local.get 1
          i32.load offset=24
          call_indirect (type 4)
          local.get 5
          local.get 5
          i32.load offset=4
          i32.const 0
          local.get 5
          i32.load
          local.tee 0
          select
          i32.store offset=44
          local.get 5
          local.get 0
          i32.const 1
          local.get 0
          select
          i32.store offset=40
          local.get 5
          local.get 5
          i32.const 40
          i32.add
          i64.extend_i32_u
          i64.const 17179869184
          i64.or
          i64.store offset=64
          local.get 5
          local.get 5
          i32.const 36
          i32.add
          i64.extend_i32_u
          i64.const 21474836480
          i64.or
          i64.store offset=56
          local.get 5
          i32.const 48
          i32.add
          local.get 5
          i32.const 79
          i32.add
          i32.const 1049873
          local.get 5
          i32.const 56
          i32.add
          call 70
          local.get 5
          i32.load8_u offset=48
          local.get 5
          i32.load offset=52
          call 50
          br 2 (;@1;)
        end
        local.get 5
        local.get 5
        i32.const 28
        i32.add
        i64.extend_i32_u
        i64.const 25769803776
        i64.or
        i64.store offset=64
        local.get 5
        local.get 5
        i32.const 36
        i32.add
        i64.extend_i32_u
        i64.const 21474836480
        i64.or
        i64.store offset=56
        local.get 5
        i32.const 48
        i32.add
        local.get 5
        i32.const 79
        i32.add
        i32.const 1049983
        local.get 5
        i32.const 56
        i32.add
        call 70
        local.get 5
        i32.load8_u offset=48
        local.get 5
        i32.load offset=52
        call 50
        br 1 (;@1;)
      end
      i32.const 1052632
      local.get 6
      i32.const 1
      i32.add
      i32.store
      block ;; label = @2
        i32.const 1052636
        i32.load
        if ;; label = @3
          local.get 5
          i32.const 16
          i32.add
          local.get 0
          local.get 1
          i32.load offset=20
          call_indirect (type 4)
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
          i32.const 1052636
          i32.load
          local.get 5
          i32.const 56
          i32.add
          i32.const 1052640
          i32.load
          i32.load offset=20
          call_indirect (type 4)
          br 1 (;@2;)
        else
        end
        local.get 5
        i32.const 8
        i32.add
        local.get 0
        local.get 1
        i32.load offset=20
        call_indirect (type 4)
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
        global.get 0
        i32.const 48
        i32.sub
        local.tee 0
        global.set 0
        local.get 0
        block (result i32) ;; label = @3
          i32.const 3
          local.get 5
          i32.const 56
          i32.add
          local.tee 1
          i32.load8_u offset=13
          br_if 0 (;@3;)
          drop
          i32.const 1
          i32.const 1052616
          i32.load
          i32.const 1
          i32.gt_u
          br_if 0 (;@3;)
          drop
          call 3
          i32.const 255
          i32.and
        end
        i32.store8 offset=11
        local.get 0
        local.get 1
        i32.load offset=8
        i32.store offset=12
        local.get 1
        i32.load
        local.set 2
        local.get 1
        i32.load offset=4
        local.set 4
        global.get 0
        i32.const 16
        i32.sub
        local.tee 1
        global.set 0
        local.get 1
        local.get 2
        local.get 4
        i32.load offset=12
        local.tee 4
        call_indirect (type 4)
        block (result i32) ;; label = @3
          local.get 2
          local.get 1
          i64.load
          i64.const 7199936582794304877
          i64.xor
          local.get 1
          i64.load offset=8
          i64.const -5076933981314334344
          i64.xor
          i64.or
          i64.eqz
          if (result i32) ;; label = @4
            i32.const 4
          else
            local.get 1
            local.get 2
            local.get 4
            call_indirect (type 4)
            local.get 1
            i64.load
            i64.const -7788913181612638748
            i64.xor
            local.get 1
            i64.load offset=8
            i64.const -9212764535765366089
            i64.xor
            i64.or
            i64.eqz
            i32.eqz
            if ;; label = @5
              i32.const 12
              local.set 4
              i32.const 1051568
              br 2 (;@3;)
            else
            end
            local.get 2
            i32.const 4
            i32.add
            local.set 2
            i32.const 8
          end
          i32.add
          i32.load
          local.set 4
          local.get 2
          i32.load
        end
        local.set 2
        local.get 0
        local.get 4
        i32.store offset=4
        local.get 0
        local.get 2
        i32.store
        local.get 1
        i32.const 16
        i32.add
        global.set 0
        local.get 0
        local.get 0
        i64.load
        i64.store offset=16 align=4
        local.get 0
        local.get 0
        i32.const 11
        i32.add
        i32.store offset=32
        local.get 0
        local.get 0
        i32.const 16
        i32.add
        i32.store offset=28
        local.get 0
        local.get 0
        i32.const 12
        i32.add
        i32.store offset=24
        block ;; label = @3
          block ;; label = @4
            i32.const 1052644
            i32.load8_u
            if ;; label = @5
              i32.const 1052644
              i32.const 1
              i32.store8
              i32.const 1052612
              i32.load
              local.set 2
              i32.const 1052612
              i32.const 0
              i32.store
              local.get 2
              br_if 1 (;@4;)
            else
            end
            local.get 0
            i32.const 24
            i32.add
            local.get 0
            i32.const 47
            i32.add
            i32.const 1051488
            call 26
            br 1 (;@3;)
          end
          global.get 0
          i32.const 16
          i32.sub
          local.tee 1
          global.set 0
          local.get 2
          i32.const 8
          i32.add
          local.tee 4
          i32.load8_u
          local.set 6
          local.get 4
          i32.const 1
          i32.store8
          local.get 1
          local.get 6
          i32.store8 offset=15
          local.get 6
          i32.const 1
          i32.eq
          if ;; label = @4
            local.get 1
            i32.const 15
            i32.add
            call 35
            unreachable
          else
          end
          local.get 1
          i32.const 16
          i32.add
          global.set 0
          local.get 0
          i32.const 24
          i32.add
          local.get 4
          i32.const 4
          i32.add
          i32.const 1051528
          call 26
          local.get 4
          i32.const 0
          i32.store8
          i32.const 1052644
          i32.const 1
          i32.store8
          i32.const 1052612
          i32.load
          local.set 1
          i32.const 1052612
          local.get 2
          i32.store
          local.get 0
          local.get 1
          i32.store offset=40
          local.get 0
          i32.const 1
          i32.store offset=36
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 1
          i32.load
          local.tee 1
          i32.const 1
          i32.sub
          i32.store
          local.get 1
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.const 40
          i32.add
          i32.load
          local.tee 1
          i32.const 12
          i32.add
          i32.load
          if ;; label = @4
            local.get 1
            i32.const 16
            i32.add
            i32.load
            call 5
          else
          end
          block ;; label = @4
            local.get 1
            i32.const -1
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            local.get 1
            i32.load offset=4
            local.tee 2
            i32.const 1
            i32.sub
            i32.store offset=4
            local.get 2
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            call 5
          end
        end
        local.get 0
        i32.const 48
        i32.add
        global.set 0
      end
      i32.const 1052632
      i32.const 1052632
      i32.load
      i32.const 1
      i32.sub
      i32.store
      i32.const 1052620
      i32.const 0
      i32.store8
      local.get 3
      i32.eqz
      if ;; label = @2
        local.get 5
        i32.const 56
        i32.add
        local.get 5
        i32.const 79
        i32.add
        i32.const 1051580
        i32.const 91
        call 70
        local.get 5
        i32.load8_u offset=56
        local.get 5
        i32.load offset=60
        call 50
        br 1 (;@1;)
      else
      end
      global.get 0
      i32.const 32
      i32.sub
      global.set 0
      unreachable
    end
    unreachable
  )
  (func (;9;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i64 i64 i64 i64)
    global.get 0
    i32.const 592
    i32.sub
    local.tee 2
    global.set 0
    block ;; label = @1
      local.get 1
      if ;; label = @2
        local.get 1
        i32.load
        local.tee 3
        i32.load offset=16
        local.tee 1
        if ;; label = @3
          local.get 3
          i32.const 20
          i32.add
          i32.load
          i32.const 1
          i32.sub
          local.set 3
          br 2 (;@1;)
        else
        end
        i32.const 0
        local.set 1
        i32.const 1052648
        i64.load
        local.tee 7
        i64.eqz
        br_if 1 (;@1;)
        i32.const 1051016
        i32.const 0
        local.get 7
        local.get 3
        i64.load offset=8
        i64.eq
        select
        local.set 1
        i32.const 4
        local.set 3
        br 1 (;@1;)
      else
      end
      i32.const 0
      local.set 1
      i32.const 1052648
      i64.load
      local.tee 7
      i64.eqz
      br_if 0 (;@1;)
      i32.const 1051016
      i32.const 0
      i32.const 1052664
      i64.load
      local.get 7
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
    i32.const 1051020
    local.get 1
    select
    i32.store offset=8
    block ;; label = @1
      i32.const 1052664
      i64.load
      local.tee 9
      i64.eqz
      if ;; label = @2
        i32.const 1052672
        i64.load
        local.set 7
        loop ;; label = @3
          local.get 7
          i64.const -1
          i64.eq
          br_if 2 (;@1;)
          i32.const 1052672
          local.get 7
          i64.const 1
          i64.add
          local.tee 9
          i32.const 1052672
          i64.load
          local.tee 8
          local.get 7
          local.get 8
          i64.eq
          local.tee 1
          select
          i64.store
          local.get 8
          local.set 7
          local.get 1
          i32.eqz
          br_if 0 (;@3;)
        end
        i32.const 1052664
        local.get 9
        i64.store
      else
      end
      local.get 2
      local.get 9
      i64.store offset=16
      local.get 2
      i32.const 24
      i32.add
      local.tee 1
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
      local.set 7
      local.get 2
      local.get 1
      i32.store offset=536
      local.get 0
      i64.load32_u
      local.set 8
      local.get 2
      local.get 7
      i64.const 17179869184
      i64.or
      local.tee 7
      i64.store offset=584
      local.get 2
      local.get 8
      i64.const 21474836480
      i64.or
      local.tee 8
      i64.store offset=576
      local.get 2
      local.get 2
      i32.const 16
      i32.add
      i64.extend_i32_u
      i64.const 30064771072
      i64.or
      local.tee 9
      i64.store offset=568
      local.get 2
      local.get 2
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.const 17179869184
      i64.or
      local.tee 10
      i64.store offset=560
      global.get 0
      i32.const 16
      i32.sub
      local.tee 1
      global.set 0
      local.get 2
      i32.const 552
      i32.add
      local.tee 3
      i32.const 4
      i32.store8
      local.get 1
      local.get 2
      i32.const 536
      i32.add
      i32.store offset=8
      local.get 1
      local.get 3
      i64.load align=4
      i64.store
      local.get 1
      i32.const 1050776
      i32.const 1049944
      local.get 2
      i32.const 560
      i32.add
      call 12
      local.set 5
      local.get 1
      i32.load8_u
      local.set 4
      block ;; label = @2
        block ;; label = @3
          local.get 5
          if ;; label = @4
            local.get 4
            i32.const 4
            i32.ne
            br_if 1 (;@3;)
            i32.const 1050800
            i32.const 173
            i32.const 1050888
            call 32
            unreachable
          else
          end
          local.get 4
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.load offset=4
          local.tee 3
          i32.load
          local.set 4
          local.get 3
          i32.const 4
          i32.add
          i32.load
          local.tee 5
          i32.load
          local.tee 6
          if ;; label = @4
            local.get 4
            local.get 6
            call_indirect (type 2)
          else
          end
          local.get 5
          i32.load offset=4
          if ;; label = @4
            local.get 5
            i32.load offset=8
            drop
            local.get 4
            call 5
          else
          end
          local.get 3
          call 5
          br 1 (;@2;)
        end
        local.get 3
        local.get 1
        i64.load
        i64.store align=4
      end
      local.get 1
      i32.const 16
      i32.add
      global.set 0
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.load8_u offset=552
          local.tee 1
          i32.const 4
          i32.eq
          if ;; label = @4
            local.get 2
            i32.load offset=544
            local.tee 1
            i32.const 513
            i32.lt_u
            br_if 1 (;@3;)
            i32.const 0
            local.get 1
            i32.const 512
            i32.const 1051032
            call 27
            unreachable
          else
          end
          local.get 1
          i32.const 3
          i32.eq
          if ;; label = @4
            local.get 2
            i32.load offset=556
            local.tee 1
            i32.load
            local.set 3
            local.get 1
            i32.const 4
            i32.add
            i32.load
            local.tee 4
            i32.load
            local.tee 5
            if ;; label = @5
              local.get 3
              local.get 5
              call_indirect (type 2)
            else
            end
            local.get 4
            i32.load offset=4
            if ;; label = @5
              local.get 4
              i32.load offset=8
              drop
              local.get 3
              call 5
            else
            end
            local.get 1
            call 5
          else
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
          local.get 7
          i64.store offset=584
          local.get 2
          local.get 8
          i64.store offset=576
          local.get 2
          local.get 9
          i64.store offset=568
          local.get 2
          local.get 10
          i64.store offset=560
          local.get 2
          i32.const 552
          i32.add
          local.get 0
          i32.const 1049944
          local.get 2
          i32.const 560
          i32.add
          local.get 1
          call_indirect (type 8)
          local.get 2
          i32.load8_u offset=552
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 2
          i32.load offset=556
          local.tee 0
          i32.load
          local.set 1
          local.get 0
          i32.const 4
          i32.add
          i32.load
          local.tee 3
          i32.load
          local.tee 4
          if ;; label = @4
            local.get 1
            local.get 4
            call_indirect (type 2)
          else
          end
          local.get 3
          i32.load offset=4
          if ;; label = @4
            local.get 3
            i32.load offset=8
            drop
            local.get 1
            call 5
          else
          end
          local.get 0
          call 5
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
        call_indirect (type 8)
        local.get 2
        i32.load8_u offset=560
        i32.const 3
        i32.ne
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=564
        local.tee 0
        i32.load
        local.set 1
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.load
        local.tee 4
        if ;; label = @3
          local.get 1
          local.get 4
          call_indirect (type 2)
        else
        end
        local.get 3
        i32.load offset=4
        if ;; label = @3
          local.get 3
          i32.load offset=8
          drop
          local.get 1
          call 5
        else
        end
        local.get 0
        call 5
      end
      local.get 2
      i32.const 592
      i32.add
      global.set 0
      return
    end
    i32.const 1051785
    i32.const 111
    i32.const 1051840
    call 32
    unreachable
  )
  (func (;10;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block ;; label = @1
      block (result i32) ;; label = @2
        local.get 1
        i32.const 16
        i32.eq
        if ;; label = @3
          local.get 2
          call 2
          br 1 (;@2;)
        else
        end
        i32.const 28
        local.set 4
        local.get 1
        i32.const 3
        i32.and
        local.get 1
        i32.const 4
        i32.lt_u
        i32.or
        br_if 1 (;@1;)
        local.get 1
        i32.const 2
        i32.shr_u
        local.tee 3
        local.get 3
        i32.const 1
        i32.sub
        i32.and
        br_if 1 (;@1;)
        i32.const -64
        local.get 1
        i32.sub
        local.get 2
        i32.lt_u
        if ;; label = @3
          i32.const 48
          return
        else
        end
        block (result i32) ;; label = @3
          block ;; label = @4
            i32.const 16
            i32.const 16
            local.get 1
            local.get 1
            i32.const 16
            i32.le_u
            select
            local.tee 1
            local.get 1
            i32.const 16
            i32.le_u
            select
            local.tee 4
            local.get 4
            i32.const 1
            i32.sub
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 4
              local.set 1
              br 1 (;@4;)
            else
            end
            i32.const 32
            local.set 3
            loop ;; label = @5
              local.get 3
              local.tee 1
              i32.const 1
              i32.shl
              local.set 3
              local.get 1
              local.get 4
              i32.lt_u
              br_if 0 (;@5;)
            end
          end
          i32.const -64
          local.get 1
          i32.sub
          local.get 2
          i32.le_u
          if ;; label = @4
            i32.const 1052684
            i32.const 48
            i32.store
            i32.const 0
            br 1 (;@3;)
          else
          end
          i32.const 0
          local.get 1
          i32.const 16
          local.get 2
          i32.const 19
          i32.add
          i32.const -16
          i32.and
          local.get 2
          i32.const 11
          i32.lt_u
          select
          local.tee 4
          i32.add
          i32.const 12
          i32.add
          call 2
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          drop
          local.get 3
          i32.const 8
          i32.sub
          local.set 2
          block ;; label = @4
            local.get 1
            i32.const 1
            i32.sub
            local.get 3
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 2
              local.set 1
              br 1 (;@4;)
            else
            end
            local.get 3
            i32.const 4
            i32.sub
            local.tee 6
            i32.load
            local.tee 7
            i32.const -8
            i32.and
            local.get 1
            local.get 3
            i32.add
            i32.const 1
            i32.sub
            i32.const 0
            local.get 1
            i32.sub
            i32.and
            i32.const 8
            i32.sub
            local.tee 3
            local.get 1
            i32.const 0
            local.get 3
            local.get 2
            i32.sub
            i32.const 15
            i32.le_u
            select
            i32.add
            local.tee 1
            local.get 2
            i32.sub
            local.tee 3
            i32.sub
            local.set 5
            local.get 7
            i32.const 3
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 1
              local.get 5
              i32.store offset=4
              local.get 1
              local.get 2
              i32.load
              local.get 3
              i32.add
              i32.store
              br 1 (;@4;)
            else
            end
            local.get 1
            local.get 5
            local.get 1
            i32.load offset=4
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store offset=4
            local.get 1
            local.get 5
            i32.add
            local.tee 5
            local.get 5
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 6
            local.get 3
            local.get 6
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
            local.tee 5
            local.get 5
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 3
            call 6
          end
          block ;; label = @4
            local.get 1
            i32.load offset=4
            local.tee 2
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const -8
            i32.and
            local.tee 3
            local.get 4
            i32.const 16
            i32.add
            i32.le_u
            br_if 0 (;@4;)
            local.get 1
            local.get 4
            local.get 2
            i32.const 1
            i32.and
            i32.or
            i32.const 2
            i32.or
            i32.store offset=4
            local.get 1
            local.get 4
            i32.add
            local.tee 2
            local.get 3
            local.get 4
            i32.sub
            local.tee 4
            i32.const 3
            i32.or
            i32.store offset=4
            local.get 1
            local.get 3
            i32.add
            local.tee 3
            local.get 3
            i32.load offset=4
            i32.const 1
            i32.or
            i32.store offset=4
            local.get 2
            local.get 4
            call 6
          end
          local.get 1
          i32.const 8
          i32.add
        end
      end
      local.tee 1
      i32.eqz
      if ;; label = @2
        i32.const 48
        return
      else
      end
      local.get 0
      local.get 1
      i32.store
      i32.const 0
      local.set 4
    end
    local.get 4
  )
  (func (;11;) (type 8) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 7
    global.set 0
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 4
          i32.add
          local.set 4
          local.get 3
          i32.const 3
          i32.shl
          local.tee 5
          i32.const 8
          i32.sub
          i32.const 3
          i32.shr_u
          i32.const 1
          i32.add
          local.set 6
          i32.const 0
          local.set 1
          block ;; label = @4
            loop ;; label = @5
              local.get 4
              i32.load
              br_if 1 (;@4;)
              local.get 4
              i32.const 8
              i32.add
              local.set 4
              local.get 1
              i32.const 1
              i32.add
              local.set 1
              local.get 5
              i32.const 8
              i32.sub
              local.tee 5
              br_if 0 (;@5;)
            end
            local.get 6
            local.set 1
          end
          block ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 3
              i32.le_u
              if ;; label = @6
                local.get 1
                local.get 3
                i32.eq
                br_if 3 (;@3;)
                local.get 3
                local.get 1
                i32.sub
                local.set 6
                local.get 2
                local.get 1
                i32.const 3
                i32.shl
                i32.add
                local.set 8
                loop ;; label = @7
                  local.get 8
                  i32.const 16
                  local.get 6
                  local.get 6
                  i32.const 16
                  i32.ge_u
                  select
                  call 45
                  local.tee 4
                  i32.const -1
                  i32.eq
                  if ;; label = @8
                    local.get 7
                    i32.const 0
                    i32.store8 offset=11
                    local.get 7
                    i32.const 0
                    i32.store16 offset=9 align=1
                    local.get 7
                    i32.const 0
                    i32.store8 offset=8
                    local.get 7
                    i32.const 1052684
                    i32.load
                    local.tee 1
                    i32.store offset=12
                    local.get 1
                    i32.const 27
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 7
                    i32.const 8
                    i32.add
                    local.set 4
                    br 6 (;@2;)
                  else
                  end
                  local.get 7
                  local.get 4
                  i32.store offset=12
                  local.get 7
                  i32.const 4
                  i32.store8 offset=8
                  local.get 4
                  i32.eqz
                  if ;; label = @8
                    i32.const 1051672
                    local.set 4
                    br 6 (;@2;)
                  else
                  end
                  local.get 8
                  i32.const 4
                  i32.add
                  local.set 1
                  local.get 6
                  i32.const 3
                  i32.shl
                  local.tee 3
                  i32.const 8
                  i32.sub
                  i32.const 3
                  i32.shr_u
                  i32.const 1
                  i32.add
                  local.set 2
                  i32.const 0
                  local.set 5
                  block ;; label = @8
                    loop ;; label = @9
                      local.get 4
                      local.get 1
                      i32.load
                      local.tee 9
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 1
                      i32.const 8
                      i32.add
                      local.set 1
                      local.get 5
                      i32.const 1
                      i32.add
                      local.set 5
                      local.get 4
                      local.get 9
                      i32.sub
                      local.set 4
                      local.get 3
                      i32.const 8
                      i32.sub
                      local.tee 3
                      br_if 0 (;@9;)
                    end
                    local.get 2
                    local.set 5
                  end
                  local.get 5
                  local.get 6
                  i32.gt_u
                  br_if 2 (;@5;)
                  local.get 5
                  local.get 6
                  i32.eq
                  if ;; label = @8
                    local.get 4
                    i32.eqz
                    br_if 5 (;@3;)
                    i32.const 1051360
                    i32.const 79
                    i32.const 1051400
                    call 32
                    unreachable
                  else
                  end
                  local.get 8
                  local.get 5
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 8
                  i32.load offset=4
                  local.tee 1
                  local.get 4
                  i32.lt_u
                  br_if 3 (;@4;)
                  local.get 6
                  local.get 5
                  i32.sub
                  local.set 6
                  local.get 8
                  local.get 1
                  local.get 4
                  i32.sub
                  i32.store offset=4
                  local.get 8
                  local.get 8
                  i32.load
                  local.get 4
                  i32.add
                  i32.store
                  local.get 7
                  i32.load8_u offset=8
                  local.tee 1
                  i32.const 4
                  i32.eq
                  local.get 1
                  i32.const 3
                  i32.ne
                  i32.or
                  br_if 0 (;@7;)
                  local.get 7
                  i32.load offset=12
                  local.tee 1
                  i32.load
                  local.set 2
                  local.get 1
                  i32.const 4
                  i32.add
                  i32.load
                  local.tee 3
                  i32.load
                  local.tee 5
                  if ;; label = @8
                    local.get 2
                    local.get 5
                    call_indirect (type 2)
                  else
                  end
                  local.get 3
                  i32.load offset=4
                  if ;; label = @8
                    local.get 3
                    i32.load offset=8
                    drop
                    local.get 2
                    call 5
                  else
                  end
                  local.get 1
                  call 5
                  br 0 (;@7;)
                end
                unreachable
              else
              end
              local.get 1
              local.get 3
              local.get 3
              i32.const 1051468
              call 27
              unreachable
            end
            local.get 5
            local.get 6
            local.get 6
            i32.const 1051468
            call 27
            unreachable
          end
          i32.const 1051416
          i32.const 71
          i32.const 1051452
          call 32
          unreachable
        end
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i64.load
      i64.store align=4
    end
    local.get 7
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;12;) (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 6
    global.set 0
    block (result i32) ;; label = @1
      block ;; label = @2
        local.get 3
        i32.const 1
        i32.and
        i32.eqz
        if ;; label = @3
          local.get 2
          i32.load8_u
          local.tee 4
          br_if 1 (;@2;)
          i32.const 0
          br 2 (;@1;)
        else
        end
        local.get 0
        local.get 2
        local.get 3
        i32.const 1
        i32.shr_u
        local.get 1
        i32.load offset=12
        call_indirect (type 7)
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=12
      local.set 10
      loop ;; label = @2
        local.get 2
        i32.const 1
        i32.add
        local.set 5
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.extend8_s
                i32.const 0
                i32.lt_s
                if ;; label = @7
                  local.get 4
                  i32.const 128
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 4
                  i32.const 192
                  i32.ne
                  br_if 3 (;@4;)
                  local.get 6
                  local.get 1
                  i32.store offset=4
                  local.get 6
                  local.get 0
                  i32.store
                  local.get 6
                  i64.const 1610612768
                  i64.store offset=8 align=4
                  local.get 3
                  local.get 7
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 2
                  i32.load
                  local.get 6
                  local.get 2
                  i32.load offset=4
                  call_indirect (type 5)
                  i32.eqz
                  br_if 2 (;@5;)
                  i32.const 1
                  br 6 (;@1;)
                else
                end
                local.get 0
                local.get 5
                local.get 4
                local.get 10
                call_indirect (type 7)
                i32.eqz
                if ;; label = @7
                  local.get 4
                  local.get 5
                  i32.add
                  local.set 2
                  br 4 (;@3;)
                else
                end
                i32.const 1
                br 5 (;@1;)
              end
              local.get 0
              local.get 2
              i32.const 3
              i32.add
              local.tee 5
              local.get 2
              i32.load16_u offset=1 align=1
              local.tee 2
              local.get 10
              call_indirect (type 7)
              i32.eqz
              if ;; label = @6
                local.get 2
                local.get 5
                i32.add
                local.set 2
                br 3 (;@3;)
              else
              end
              i32.const 1
              br 4 (;@1;)
            end
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 5
            local.set 2
            br 1 (;@3;)
          end
          i32.const 1610612768
          local.set 11
          local.get 4
          i32.const 1
          i32.and
          if ;; label = @4
            local.get 2
            i32.load offset=1 align=1
            local.set 11
            local.get 2
            i32.const 5
            i32.add
            local.set 5
          else
          end
          i32.const 0
          local.set 9
          block (result i32) ;; label = @4
            local.get 4
            i32.const 2
            i32.and
            i32.eqz
            if ;; label = @5
              i32.const 0
              local.set 8
              local.get 5
              br 1 (;@4;)
            else
            end
            local.get 5
            i32.load16_u align=1
            local.set 8
            local.get 5
            i32.const 2
            i32.add
          end
          local.set 2
          local.get 4
          i32.const 4
          i32.and
          if ;; label = @4
            local.get 2
            i32.load16_u align=1
            local.set 9
            local.get 2
            i32.const 2
            i32.add
            local.set 2
          else
          end
          local.get 4
          i32.const 8
          i32.and
          if ;; label = @4
            local.get 2
            i32.load16_u align=1
            local.set 7
            local.get 2
            i32.const 2
            i32.add
            local.set 2
          else
          end
          local.get 4
          i32.const 16
          i32.and
          if ;; label = @4
            local.get 3
            local.get 8
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
            local.set 8
          else
          end
          local.get 6
          local.get 4
          i32.const 32
          i32.and
          if (result i32) ;; label = @4
            local.get 3
            local.get 9
            i32.const 3
            i32.shl
            i32.add
            i32.load16_u offset=4
          else
            local.get 9
          end
          i32.store16 offset=14
          local.get 6
          local.get 8
          i32.store16 offset=12
          local.get 6
          local.get 11
          i32.store offset=8
          local.get 6
          local.get 1
          i32.store offset=4
          local.get 6
          local.get 0
          i32.store
          i32.const 1
          local.get 3
          local.get 7
          i32.const 3
          i32.shl
          i32.add
          local.tee 5
          i32.load
          local.get 6
          local.get 5
          i32.load offset=4
          call_indirect (type 5)
          br_if 2 (;@1;)
          drop
          local.get 7
          i32.const 1
          i32.add
          local.set 7
        end
        local.get 2
        i32.load8_u
        local.tee 4
        br_if 0 (;@2;)
      end
      i32.const 0
    end
    local.get 6
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;13;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64)
    i32.const 43
    i32.const 1114112
    local.get 0
    i32.load offset=8
    local.tee 4
    i32.const 2097152
    i32.and
    local.tee 3
    select
    local.set 9
    local.get 4
    i32.const 8388608
    i32.and
    i32.eqz
    i32.eqz
    local.set 10
    block ;; label = @1
      local.get 3
      i32.const 21
      i32.shr_u
      local.get 2
      i32.add
      local.tee 3
      local.get 0
      i32.load16_u offset=12
      local.tee 7
      i32.lt_u
      if ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 4
            i32.const 16777216
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 7
              local.get 3
              i32.sub
              local.set 7
              i32.const 0
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 4
                    i32.const 29
                    i32.shr_u
                    i32.const 3
                    i32.and
                    i32.const 1
                    i32.sub
                    br_table 0 (;@8;) 1 (;@7;) 0 (;@8;) 2 (;@6;)
                  end
                  local.get 7
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 7
                i32.const 65534
                i32.and
                i32.const 1
                i32.shr_u
                local.set 3
              end
              local.get 4
              i32.const 2097151
              i32.and
              local.set 8
              local.get 0
              i32.load offset=4
              local.set 6
              local.get 0
              i32.load
              local.set 0
              loop ;; label = @6
                local.get 5
                i32.const 65535
                i32.and
                local.get 3
                i32.const 65535
                i32.and
                i32.ge_u
                br_if 2 (;@4;)
                i32.const 1
                local.set 4
                local.get 5
                i32.const 1
                i32.add
                local.set 5
                local.get 0
                local.get 8
                local.get 6
                i32.load offset=16
                call_indirect (type 5)
                i32.eqz
                br_if 0 (;@6;)
              end
              br 4 (;@1;)
            else
            end
            local.get 0
            local.get 0
            i64.load offset=8 align=4
            local.tee 11
            i32.wrap_i64
            i32.const -1612709888
            i32.and
            i32.const 536870960
            i32.or
            i32.store offset=8
            i32.const 1
            local.set 4
            local.get 0
            i32.load
            local.tee 6
            local.get 0
            i32.load offset=4
            local.tee 8
            local.get 9
            local.get 10
            call 56
            br_if 3 (;@1;)
            local.get 7
            local.get 3
            i32.sub
            i32.const 65535
            i32.and
            local.set 3
            loop ;; label = @5
              local.get 5
              i32.const 65535
              i32.and
              local.get 3
              i32.ge_u
              br_if 2 (;@3;)
              local.get 5
              i32.const 1
              i32.add
              local.set 5
              local.get 6
              i32.const 48
              local.get 8
              i32.load offset=16
              call_indirect (type 5)
              i32.eqz
              br_if 0 (;@5;)
            end
            br 3 (;@1;)
          end
          i32.const 1
          local.set 4
          local.get 0
          local.get 6
          local.get 9
          local.get 10
          call 56
          br_if 2 (;@1;)
          local.get 0
          local.get 1
          local.get 2
          local.get 6
          i32.load offset=12
          call_indirect (type 7)
          br_if 2 (;@1;)
          i32.const 0
          local.set 5
          local.get 7
          local.get 3
          i32.sub
          i32.const 65535
          i32.and
          local.set 1
          loop ;; label = @4
            local.get 5
            i32.const 65535
            i32.and
            local.tee 2
            local.get 1
            i32.lt_u
            local.set 4
            local.get 1
            local.get 2
            i32.le_u
            br_if 3 (;@1;)
            local.get 5
            i32.const 1
            i32.add
            local.set 5
            local.get 0
            local.get 8
            local.get 6
            i32.load offset=16
            call_indirect (type 5)
            i32.eqz
            br_if 0 (;@4;)
          end
          br 2 (;@1;)
        end
        local.get 6
        local.get 1
        local.get 2
        local.get 8
        i32.load offset=12
        call_indirect (type 7)
        br_if 1 (;@1;)
        local.get 0
        local.get 11
        i64.store offset=8 align=4
        i32.const 0
        return
      else
      end
      i32.const 1
      local.set 4
      local.get 0
      i32.load
      local.tee 3
      local.get 0
      i32.load offset=4
      local.tee 0
      local.get 9
      local.get 10
      call 56
      br_if 0 (;@1;)
      local.get 3
      local.get 1
      local.get 2
      local.get 0
      i32.load offset=12
      call_indirect (type 7)
      local.set 4
    end
    local.get 4
  )
  (func (;14;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 5
    global.set 0
    local.get 1
    i32.load offset=4
    local.set 7
    local.get 1
    i32.load
    local.set 6
    local.get 0
    i32.load8_u
    local.set 8
    local.get 5
    i32.const 4
    i32.add
    local.set 4
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    i32.const 512
    local.set 1
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 512
          i32.const 1
          call 42
          local.tee 2
          if ;; label = @4
            local.get 0
            local.get 2
            i32.store offset=8
            local.get 0
            i32.const 512
            i32.store offset=4
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i32.const 512
                call 15
                i32.eqz
                if ;; label = @7
                  loop ;; label = @8
                    i32.const 1052684
                    i32.load
                    local.tee 3
                    i32.const 68
                    i32.ne
                    br_if 2 (;@6;)
                    local.get 0
                    local.get 1
                    i32.store offset=12
                    local.get 0
                    i32.const 4
                    i32.add
                    local.get 1
                    i32.const 1
                    call 25
                    local.get 0
                    i32.load offset=8
                    local.tee 2
                    local.get 0
                    i32.load offset=4
                    local.tee 1
                    call 15
                    i32.eqz
                    br_if 0 (;@8;)
                  end
                else
                end
                local.get 0
                local.get 2
                call 28
                local.tee 3
                i32.store offset=12
                local.get 1
                local.get 3
                i32.le_u
                br_if 4 (;@2;)
                local.get 3
                br_if 1 (;@5;)
                i32.const 1
                local.set 1
                local.get 2
                call 5
                br 3 (;@3;)
              end
              local.get 4
              local.get 3
              i32.store offset=8
              local.get 4
              i64.const 2147483648
              i64.store align=4
              local.get 1
              i32.eqz
              br_if 4 (;@1;)
              local.get 2
              call 5
              br 4 (;@1;)
            end
            local.get 2
            local.get 1
            i32.const 1
            local.get 3
            call 7
            local.tee 1
            br_if 1 (;@3;)
            i32.const 1
            local.get 3
            call 61
            unreachable
          else
          end
          i32.const 1
          i32.const 512
          call 61
          unreachable
        end
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 0
        local.get 1
        i32.store offset=8
      end
      local.get 4
      local.get 0
      i32.load offset=12
      i32.store offset=8
      local.get 4
      local.get 0
      i64.load offset=4 align=4
      i64.store align=4
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0
    local.get 5
    i32.load offset=4
    local.tee 0
    i32.const -2147483648
    i32.ne
    local.get 5
    i64.load offset=8 align=4
    local.tee 9
    i64.const 255
    i64.and
    i64.const 3
    i64.ne
    i32.or
    i32.eqz
    if ;; label = @1
      local.get 9
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.tee 1
      i32.load
      local.set 2
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 3
      i32.load
      local.tee 4
      if ;; label = @2
        local.get 2
        local.get 4
        call_indirect (type 2)
      else
      end
      local.get 3
      i32.load offset=4
      if ;; label = @2
        local.get 3
        i32.load offset=8
        drop
        local.get 2
        call 5
      else
      end
      local.get 1
      call 5
    else
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 6
          i32.const 1051680
          i32.const 17
          local.get 7
          i32.load offset=12
          local.tee 1
          call_indirect (type 7)
          i32.eqz
          if ;; label = @4
            local.get 8
            i32.const 1
            i32.and
            br_if 1 (;@3;)
            local.get 6
            i32.const 1051697
            i32.const 88
            local.get 1
            call_indirect (type 7)
            i32.eqz
            br_if 1 (;@3;)
          else
          end
          i32.const 1
          local.set 1
          local.get 0
          i32.const 0
          i32.gt_s
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        local.set 1
        local.get 0
        i32.const 0
        i32.le_s
        br_if 1 (;@1;)
      end
      local.get 9
      i32.wrap_i64
      call 5
    end
    local.get 5
    i32.const 16
    i32.add
    global.set 0
    local.get 1
  )
  (func (;15;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 1052600
    i32.load
    local.set 3
    block ;; label = @1
      local.get 0
      i32.eqz
      if ;; label = @2
        local.get 3
        call 28
        i32.const 1
        i32.add
        local.tee 1
        call 2
        local.tee 0
        i32.eqz
        local.get 1
        i32.eqz
        i32.or
        i32.eqz
        if ;; label = @3
          local.get 0
          local.get 3
          local.get 1
          memory.copy
        else
        end
        local.get 0
        br_if 1 (;@1;)
        i32.const 1052684
        i32.const 48
        i32.store
        i32.const 0
        return
      else
      end
      local.get 3
      call 28
      i32.const 1
      i32.add
      local.get 1
      i32.gt_u
      if ;; label = @2
        i32.const 1052684
        i32.const 68
        i32.store
        i32.const 0
        return
      else
      end
      block ;; label = @2
        block ;; label = @3
          local.get 3
          local.get 0
          local.tee 1
          i32.xor
          i32.const 3
          i32.and
          if ;; label = @4
            local.get 3
            i32.load8_u
            local.set 2
            br 1 (;@3;)
          else
          end
          block ;; label = @4
            local.get 3
            i32.const 3
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.load8_u
            local.tee 2
            i32.store8
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            i32.const 1
            i32.add
            local.set 2
            local.get 3
            i32.const 1
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 2
              local.set 1
              local.get 4
              local.set 3
              br 1 (;@4;)
            else
            end
            local.get 2
            local.get 4
            i32.load8_u
            local.tee 2
            i32.store8
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            i32.const 2
            i32.add
            local.set 2
            local.get 3
            i32.const 2
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 2
              local.set 1
              local.get 4
              local.set 3
              br 1 (;@4;)
            else
            end
            local.get 2
            local.get 4
            i32.load8_u
            local.tee 2
            i32.store8
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            i32.const 3
            i32.add
            local.set 2
            local.get 3
            i32.const 3
            i32.add
            local.tee 4
            i32.const 3
            i32.and
            i32.eqz
            if ;; label = @5
              local.get 2
              local.set 1
              local.get 4
              local.set 3
              br 1 (;@4;)
            else
            end
            local.get 2
            local.get 4
            i32.load8_u
            local.tee 2
            i32.store8
            local.get 2
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 3
            i32.const 4
            i32.add
            local.set 3
          end
          i32.const 16843008
          local.get 3
          i32.load
          local.tee 2
          i32.sub
          local.get 2
          i32.or
          i32.const -2139062144
          i32.and
          i32.const -2139062144
          i32.ne
          br_if 0 (;@3;)
          loop ;; label = @4
            local.get 1
            local.get 2
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            i32.const 16843008
            local.get 3
            i32.const 4
            i32.add
            local.tee 3
            i32.load
            local.tee 2
            i32.sub
            local.get 2
            i32.or
            i32.const -2139062144
            i32.and
            i32.const -2139062144
            i32.eq
            br_if 0 (;@4;)
          end
        end
        local.get 1
        local.get 2
        i32.store8
        local.get 2
        i32.const 255
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1
        i32.add
        local.set 2
        loop ;; label = @3
          local.get 1
          i32.const 1
          i32.add
          local.tee 1
          local.get 2
          i32.load8_u
          local.tee 3
          i32.store8
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 3
          br_if 0 (;@3;)
        end
      end
    end
    local.get 0
  )
  (func (;16;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block (result i32) ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if ;; label = @2
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 4
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 3
        local.get 1
        i32.const 2048
        i32.lt_u
        if ;; label = @3
          local.get 2
          local.get 4
          i32.store8 offset=13
          local.get 2
          local.get 3
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          br 2 (;@1;)
        else
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 5
        local.get 3
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 65535
        i32.le_u
        if ;; label = @3
          local.get 2
          local.get 4
          i32.store8 offset=14
          local.get 2
          local.get 3
          i32.store8 offset=13
          local.get 2
          local.get 5
          i32.const 224
          i32.or
          i32.store8 offset=12
          i32.const 3
          br 2 (;@1;)
        else
        end
        local.get 2
        local.get 4
        i32.store8 offset=15
        local.get 2
        local.get 3
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
        br 1 (;@1;)
      else
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
    end
    local.set 1
    i32.const 0
    local.set 3
    local.get 0
    i32.load offset=8
    local.tee 4
    i32.load offset=4
    local.tee 5
    i64.const 4294967295
    local.get 4
    i64.load offset=8
    local.tee 8
    local.get 8
    i64.const 4294967295
    i64.ge_u
    select
    i32.wrap_i64
    i32.sub
    local.tee 6
    i32.const 0
    local.get 5
    local.get 6
    i32.ge_u
    select
    local.tee 6
    local.get 1
    local.get 1
    local.get 6
    i32.gt_u
    select
    local.tee 7
    if ;; label = @1
      local.get 4
      i32.load
      local.get 8
      local.get 5
      i64.extend_i32_u
      local.tee 9
      local.get 8
      local.get 9
      i64.lt_u
      select
      i32.wrap_i64
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 7
      memory.copy
    else
    end
    local.get 4
    local.get 8
    local.get 7
    i64.extend_i32_u
    i64.add
    i64.store offset=8
    block ;; label = @1
      local.get 1
      local.get 6
      i32.le_u
      br_if 0 (;@1;)
      i32.const 1051672
      i64.load
      local.tee 8
      i64.const 255
      i64.and
      i64.const 4
      i64.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u
      i32.const 3
      i32.eq
      if ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.load
        local.set 4
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.load
        local.tee 5
        if ;; label = @3
          local.get 4
          local.get 5
          call_indirect (type 2)
        else
        end
        local.get 3
        i32.load offset=4
        if ;; label = @3
          local.get 3
          i32.load offset=8
          drop
          local.get 4
          call 5
        else
        end
        local.get 1
        call 5
      else
      end
      local.get 0
      local.get 8
      i64.store align=4
      i32.const 1
      local.set 3
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 3
  )
  (func (;17;) (type 8) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 3
    if ;; label = @1
      local.get 3
      i32.const 3
      i32.and
      local.set 4
      block ;; label = @2
        local.get 3
        i32.const 4
        i32.ge_u
        if ;; label = @3
          local.get 2
          i32.const 28
          i32.add
          local.set 5
          local.get 3
          i32.const 268435452
          i32.and
          local.set 8
          loop ;; label = @4
            local.get 5
            i32.load
            local.get 5
            i32.const 8
            i32.sub
            i32.load
            local.get 5
            i32.const 16
            i32.sub
            i32.load
            local.get 5
            i32.const 24
            i32.sub
            i32.load
            local.get 6
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 6
            local.get 5
            i32.const 32
            i32.add
            local.set 5
            local.get 8
            local.get 7
            i32.const 4
            i32.add
            local.tee 7
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        else
        end
        local.get 7
        i32.const 3
        i32.shl
        local.get 2
        i32.add
        i32.const 4
        i32.add
        local.set 5
        loop ;; label = @3
          local.get 5
          i32.load
          local.get 6
          i32.add
          local.set 6
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 4
          i32.const 1
          i32.sub
          local.tee 4
          br_if 0 (;@3;)
        end
      end
      local.get 1
      i32.load
      local.get 1
      i32.load offset=8
      local.tee 4
      i32.sub
      local.get 6
      i32.lt_u
      if ;; label = @2
        local.get 1
        local.get 4
        local.get 6
        call 25
      else
      end
      local.get 3
      i32.const 3
      i32.shl
      local.get 2
      i32.add
      local.set 5
      local.get 1
      i32.load offset=8
      local.set 4
      loop ;; label = @2
        local.get 2
        i32.load
        local.set 7
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 3
            local.get 1
            i32.load
            local.get 4
            i32.sub
            i32.gt_u
            if ;; label = @5
              local.get 1
              local.get 4
              local.get 3
              call 25
              local.get 1
              i32.load offset=8
              local.set 4
              br 1 (;@4;)
            else
            end
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.get 4
          i32.add
          local.get 7
          local.get 3
          memory.copy
        end
        local.get 1
        local.get 3
        local.get 4
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
    else
    end
    local.get 0
    i32.const 4
    i32.store8
    local.get 0
    local.get 6
    i32.store offset=4
  )
  (func (;18;) (type 8) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 3
    if ;; label = @1
      local.get 3
      i32.const 3
      i32.and
      local.set 4
      block ;; label = @2
        local.get 3
        i32.const 4
        i32.ge_u
        if ;; label = @3
          local.get 2
          i32.const 28
          i32.add
          local.set 5
          local.get 3
          i32.const 268435452
          i32.and
          local.set 8
          loop ;; label = @4
            local.get 5
            i32.load
            local.get 5
            i32.const 8
            i32.sub
            i32.load
            local.get 5
            i32.const 16
            i32.sub
            i32.load
            local.get 5
            i32.const 24
            i32.sub
            i32.load
            local.get 6
            i32.add
            i32.add
            i32.add
            i32.add
            local.set 6
            local.get 5
            i32.const 32
            i32.add
            local.set 5
            local.get 8
            local.get 7
            i32.const 4
            i32.add
            local.tee 7
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        else
        end
        local.get 7
        i32.const 3
        i32.shl
        local.get 2
        i32.add
        i32.const 4
        i32.add
        local.set 5
        loop ;; label = @3
          local.get 5
          i32.load
          local.get 6
          i32.add
          local.set 6
          local.get 5
          i32.const 8
          i32.add
          local.set 5
          local.get 4
          i32.const 1
          i32.sub
          local.tee 4
          br_if 0 (;@3;)
        end
      end
      local.get 1
      i32.load
      local.get 1
      i32.load offset=8
      local.tee 4
      i32.sub
      local.get 6
      i32.lt_u
      if ;; label = @2
        local.get 1
        local.get 4
        local.get 6
        call 25
        local.get 1
        i32.load offset=8
        local.set 4
      else
      end
      local.get 3
      i32.const 3
      i32.shl
      local.get 2
      i32.add
      local.set 5
      loop ;; label = @2
        local.get 2
        i32.load
        local.set 6
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.const 4
            i32.add
            i32.load
            local.tee 3
            local.get 1
            i32.load
            local.get 4
            i32.sub
            i32.gt_u
            if ;; label = @5
              local.get 1
              local.get 4
              local.get 3
              call 25
              local.get 1
              i32.load offset=8
              local.set 4
              br 1 (;@4;)
            else
            end
            local.get 3
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=4
          local.get 4
          i32.add
          local.get 6
          local.get 3
          memory.copy
        end
        local.get 1
        local.get 3
        local.get 4
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
    else
    end
    local.get 0
    i32.const 4
    i32.store8
  )
  (func (;19;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i64 i64 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    i32.const 20
    local.set 2
    local.get 0
    i64.load
    local.tee 7
    local.set 6
    local.get 7
    i64.const 1000
    i64.ge_u
    if ;; label = @1
      loop ;; label = @2
        local.get 3
        i32.const 12
        i32.add
        local.get 2
        i32.add
        local.tee 0
        i32.const 4
        i32.sub
        local.get 6
        local.tee 8
        local.get 6
        i64.const 10000
        i64.div_u
        local.tee 6
        i64.const 10000
        i64.mul
        i64.sub
        i32.wrap_i64
        local.tee 4
        i32.const 65535
        i32.and
        i32.const 100
        i32.div_u
        local.tee 5
        i32.const 1
        i32.shl
        i32.load16_u offset=1051994 align=1
        i32.store16 align=1
        local.get 0
        i32.const 2
        i32.sub
        local.get 4
        local.get 5
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1051994 align=1
        i32.store16 align=1
        local.get 2
        i32.const 4
        i32.sub
        local.set 2
        local.get 8
        i64.const 9999999
        i64.gt_u
        br_if 0 (;@2;)
      end
    else
    end
    local.get 6
    i64.const 9
    i64.gt_u
    if ;; label = @1
      local.get 2
      i32.const 2
      i32.sub
      local.tee 2
      local.get 3
      i32.const 12
      i32.add
      i32.add
      local.get 6
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
      i32.load16_u offset=1051994 align=1
      i32.store16 align=1
      local.get 0
      i64.extend_i32_u
      local.set 6
    else
    end
    local.get 7
    i64.eqz
    i32.eqz
    local.get 6
    i64.eqz
    i32.and
    i32.eqz
    if ;; label = @1
      local.get 2
      i32.const 1
      i32.sub
      local.tee 2
      local.get 3
      i32.const 12
      i32.add
      i32.add
      local.get 6
      i32.wrap_i64
      i32.const 1
      i32.shl
      i32.load8_u offset=1051995
      i32.store8
    else
    end
    local.get 1
    local.get 3
    i32.const 12
    i32.add
    local.get 2
    i32.add
    i32.const 20
    local.get 2
    i32.sub
    call 13
    local.get 3
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;20;) (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    i32.const 1052680
    i32.load8_u
    local.set 2
    i32.const 1052680
    i32.const 1
    i32.store8
    block ;; label = @1
      local.get 2
      i32.eqz
      if ;; label = @2
        local.get 0
        local.get 1
        i32.store offset=12
        local.get 0
        local.get 0
        i32.const 12
        i32.add
        i64.extend_i32_u
        i64.const 34359738368
        i64.or
        i64.store offset=16
        local.get 0
        i32.const 4
        i32.add
        local.get 0
        i32.const 31
        i32.add
        i32.const 1049834
        local.get 0
        i32.const 16
        i32.add
        call 70
        local.get 0
        i32.load8_u offset=4
        local.get 0
        i32.load offset=8
        call 50
        call 51
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                call 3
                i32.const 255
                i32.and
                i32.const 1
                i32.sub
                br_table 1 (;@5;) 2 (;@4;) 3 (;@3;) 0 (;@6;)
              end
              local.get 0
              i32.const 16
              i32.add
              local.get 0
              i32.const 31
              i32.add
              i32.const 9
              i32.const 0
              call 52
              local.get 0
              i32.load8_u offset=16
              local.get 0
              i32.load offset=20
              call 50
              br 2 (;@3;)
            end
            local.get 0
            i32.const 16
            i32.add
            local.get 0
            i32.const 31
            i32.add
            i32.const 9
            i32.const 1
            call 52
            local.get 0
            i32.load8_u offset=16
            local.get 0
            i32.load offset=20
            call 50
            br 1 (;@3;)
          end
          local.get 0
          i32.const 16
          i32.add
          local.get 0
          i32.const 31
          i32.add
          i32.const 1051072
          i32.const 157
          call 70
          local.get 0
          i32.load8_u offset=16
          local.get 0
          i32.load offset=20
          call 50
        end
        i32.const 0
        i32.store8
        br 1 (;@1;)
      else
      end
      local.get 0
      local.get 1
      i32.store offset=12
      local.get 0
      local.get 0
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.const 34359738368
      i64.or
      i64.store offset=16
      local.get 0
      i32.const 4
      i32.add
      local.get 0
      i32.const 31
      i32.add
      i32.const 1049670
      local.get 0
      i32.const 16
      i32.add
      call 70
      local.get 0
      i32.load8_u offset=4
      local.get 0
      i32.load offset=8
      call 50
    end
    local.get 0
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;21;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block (result i32) ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if ;; label = @2
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
        local.get 1
        i32.const 2048
        i32.lt_u
        if ;; label = @3
          local.get 2
          local.get 3
          i32.store8 offset=13
          local.get 2
          local.get 4
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          br 2 (;@1;)
        else
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
        local.get 1
        i32.const 65535
        i32.le_u
        if ;; label = @3
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
          br 2 (;@1;)
        else
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
        br 1 (;@1;)
      else
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
    end
    local.tee 1
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.sub
    i32.gt_u
    if ;; label = @1
      local.get 0
      local.get 3
      local.get 1
      call 25
      local.get 0
      i32.load offset=8
      local.set 3
    else
    end
    local.get 1
    if ;; label = @1
      local.get 0
      i32.load offset=4
      local.get 3
      i32.add
      local.get 2
      i32.const 12
      i32.add
      local.get 1
      memory.copy
    else
    end
    local.get 0
    local.get 1
    local.get 3
    i32.add
    i32.store offset=8
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    i32.const 0
  )
  (func (;22;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    block (result i32) ;; label = @1
      i32.const 0
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      drop
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              local.get 1
              local.get 2
              call 46
              local.tee 3
              i32.const -1
              i32.eq
              if ;; label = @6
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
                i32.const 1052684
                i32.load
                local.tee 3
                i32.store offset=12
                local.get 3
                i32.const 27
                i32.eq
                br_if 1 (;@5;)
                local.get 4
                i32.const 8
                i32.add
                local.set 1
                br 4 (;@2;)
              else
              end
              local.get 4
              local.get 3
              i32.store offset=12
              local.get 4
              i32.const 4
              i32.store8 offset=8
              local.get 3
              i32.eqz
              if ;; label = @6
                i32.const 1051672
                local.set 1
                br 4 (;@2;)
              else
              end
              local.get 2
              local.get 3
              i32.lt_u
              br_if 2 (;@3;)
              local.get 1
              local.get 3
              i32.add
              local.set 1
              local.get 2
              local.get 3
              i32.sub
              local.set 2
            end
            local.get 2
            br_if 0 (;@4;)
          end
          i32.const 0
          br 2 (;@1;)
        end
        local.get 3
        local.get 2
        local.get 2
        i32.const 1051888
        call 27
        unreachable
      end
      i32.const 0
      local.get 1
      i64.load
      local.tee 6
      i64.const 255
      i64.and
      i64.const 4
      i64.eq
      br_if 0 (;@1;)
      drop
      local.get 0
      i32.load8_u
      i32.const 3
      i32.eq
      if ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.load
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.load
        local.tee 5
        if ;; label = @3
          local.get 2
          local.get 5
          call_indirect (type 2)
        else
        end
        local.get 3
        i32.load offset=4
        if ;; label = @3
          local.get 3
          i32.load offset=8
          drop
          local.get 2
          call 5
        else
        end
        local.get 1
        call 5
      else
      end
      local.get 0
      local.get 6
      i64.store align=4
      i32.const 1
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;23;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    i32.const 10
    local.set 2
    local.get 0
    i32.load
    local.tee 4
    local.set 0
    local.get 4
    i32.const 1000
    i32.ge_u
    if ;; label = @1
      loop ;; label = @2
        local.get 3
        i32.const 6
        i32.add
        local.get 2
        i32.add
        local.tee 5
        i32.const 4
        i32.sub
        local.get 0
        local.tee 6
        local.get 0
        i32.const 10000
        i32.div_u
        local.tee 0
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
        i32.load16_u offset=1051994 align=1
        i32.store16 align=1
        local.get 5
        i32.const 2
        i32.sub
        local.get 7
        local.get 8
        i32.const 100
        i32.mul
        i32.sub
        i32.const 65535
        i32.and
        i32.const 1
        i32.shl
        i32.load16_u offset=1051994 align=1
        i32.store16 align=1
        local.get 2
        i32.const 4
        i32.sub
        local.set 2
        local.get 6
        i32.const 9999999
        i32.gt_u
        br_if 0 (;@2;)
      end
    else
    end
    local.get 0
    i32.const 9
    i32.gt_u
    if ;; label = @1
      local.get 2
      i32.const 2
      i32.sub
      local.tee 2
      local.get 3
      i32.const 6
      i32.add
      i32.add
      local.get 0
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
      i32.load16_u offset=1051994 align=1
      i32.store16 align=1
    else
    end
    i32.const 0
    local.get 4
    local.get 0
    select
    i32.eqz
    if ;; label = @1
      local.get 2
      i32.const 1
      i32.sub
      local.tee 2
      local.get 3
      i32.const 6
      i32.add
      i32.add
      local.get 0
      i32.const 1
      i32.shl
      i32.load8_u offset=1051995
      i32.store8
    else
    end
    local.get 1
    local.get 3
    i32.const 6
    i32.add
    local.get 2
    i32.add
    i32.const 10
    local.get 2
    i32.sub
    call 13
    local.get 3
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;24;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.load offset=8
    local.tee 4
    local.set 2
    block (result i32) ;; label = @1
      i32.const 1
      local.get 1
      i32.const 128
      i32.lt_u
      br_if 0 (;@1;)
      drop
      i32.const 2
      local.get 1
      i32.const 2048
      i32.lt_u
      br_if 0 (;@1;)
      drop
      i32.const 3
      i32.const 4
      local.get 1
      i32.const 65536
      i32.lt_u
      select
    end
    local.tee 6
    local.get 0
    i32.load
    local.get 4
    i32.sub
    i32.gt_u
    if (result i32) ;; label = @1
      local.get 0
      local.get 4
      local.get 6
      call 25
      local.get 0
      i32.load offset=8
    else
      local.get 2
    end
    local.get 0
    i32.load offset=4
    i32.add
    local.set 2
    block ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if ;; label = @2
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 5
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 3
        local.get 1
        i32.const 2048
        i32.lt_u
        if ;; label = @3
          local.get 2
          local.get 5
          i32.store8 offset=1
          local.get 2
          local.get 3
          i32.const 192
          i32.or
          i32.store8
          br 2 (;@1;)
        else
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 7
        local.get 3
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 65535
        i32.le_u
        if ;; label = @3
          local.get 2
          local.get 5
          i32.store8 offset=2
          local.get 2
          local.get 3
          i32.store8 offset=1
          local.get 2
          local.get 7
          i32.const 224
          i32.or
          i32.store8
          br 2 (;@1;)
        else
        end
        local.get 2
        local.get 5
        i32.store8 offset=3
        local.get 2
        local.get 3
        i32.store8 offset=2
        local.get 2
        local.get 7
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        i32.store8 offset=1
        local.get 2
        local.get 1
        i32.const 18
        i32.shr_u
        i32.const -16
        i32.or
        i32.store8
        br 1 (;@1;)
      else
      end
      local.get 2
      local.get 1
      i32.store8
    end
    local.get 0
    local.get 4
    local.get 6
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func (;25;) (type 6) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 2
    local.get 1
    local.get 2
    i32.add
    local.tee 4
    i32.gt_u
    if ;; label = @1
      i32.const 0
      i32.const 0
      call 61
      unreachable
    else
    end
    local.get 3
    i32.const 4
    i32.add
    local.set 5
    local.get 0
    i32.load
    local.tee 2
    local.set 6
    local.get 0
    i32.load offset=4
    local.set 8
    i32.const 1
    local.set 7
    i32.const 4
    local.set 1
    block ;; label = @1
      i32.const 8
      local.get 4
      local.get 2
      i32.const 1
      i32.shl
      local.tee 2
      local.get 2
      local.get 4
      i32.lt_u
      select
      local.tee 2
      local.get 2
      i32.const 8
      i32.le_u
      select
      local.tee 4
      local.tee 2
      i32.const 2147483647
      i32.gt_u
      if ;; label = @2
        i32.const 0
        local.set 2
        br 1 (;@1;)
      else
      end
      block ;; label = @2
        block ;; label = @3
          block (result i32) ;; label = @4
            local.get 6
            if ;; label = @5
              local.get 8
              local.get 6
              i32.const 1
              local.get 2
              call 7
              br 1 (;@4;)
            else
            end
            local.get 2
            i32.eqz
            if ;; label = @5
              i32.const 1
              local.set 1
              br 2 (;@3;)
            else
            end
            local.get 2
            i32.const 1
            call 42
          end
          local.tee 1
          br_if 0 (;@3;)
          local.get 5
          i32.const 1
          i32.store offset=4
          br 1 (;@2;)
        end
        local.get 5
        local.get 1
        i32.store offset=4
        i32.const 0
        local.set 7
      end
      i32.const 8
      local.set 1
    end
    local.get 1
    local.get 5
    i32.add
    local.get 2
    i32.store
    local.get 5
    local.get 7
    i32.store
    local.get 3
    i32.load offset=4
    i32.const 1
    i32.eq
    if ;; label = @1
      local.get 3
      i32.load offset=8
      local.get 3
      i32.load offset=12
      call 61
      unreachable
    else
    end
    local.get 3
    i32.load offset=8
    local.set 1
    local.get 0
    local.get 4
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;26;) (type 6) (param i32 i32 i32)
    (local i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    call 51
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
      i32.const 1052656
      i32.load
      local.tee 4
      i32.const 2
      i32.le_u
      if ;; label = @2
        local.get 3
        i32.const 12
        i32.add
        i32.const 0
        call 9
        br 1 (;@1;)
      else
      end
      local.get 3
      local.get 4
      i32.const 8
      i32.sub
      i32.store offset=28
      local.get 3
      i32.const 12
      i32.add
      local.get 3
      i32.const 28
      i32.add
      call 9
    end
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load offset=8
            i32.load8_u
            i32.const 1
            i32.sub
            br_table 1 (;@3;) 2 (;@2;) 3 (;@1;) 0 (;@4;)
          end
          local.get 3
          i32.const 12
          i32.add
          local.get 1
          local.get 2
          i32.load offset=36
          i32.const 0
          call 52
          local.get 3
          i32.load8_u offset=12
          local.get 3
          i32.load offset=16
          call 50
          br 2 (;@1;)
        end
        local.get 3
        i32.const 12
        i32.add
        local.get 1
        local.get 2
        i32.load offset=36
        i32.const 1
        call 52
        local.get 3
        i32.load8_u offset=12
        local.get 3
        i32.load offset=16
        call 50
        br 1 (;@1;)
      end
      i32.const 1052592
      i32.load8_u
      i32.const 1052592
      i32.const 0
      i32.store8
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 12
      i32.add
      local.get 1
      i32.const 1051072
      i32.const 157
      local.get 2
      i32.load offset=36
      call_indirect (type 8)
      local.get 3
      i32.load8_u offset=12
      local.get 3
      i32.load offset=16
      call 50
    end
    i32.const 0
    i32.store8
    local.get 3
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;27;) (type 8) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    block ;; label = @1
      local.get 0
      local.get 2
      i32.le_u
      if ;; label = @2
        local.get 0
        local.get 1
        i32.le_u
        local.get 1
        local.get 2
        i32.gt_u
        i32.or
        br_if 1 (;@1;)
        local.get 4
        local.get 0
        i32.store offset=8
        local.get 4
        local.get 1
        i32.store offset=12
        local.get 4
        local.get 4
        i32.const 12
        i32.add
        i64.extend_i32_u
        i64.const 34359738368
        i64.or
        i64.store offset=24
        local.get 4
        local.get 4
        i32.const 8
        i32.add
        i64.extend_i32_u
        i64.const 34359738368
        i64.or
        i64.store offset=16
        i32.const 1048624
        local.get 4
        i32.const 16
        i32.add
        local.get 3
        call 32
        unreachable
      else
      end
      local.get 4
      local.get 0
      i32.store offset=8
      local.get 4
      local.get 2
      i32.store offset=12
      local.get 4
      local.get 4
      i32.const 12
      i32.add
      i64.extend_i32_u
      i64.const 34359738368
      i64.or
      i64.store offset=24
      local.get 4
      local.get 4
      i32.const 8
      i32.add
      i64.extend_i32_u
      i64.const 34359738368
      i64.or
      i64.store offset=16
      i32.const 1048719
      local.get 4
      i32.const 16
      i32.add
      local.get 3
      call 32
      unreachable
    end
    local.get 4
    local.get 1
    i32.store offset=8
    local.get 4
    local.get 2
    i32.store offset=12
    local.get 4
    local.get 4
    i32.const 12
    i32.add
    i64.extend_i32_u
    i64.const 34359738368
    i64.or
    i64.store offset=24
    local.get 4
    local.get 4
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.const 34359738368
    i64.or
    i64.store offset=16
    i32.const 1048776
    local.get 4
    i32.const 16
    i32.add
    local.get 3
    call 32
    unreachable
  )
  (func (;28;) (type 3) (param i32) (result i32)
    (local i32 i32 i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        local.tee 1
        i32.const 3
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u
        i32.eqz
        if ;; label = @3
          i32.const 0
          return
        else
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
      i32.const 4
      i32.sub
      local.set 2
      local.get 1
      i32.const 5
      i32.sub
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
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 0
    i32.sub
  )
  (func (;29;) (type 7) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64)
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.load offset=4
    local.tee 4
    i64.const 4294967295
    local.get 3
    i64.load offset=8
    local.tee 8
    local.get 8
    i64.const 4294967295
    i64.ge_u
    select
    i32.wrap_i64
    i32.sub
    local.tee 5
    i32.const 0
    local.get 4
    local.get 5
    i32.ge_u
    select
    local.tee 5
    local.get 2
    local.get 2
    local.get 5
    i32.gt_u
    select
    local.tee 6
    if ;; label = @1
      local.get 3
      i32.load
      local.get 8
      local.get 4
      i64.extend_i32_u
      local.tee 9
      local.get 8
      local.get 9
      i64.lt_u
      select
      i32.wrap_i64
      i32.add
      local.get 1
      local.get 6
      memory.copy
    else
    end
    local.get 3
    local.get 8
    local.get 6
    i64.extend_i32_u
    i64.add
    i64.store offset=8
    block ;; label = @1
      local.get 2
      local.get 5
      i32.le_u
      br_if 0 (;@1;)
      i32.const 1051672
      i64.load
      local.tee 8
      i64.const 255
      i64.and
      i64.const 4
      i64.eq
      br_if 0 (;@1;)
      local.get 0
      i32.load8_u
      i32.const 3
      i32.eq
      if ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.load
        local.set 2
        local.get 1
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.load
        local.tee 4
        if ;; label = @3
          local.get 2
          local.get 4
          call_indirect (type 2)
        else
        end
        local.get 3
        i32.load offset=4
        if ;; label = @3
          local.get 3
          i32.load offset=8
          drop
          local.get 2
          call 5
        else
        end
        local.get 1
        call 5
      else
      end
      local.get 0
      local.get 8
      i64.store align=4
      i32.const 1
      local.set 7
    end
    local.get 7
  )
  (func (;30;) (type 0)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 2
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    block ;; label = @1
      block ;; label = @2
        loop ;; label = @3
          local.get 0
          i32.const 8
          i32.add
          local.tee 3
          global.get 1
          local.get 4
          i32.add
          i64.load align=1
          i64.store align=1
          global.get 1
          local.get 0
          i32.load offset=8
          i32.add
          local.set 1
          local.get 0
          local.get 0
          i32.load offset=12
          i32.store offset=12
          local.get 0
          local.get 1
          i32.store offset=8
          global.get 0
          i32.const 16
          i32.sub
          local.tee 1
          global.set 0
          local.get 0
          block (result i32) ;; label = @4
            i32.const 1
            local.get 3
            i32.const 1
            local.get 1
            i32.const 12
            i32.add
            call 0
            local.tee 6
            i32.eqz
            if ;; label = @5
              local.get 0
              local.get 1
              i32.load offset=12
              i32.store offset=4
              i32.const 0
              br 1 (;@4;)
            else
            end
            local.get 0
            local.get 6
            i32.store16 offset=2
            i32.const 1
          end
          i32.store16
          local.get 1
          i32.const 16
          i32.add
          global.set 0
          local.get 0
          i32.load16_u
          br_if 1 (;@2;)
          local.get 0
          i32.load offset=4
          local.get 5
          i32.add
          local.set 5
          local.get 4
          i32.const 8
          i32.add
          local.set 4
          local.get 2
          i32.const 1
          i32.sub
          local.tee 2
          br_if 0 (;@3;)
        end
        local.get 0
        local.get 5
        i32.store offset=8
        global.get 1
        i32.const 8
        i32.add
        local.get 3
        i32.load align=1
        i32.store align=1
        br 1 (;@1;)
      end
      local.get 0
      i32.load16_u offset=2
      drop
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0
    i32.const 0
    call 69
  )
  (func (;31;) (type 5) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    block (result i32) ;; label = @1
      local.get 1
      i32.const 128
      i32.ge_u
      if ;; label = @2
        local.get 1
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 0
        local.get 1
        i32.const 6
        i32.shr_u
        local.set 3
        local.get 1
        i32.const 2048
        i32.lt_u
        if ;; label = @3
          local.get 2
          local.get 0
          i32.store8 offset=13
          local.get 2
          local.get 3
          i32.const 192
          i32.or
          i32.store8 offset=12
          i32.const 2
          br 2 (;@1;)
        else
        end
        local.get 1
        i32.const 12
        i32.shr_u
        local.set 4
        local.get 3
        i32.const 63
        i32.and
        i32.const -128
        i32.or
        local.set 3
        local.get 1
        i32.const 65535
        i32.le_u
        if ;; label = @3
          local.get 2
          local.get 0
          i32.store8 offset=14
          local.get 2
          local.get 3
          i32.store8 offset=13
          local.get 2
          local.get 4
          i32.const 224
          i32.or
          i32.store8 offset=12
          i32.const 3
          br 2 (;@1;)
        else
        end
        local.get 2
        local.get 0
        i32.store8 offset=15
        local.get 2
        local.get 3
        i32.store8 offset=14
        local.get 2
        local.get 4
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
        br 1 (;@1;)
      else
      end
      local.get 2
      local.get 1
      i32.store8 offset=12
      i32.const 1
    end
    call 22
    local.get 2
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;32;) (type 6) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
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
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 3
    i32.const 20
    i32.add
    local.tee 0
    i64.load align=4
    local.set 4
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 4
    i64.store offset=4 align=4
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    local.get 1
    i32.const 4
    i32.add
    local.tee 1
    i32.load
    local.tee 2
    i32.load offset=4
    local.tee 3
    i32.const 1
    i32.and
    if ;; label = @1
      local.get 2
      i32.load
      local.set 2
      local.get 0
      local.get 3
      i32.const 1
      i32.shr_u
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      local.get 0
      i32.const 1051152
      local.get 1
      i32.load offset=4
      local.get 1
      i32.load offset=8
      local.tee 0
      i32.load8_u offset=8
      local.get 0
      i32.load8_u offset=9
      call 8
      unreachable
    else
    end
    local.get 0
    i32.const -2147483648
    i32.store
    local.get 0
    local.get 1
    i32.store offset=12
    local.get 0
    i32.const 1051180
    local.get 1
    i32.load offset=4
    local.get 1
    i32.load offset=8
    local.tee 0
    i32.load8_u offset=8
    local.get 0
    i32.load8_u offset=9
    call 8
    unreachable
  )
  (func (;33;) (type 4) (param i32 i32)
    (local i32 i32 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    i32.const -2147483648
    i32.eq
    if ;; label = @1
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
      i32.const 1051048
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call 12
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
    else
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
    i32.const 12
    i32.const 4
    call 42
    local.tee 1
    i32.eqz
    if ;; label = @1
      i32.const 4
      i32.const 12
      call 55
      unreachable
    else
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
    i32.const 1051872
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0
  )
  (func (;34;) (type 8) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block ;; label = @1
      block ;; label = @2
        local.get 3
        if ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 0
                block (result i32) ;; label = @7
                  local.get 2
                  local.get 3
                  call 46
                  local.tee 4
                  i32.const -1
                  i32.eq
                  if ;; label = @8
                    local.get 1
                    i32.const 0
                    i32.store8 offset=11
                    local.get 1
                    i32.const 0
                    i32.store16 offset=9 align=1
                    local.get 1
                    i32.const 0
                    i32.store8 offset=8
                    local.get 1
                    i32.const 1052684
                    i32.load
                    local.tee 4
                    i32.store offset=12
                    local.get 4
                    i32.const 27
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 1
                    i32.const 8
                    i32.add
                    br 1 (;@7;)
                  else
                  end
                  local.get 1
                  local.get 4
                  i32.store offset=12
                  local.get 1
                  i32.const 4
                  i32.store8 offset=8
                  local.get 4
                  br_if 1 (;@6;)
                  i32.const 1051672
                end
                i64.load
                i64.store align=4
                br 5 (;@1;)
              end
              local.get 3
              local.get 4
              i32.lt_u
              br_if 3 (;@2;)
              local.get 2
              local.get 4
              i32.add
              local.set 2
              local.get 3
              local.get 4
              i32.sub
              local.set 3
            end
            local.get 3
            br_if 0 (;@4;)
          end
        else
        end
        local.get 0
        i32.const 4
        i32.store8
        br 1 (;@1;)
      end
      local.get 4
      local.get 3
      local.get 3
      i32.const 1051888
      call 27
      unreachable
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;35;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 1051484
    i32.store offset=12
    local.get 1
    local.get 0
    i32.store offset=8
    global.get 0
    i32.const -64
    i32.add
    local.tee 0
    global.set 0
    local.get 0
    i32.const 1050760
    i32.store offset=4
    local.get 0
    local.get 1
    i32.const 8
    i32.add
    i32.store
    local.get 0
    i32.const 1050760
    i32.store offset=12
    local.get 0
    local.get 1
    i32.const 12
    i32.add
    i32.store offset=8
    local.get 0
    i32.const 1052216
    i32.load
    i32.store offset=20
    local.get 0
    i32.const 1052204
    i32.load
    i32.store offset=16
    local.get 0
    i32.const 65
    i32.store offset=28
    local.get 0
    i32.const 1051311
    i32.store offset=24
    local.get 0
    local.get 0
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.const 219043332096
    i64.or
    i64.store offset=56
    local.get 0
    local.get 0
    i64.extend_i32_u
    i64.const 219043332096
    i64.or
    i64.store offset=48
    local.get 0
    local.get 0
    i32.const 24
    i32.add
    i64.extend_i32_u
    i64.const 223338299392
    i64.or
    i64.store offset=40
    local.get 0
    local.get 0
    i32.const 16
    i32.add
    i64.extend_i32_u
    i64.const 227633266688
    i64.or
    i64.store offset=32
    i32.const 1048886
    local.get 0
    i32.const 32
    i32.add
    i32.const 1051344
    call 32
    unreachable
  )
  (func (;36;) (type 10) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 5
    global.set 0
    local.get 0
    i32.const 4
    i32.store8
    local.get 5
    local.get 1
    i32.store offset=8
    local.get 5
    local.get 0
    i64.load align=4
    i64.store
    local.get 5
    local.get 4
    local.get 2
    local.get 3
    call 12
    local.set 2
    local.get 5
    i32.load8_u
    local.set 1
    block ;; label = @1
      block ;; label = @2
        local.get 2
        if ;; label = @3
          local.get 1
          i32.const 4
          i32.ne
          br_if 1 (;@2;)
          i32.const 1050800
          i32.const 173
          i32.const 1050888
          call 32
          unreachable
        else
        end
        local.get 1
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 5
        i32.load offset=4
        local.tee 0
        i32.load
        local.set 1
        local.get 0
        i32.const 4
        i32.add
        i32.load
        local.tee 2
        i32.load
        local.tee 3
        if ;; label = @3
          local.get 1
          local.get 3
          call_indirect (type 2)
        else
        end
        local.get 2
        i32.load offset=4
        if ;; label = @3
          local.get 2
          i32.load offset=8
          drop
          local.get 1
          call 5
        else
        end
        local.get 0
        call 5
        br 1 (;@1;)
      end
      local.get 0
      local.get 5
      i64.load
      i64.store align=4
    end
    local.get 5
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;37;) (type 3) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.eqz
    if ;; label = @1
      call 84
      i32.const 16
      i32.shl
      return
    else
    end
    local.get 0
    i32.const 65535
    i32.and
    local.get 0
    i32.const 0
    i32.lt_s
    i32.or
    i32.eqz
    if ;; label = @1
      block (result i32) ;; label = @2
        call 84
        memory.size
        local.set 1
        i32.const -1
        local.get 0
        i32.const 16
        i32.shr_u
        local.tee 0
        memory.grow
        i32.const -1
        i32.eq
        br_if 0 (;@2;)
        drop
        local.get 0
        i32.const 16
        i32.shl
        local.tee 0
        global.get 1
        i32.add
        local.tee 2
        global.get 1
        local.get 1
        i32.const 16
        i32.shl
        global.get 1
        i32.sub
        memory.copy
        local.get 2
        global.set 1
        local.get 0
        global.get 2
        i32.add
        global.set 2
      end
      local.tee 0
      i32.const -1
      i32.eq
      if ;; label = @2
        i32.const 1052684
        i32.const 48
        i32.store
        i32.const -1
        return
      else
      end
      local.get 0
      i32.const 16
      i32.shl
      return
    else
    end
    unreachable
  )
  (func (;38;) (type 4) (param i32 i32)
    (local i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load
    i32.const -2147483648
    i32.eq
    if ;; label = @1
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
      i32.const 1051048
      local.get 3
      i32.load
      local.tee 3
      i32.load
      local.get 3
      i32.load offset=4
      call 12
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
    else
    end
    local.get 0
    i32.const 1051872
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;39;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i32.load
    local.get 2
    local.get 0
    i32.load
    local.tee 0
    i64.load align=4
    i64.store align=4
    local.get 2
    local.get 0
    i32.const 12
    i32.add
    i64.extend_i32_u
    i64.const 34359738368
    i64.or
    i64.store offset=24
    local.get 2
    local.get 0
    i32.const 8
    i32.add
    i64.extend_i32_u
    i64.const 34359738368
    i64.or
    i64.store offset=16
    local.get 2
    local.get 2
    i64.extend_i32_u
    i64.const 17179869184
    i64.or
    i64.store offset=8
    local.get 3
    i32.const 1048616
    local.get 2
    i32.const 8
    i32.add
    call 12
    local.get 2
    i32.const 32
    i32.add
    global.set 0
  )
  (func (;40;) (type 0)
    i32.const 1052608
    i32.load8_u
    i32.eqz
    if ;; label = @1
      i32.const 1052608
      i32.const 1
      i32.store8
    else
    end
    global.get 1
    i32.const 0
    i32.const 16
    memory.fill
    global.get 1
    i32.const 30
    i32.add
    i32.const 0
    i32.const -30
    memory.fill
    global.get 1
    i32.const 5
    i32.add
    i32.const 0
    global.get 2
    i32.const 16
    i32.shr_u
    global.get 1
    i32.const 16
    i32.shr_u
    i32.sub
    i32.const 16
    i32.shl
    i32.const 5
    i32.sub
    memory.fill
    global.get 1
    i32.const 16
    i32.add
    global.get 2
    i32.const 14
    memory.copy
    global.get 1
    global.get 2
    i32.const 14
    i32.add
    i32.const 5
    memory.copy
    call 60
    call 30
    call 30
  )
  (func (;41;) (type 8) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 3
        i32.lt_u
        if ;; label = @3
          local.get 1
          local.get 4
          local.get 3
          call 25
          local.get 1
          i32.load offset=8
          local.set 4
          br 1 (;@2;)
        else
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
    local.get 3
    local.get 4
    i32.add
    i32.store offset=8
  )
  (func (;42;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block (result i32) ;; label = @1
      local.get 1
      i32.const 8
      i32.le_u
      local.get 0
      local.get 1
      i32.ge_u
      i32.and
      i32.eqz
      if ;; label = @2
        local.get 2
        i32.const 0
        i32.store offset=12
        local.get 2
        i32.const 12
        i32.add
        i32.const 4
        local.get 1
        local.get 1
        i32.const 4
        i32.le_u
        select
        local.get 0
        call 10
        local.set 0
        i32.const 0
        local.get 2
        i32.load offset=12
        local.get 0
        select
        br 1 (;@1;)
      else
      end
      local.get 0
      call 2
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;43;) (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.lt_u
        if ;; label = @3
          local.get 0
          local.get 3
          local.get 2
          call 25
          local.get 0
          i32.load offset=8
          local.set 3
          br 1 (;@2;)
        else
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
    local.get 2
    local.get 3
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func (;44;) (type 8) (param i32 i32 i32 i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load
        local.get 1
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 3
        i32.lt_u
        if ;; label = @3
          local.get 1
          local.get 4
          local.get 3
          call 25
          local.get 1
          i32.load offset=8
          local.set 4
          br 1 (;@2;)
        else
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
    local.get 3
    local.get 4
    i32.add
    i32.store offset=8
  )
  (func (;45;) (type 5) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    i32.const -1
    local.set 3
    block ;; label = @1
      local.get 1
      i32.const 0
      i32.lt_s
      if ;; label = @2
        i32.const 1052684
        i32.const 28
        i32.store
        br 1 (;@1;)
      else
      end
      local.get 2
      i32.const 0
      i32.store offset=12
      local.get 0
      local.get 1
      local.get 2
      i32.const 12
      i32.add
      call 68
      local.tee 0
      if ;; label = @2
        i32.const 1052684
        local.get 0
        i32.store
        br 1 (;@1;)
      else
      end
      local.get 2
      i32.load offset=12
      local.set 3
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 3
  )
  (func (;46;) (type 5) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    block (result i32) ;; label = @1
      local.get 2
      i32.const 8
      i32.add
      i32.const 1
      local.get 2
      i32.const 4
      i32.add
      call 68
      local.tee 0
      if ;; label = @2
        i32.const 1052684
        i32.const 8
        local.get 0
        local.get 0
        i32.const 76
        i32.eq
        select
        i32.store
        i32.const -1
        br 1 (;@1;)
      else
      end
      local.get 2
      i32.load offset=4
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;47;) (type 7) (param i32 i32 i32) (result i32)
    (local i32)
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i32.load
        local.get 0
        i32.load offset=8
        local.tee 3
        i32.sub
        local.get 2
        i32.lt_u
        if ;; label = @3
          local.get 0
          local.get 3
          local.get 2
          call 25
          local.get 0
          i32.load offset=8
          local.set 3
          br 1 (;@2;)
        else
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
    local.get 2
    local.get 3
    i32.add
    i32.store offset=8
    i32.const 0
  )
  (func (;48;) (type 8) (param i32 i32 i32 i32)
    i32.const 4
    local.set 1
    local.get 2
    i32.const 16
    local.get 3
    local.get 3
    i32.const 16
    i32.ge_u
    select
    call 45
    local.tee 3
    i32.const -1
    i32.eq
    if ;; label = @1
      local.get 0
      i32.const 0
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 1052684
      i32.load
      local.set 3
      i32.const 0
      local.set 1
    else
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store8
  )
  (func (;49;) (type 2) (param i32)
    (local i32 i32 i32)
    local.get 0
    i32.load8_u
    i32.const 3
    i32.eq
    if ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 0
      i32.load
      local.set 1
      local.get 0
      i32.const 4
      i32.add
      i32.load
      local.tee 2
      i32.load
      local.tee 3
      if ;; label = @2
        local.get 1
        local.get 3
        call_indirect (type 2)
      else
      end
      local.get 2
      i32.load offset=4
      if ;; label = @2
        local.get 2
        i32.load offset=8
        drop
        local.get 1
        call 5
      else
      end
      local.get 0
      call 5
    else
    end
  )
  (func (;50;) (type 4) (param i32 i32)
    (local i32 i32)
    local.get 0
    i32.const 255
    i32.and
    i32.const 3
    i32.eq
    if ;; label = @1
      local.get 1
      i32.load
      local.set 0
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 2
      i32.load
      local.tee 3
      if ;; label = @2
        local.get 0
        local.get 3
        call_indirect (type 2)
      else
      end
      local.get 2
      i32.load offset=4
      if ;; label = @2
        local.get 2
        i32.load offset=8
        drop
        local.get 0
        call 5
      else
      end
      local.get 1
      call 5
    else
    end
  )
  (func (;51;) (type 1) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    i32.const 1052681
    i32.load8_u
    local.set 1
    i32.const 1052681
    i32.const 1
    i32.store8
    local.get 0
    local.get 1
    i32.store8 offset=15
    local.get 1
    i32.const 1
    i32.eq
    if ;; label = @1
      local.get 0
      i32.const 15
      i32.add
      call 35
      unreachable
    else
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0
    i32.const 1052681
  )
  (func (;52;) (type 8) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    local.get 3
    i32.store8 offset=7
    local.get 4
    local.get 4
    i32.const 7
    i32.add
    i64.extend_i32_u
    i64.const 42949672960
    i64.or
    i64.store offset=8
    local.get 0
    local.get 1
    i32.const 1048943
    local.get 4
    i32.const 8
    i32.add
    local.get 2
    call_indirect (type 8)
    local.get 4
    i32.const 16
    i32.add
    global.set 0
  )
  (func (;53;) (type 8) (param i32 i32 i32 i32)
    i32.const 4
    local.set 1
    local.get 2
    local.get 3
    call 46
    local.tee 3
    i32.const -1
    i32.eq
    if ;; label = @1
      local.get 0
      i32.const 0
      i32.store16 offset=1 align=1
      local.get 0
      i32.const 3
      i32.add
      i32.const 0
      i32.store8
      i32.const 1052684
      i32.load
      local.set 3
      i32.const 0
      local.set 1
    else
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store8
  )
  (func (;54;) (type 4) (param i32 i32)
    (local i32 i32)
    local.get 1
    i32.load offset=4
    local.set 2
    local.get 1
    i32.load
    local.set 3
    i32.const 8
    i32.const 4
    call 42
    local.tee 1
    i32.eqz
    if ;; label = @1
      i32.const 4
      i32.const 8
      call 55
      unreachable
    else
    end
    local.get 1
    local.get 2
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store
    local.get 0
    i32.const 1051856
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func (;55;) (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    local.tee 0
    i32.load
    local.get 0
    i32.load offset=4
    i32.const 1052624
    i32.load
    local.tee 0
    i32.const 3
    local.get 0
    select
    call_indirect (type 4)
    unreachable
  )
  (func (;56;) (type 9) (param i32 i32 i32 i32) (result i32)
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
    local.get 3
    i32.eqz
    if ;; label = @1
      i32.const 0
      return
    else
    end
    local.get 0
    local.get 3
    i32.const 0
    local.get 1
    i32.load offset=12
    call_indirect (type 7)
  )
  (func (;57;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load
    i32.const -2147483648
    i32.ne
    if ;; label = @1
      local.get 1
      local.get 0
      i32.load offset=4
      local.get 0
      i32.load offset=8
      call 66
      return
    else
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
    call 12
  )
  (func (;58;) (type 9) (param i32 i32 i32 i32) (result i32)
    block ;; label = @1
      block (result i32) ;; label = @2
        local.get 1
        i32.eqz
        if ;; label = @3
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          local.get 2
          call 42
          br 1 (;@2;)
        else
        end
        local.get 0
        local.get 1
        local.get 2
        local.get 3
        call 7
      end
      local.tee 2
      br_if 0 (;@1;)
      unreachable
    end
    local.get 2
  )
  (func (;59;) (type 5) (param i32 i32) (result i32)
    block (result i32) ;; label = @1
      local.get 0
      i32.load
      i32.load8_u
      i32.eqz
      if ;; label = @2
        local.get 1
        i32.const 1052194
        i32.const 5
        call 4
        br 1 (;@1;)
      else
      end
      local.get 1
      i32.const 1052199
      i32.const 4
      call 4
    end
  )
  (func (;60;) (type 0)
    global.get 2
    global.get 1
    i32.const 16
    i32.add
    i32.const 14
    memory.copy
    global.get 2
    i32.const 14
    i32.add
    global.get 1
    i32.const 5
    memory.copy
  )
  (func (;61;) (type 4) (param i32 i32)
    local.get 0
    if ;; label = @1
      local.get 0
      local.get 1
      call 55
      unreachable
    else
    end
    i32.const 1051904
    i32.const 35
    i32.const 1051924
    call 32
    unreachable
  )
  (func (;62;) (type 5) (param i32 i32) (result i32)
    local.get 1
    i32.load
    local.get 1
    i32.load offset=4
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 12
  )
  (func (;63;) (type 2) (param i32)
    local.get 0
    i32.load
    i32.const 0
    i32.gt_s
    if ;; label = @1
      local.get 0
      i32.load offset=4
      call 5
    else
    end
  )
  (func (;64;) (type 4) (param i32 i32)
    local.get 0
    i32.const 1051232
    i64.load align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 1051224
    i64.load align=4
    i64.store align=4
  )
  (func (;65;) (type 4) (param i32 i32)
    local.get 0
    i32.const 1051216
    i64.load align=4
    i64.store offset=8 align=4
    local.get 0
    i32.const 1051208
    i64.load align=4
    i64.store align=4
  )
  (func (;66;) (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 7)
  )
  (func (;67;) (type 5) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 0
    i32.load offset=4
    i32.load offset=12
    call_indirect (type 5)
  )
  (func (;68;) (type 7) (param i32 i32 i32) (result i32)
    i32.const 2
    local.get 0
    local.get 1
    local.get 2
    call 0
    i32.const 65535
    i32.and
  )
  (func (;69;) (type 2) (param i32)
    local.get 0
    call 1
    i32.const 1050700
    i32.const 81
    i32.const 1050740
    call 32
    unreachable
  )
  (func (;70;) (type 8) (param i32 i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    i32.const 1050928
    call 36
  )
  (func (;71;) (type 8) (param i32 i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    i32.const 1050904
    call 36
  )
  (func (;72;) (type 2) (param i32)
    local.get 0
    i32.load
    if ;; label = @1
      local.get 0
      i32.load offset=4
      call 5
    else
    end
  )
  (func (;73;) (type 5) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 4
  )
  (func (;74;) (type 4) (param i32 i32)
    local.get 0
    i32.const 1051856
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
  )
  (func (;75;) (type 5) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 66
  )
  (func (;76;) (type 9) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 58
  )
  (func (;77;) (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050776
    local.get 1
    local.get 2
    call 12
  )
  (func (;78;) (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050904
    local.get 1
    local.get 2
    call 12
  )
  (func (;79;) (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1050928
    local.get 1
    local.get 2
    call 12
  )
  (func (;80;) (type 7) (param i32 i32 i32) (result i32)
    local.get 0
    i32.const 1051048
    local.get 1
    local.get 2
    call 12
  )
  (func (;81;) (type 4) (param i32 i32)
    local.get 0
    local.get 1
    i64.load align=4
    i64.store
  )
  (func (;82;) (type 4) (param i32 i32)
    local.get 0
    i32.const 4
    i32.store8
  )
  (func (;83;) (type 4) (param i32 i32)
    local.get 0
    i32.const 0
    i32.store
  )
  (func (;84;) (type 1) (result i32)
    global.get 1
    i32.const 16
    i32.shr_u
  )
  (func (;85;) (type 3) (param i32) (result i32)
    unreachable
  )
  (func (;86;) (type 0)
    unreachable
  )
  (func (;87;) (type 0)
    call 60
  )
  (data (;0;) (i32.const 1048576) "internal error: entered unreachable code\c0\01:\c0\01:\c0\00\16slice index starts at \c0\0d but ends at \c0\00 index out of bounds: the len is \c0\12 but the index is \c0\00\12range start index \c0\22 out of range for slice of length \c0\00\10range end index \c0\22 out of range for slice of length \c0\00\10assertion `left \c0\17 right` failed\0a  left: \c0\09\0a right: \c0\00\10assertion `left \c0\10 right` failed: \c0\09\0a  left: \c0\09\0a right: \c0\00wasi_virt_layer\5csrc\5cwasi\5cfile\5cembedded\5clfs_raw.rs\00wasi_virt_layer\5csrc\5cwasi\5cfile\5cembedded\5cvfs.rs\00wasi_virt_layer\5csrc\5cwasi\5cfile\5cembedded\5clfs.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sys/sync/mutex/no_threads.rs\00wasi_virt_layer\5csrc\5cwasi\5cfile\5cembedded\5clfs_impl.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs\00wasi_virt_layer\5csrc\5ctransporter\5cmod.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/mod.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/alloc/src/raw_vec/mod.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/thread/id.rs\00/rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/sys/io/io_slice/iovec.rs\00/\00\15memory allocation of \c0G bytes failed\0askipping backtrace printing to avoid potential recursion\0a\005fatal runtime error: failed to initiate panic, error \c0\0b, aborting\0a\00\15memory allocation of \c0\0e bytes failed\0a\00\0cpanicked at \c0\02:\0a\c03\0athread panicked while processing panic. aborting.\0a\00\09\0athread '\c0\03' (\c0\0e) panicked at \c0\02:\0a\c0\01\0a\00\19aborting due to panic at \c0\02:\0a\c0\01\0a\00\00\00\00\a3\01\10\00-\00\00\00\dd\00\00\00\09\00\00\00\d1\01\10\00-\00\00\00T\00\00\00 \00\00\00q\01\10\001\00\00\00\f5\01\00\00/\00\00\00HOME=~//root.~root.txtThis is rootheyHey!hellohomeThis is homeuserThis is userworldHello, world!everyoneHello, everyone!\db\05\10\00\05")
  (data (;1;) (i32.const 1050204) "\03\00\00\00\04\00\00\00\e0\05\10\00\01")
  (data (;2;) (i32.const 1050228) "\04\00\00\00\06\00\00\00\e1\05\10\00\01")
  (data (;3;) (i32.const 1050252) "\06\00\00\00\08\00\00\00\e2\05\10\00\08\00\00\00\02\00\00\00\ea\05\10\00\0c\00\00\00\00\00\00\00\f6\05\10\00\03\00\00\00\02\00\00\00\f9\05\10\00\04\00\00\00\01\00\00\00\fd\05\10\00\05\00\00\00\01\00\00\00\01\00\00\00\08\00\00\00\0a\00\00\00\02\06\10\00\04\00\00\00\02\00\00\00\06\06\10\00\0c\00\00\00\02\00\00\00\12\06\10\00\04\00\00\00\02\00\00\00\16\06\10\00\0c\00\00\00\02\00\00\00\22\06\10\00\05\00\00\00\02\00\00\00'\06\10\00\0d\00\00\00\05\00\00\004\06\10\00\08\00\00\00\02\00\00\00<\06\10\00\10\00\00\00\05\00\00\00\d1\01\10\00-\00\00\00\b2\00\00\00\17\00\00\00\d1\01\10\00-\00\00\00\c3\00\00\00\18\00\00\00\d1\01\10\00-\00\00\00\e4\00\00\00'\00\00\00\d1\01\10\00-\00\00\00\e9\00\00\00\19\00\00\00..\00\00\d1\01\10\00-\00\00\00\09\01\00\00#\00\00\00internal error: entered unreachable code\d1\01\10\00-\00\00\00\01\01\00\00\12\00\00\00\d1\01\10\00-\00\00\00\8c\00\00\00*\00\00\00\d1\01\10\00-\00\00\00\92\00\00\005\00\00\00\d1\01\10\00-\00\00\00^\00\00\00 \00\00\00\d1\01\10\00-\00\00\00\86\00\00\00\11\00\00\00\5c\02\10\002\00\00\00\a5\00\00\00 \00\00\00\5c\02\10\002\00\00\00\b1\00\00\00\0d\00\00\00\d1\01\10\00-\00\00\00T\00\00\00 \00\00\00\5c\02\10\002\00\00\00\fc\00\00\00 \00\00\00\01\00\00\00internal error: entered unreachable code\dc\02\10\00&\00\00\00O\00\00\00\05\00\00\00\02\00\00\00\00\00\00\00\04\00\00\00\04\00\00\00\0b\00\00\00\0c\00\00\00\0c\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00a formatting trait implementation returned an error when the underlying stream did not\00\00\03\03\10\00I\00\00\00\88\02\00\00\11\00\00\00\0c\00\00\00\0c\00\00\00\04\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\0c\00\00\00\0c\00\00\00\04\00\00\00\13\00\00\00\14\00\00\00\15\00\00\00file name contained an unexpected NUL byte\00\00H\09\10\00*\00\00\00\14\00\00\00\02\00\00\00t\09\10\00main<unnamed>\00\00\00\8f\02\10\00L\00\00\00\16\01\00\00.\00\00\00\16\00\00\00\0c\00\00\00\04\00\00\00\17\00\00\00\18\00\00\00\19\00\00\00note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\0a\00\00\00\00\00\00\08\00\00\00\04\00\00\00\1a\00\00\00\1b\00\00\00\1c\00\00\00\1d\00\00\00\1e\00\00\00\10\00\00\00\04\00\00\00\1f\00\00\00 \00\00\00!\00\00\00\22\00\00\00m]\cb\d6,P\ebcxA\a6Wq\1b\8b\b9\e4\fd\9e\8f\ba8\e8\93\b7\8acNw\af%\80fatal runtime error: rwlock locked for writing, aborting\0aRUST_BACKTRACEcannot recursively acquire mutex\00\ff\01\10\00\5c\00\00\00\13\00\00\00\09\00\00\00advancing io slices beyond their length\00\03\03\10\00I\00\00\00Z\06\00\00\0d\00\00\00advancing IoSlice beyond its length\00\eb\03\10\00X\00\00\00\1f\00\00\00\0d\00\00\00\03\03\10\00I\00\00\00X\06\00\00 ")
  (data (;4;) (i32.const 1051496) "\01\00\00\00#\00\00\00$\00\00\00%\00\00\00&\00\00\00'\00\00\00(\00\00\00\09\00\00\00)\00\00\00\0c\00\00\00\04\00\00\00*\00\00\00+\00\00\00,\00\00\00-\00\00\00.\00\00\00/\00\00\000\00\00\00Box<dyn Any>thread caused non-unwinding panic. aborting.\0afailed to write whole buffer\00\00\00\e9\0b\10\00\1c\00\00\00\17\00\00\00\00\00\00\00\02\00\00\00\08\0c\10\00stack backtrace:\0anote: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.\0afailed to generate unique thread ID: bitspace exhausted\9e\03\10\00L\00\00\00&\00\00\00\0d\00\00\00\00\00\00\00\08\00\00\00\04\00\00\001\00\00\00\16\00\00\00\0c\00\00\00\04\00\00\002\00\00\00\03\03\10\00I\00\00\00Y\07\00\00$\00\00\00capacity overflow\00\00\00M\03\10\00P\00\00\00\1c\00\00\00\05\00\00\00called `Option::unwrap()` on a `None` value==!=matches00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899falsetrue\00O\0d\10\00Q\0d\10\00S\0d\10\00\02\00\00\00\02\00\00\00\07")
  (data (;5;) (i32.const 1052472) "\01")
  (data (;6;) (i32.const 1052484) "\01\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02")
  (data (;7;) (i32.const 1052592) "\01\00\00\00\ff\ff\ff\ffD\04\10")
  (data (;8;) (i32.const 1114128) "Hello from C!\0a")
  (data (;9;) (i32.const 1114112) "\10\00\00\00\0e")
  (@producers
    (language "Rust" "")
    (language "C11" "")
    (processed-by "rustc" "1.95.0 (59807616e 2026-04-14)")
    (processed-by "clang" "21.1.4-wasi-sdk (https://github.com/llvm/llvm-project 222fc11f2b8f25f6a0f4976272ef1bb7bf49521d)")
    (processed-by "walrus" "0.26.1")
    (processed-by "wasi-virt-layer" "0.2.9")
    (processed-by "wit-component" "0.246.1")
    (processed-by "wit-bindgen-rust" "0.55.0")
  )
  (@custom "target_features" (after data) "\0a+\07atomics+\0fmutable-globals+\13nontrapping-fptoint+\0bbulk-memory+\08sign-ext+\0freference-types+\0amultivalue+\0eextended-const+\0fbulk-memory-opt+\16call-indirect-overlong")
)
