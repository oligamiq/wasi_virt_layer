
use std::hash::BuildHasherDefault;
use rustc_hash::FxHasher;

fn test() {
    let _: BuildHasherDefault<FxHasher> = BuildHasherDefault::default();
}
