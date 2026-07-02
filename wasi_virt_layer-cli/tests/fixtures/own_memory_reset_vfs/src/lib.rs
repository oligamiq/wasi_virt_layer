use wasi_virt_layer::prelude::*;

struct ComponentABI;

wit_bindgen::generate!({
    world: "component-abi",
});

import_wasm!(own_memory_reset_target);

impl Guest for ComponentABI {
    fn main() {
        own_memory_reset_target::_reset();

        let initial = crate::memory_size::<own_memory_reset_target>();
        println!("own-memory reset initial logical size = {initial}");
        assert_eq!(initial, 1);

        let reserve = crate::memory_reserve::<own_memory_reset_target>(4);
        println!("own-memory reset reserve result = {reserve}");
        assert_ne!(reserve, -1);

        own_memory_reset_target::_start();
        own_memory_reset_target::_main();

        let grown = crate::memory_size::<own_memory_reset_target>();
        println!("own-memory reset grown logical size = {grown}");
        assert_eq!(grown, 3);

        own_memory_reset_target::_reset();

        let after_reset = crate::memory_size::<own_memory_reset_target>();
        println!("own-memory reset after-reset logical size = {after_reset}");
        assert_eq!(after_reset, initial);

        own_memory_reset_target::_main();

        let second_grown = crate::memory_size::<own_memory_reset_target>();
        println!("own-memory reset second-grown logical size = {second_grown}");
        assert_eq!(second_grown, grown);

        println!("own-memory reset logical-size test passed");
    }
}

#[cfg(not(test))]
export!(ComponentABI);

wasi_virt_layer::own_memory!(own_memory_reset_target);
