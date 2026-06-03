use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    str::FromStr,
    sync::atomic::AtomicUsize,
};

use compact_str::CompactString;
use eyre::{Context as _, ContextCompat as _};
use itertools::Itertools;


use crate::{
    args::TargetMemoryType,
    unique_name::UniqueNameMarker,
};

#[allow(dead_code)]




#[allow(dead_code)]


#[allow(dead_code)]










/// Extension trait for Camino paths to extract the main module file name.
pub trait CaminoUtilModule {
    /// Gets the base compact string name without extension and specific postfixes.
    fn get_file_main_name(&self) -> Option<CompactString>;
}

impl CaminoUtilModule for camino::Utf8Path {
    fn get_file_main_name(&self) -> Option<CompactString> {
        let binding = self.file_name().unwrap().split(".").collect::<Vec<_>>();
        let file_name_poss = binding.iter().rev();
        let mut file_name = None;
        for name in file_name_poss {
            if *name == "opt"
                || *name == "adjusted"
                || *name == "wasm"
                || *name == "core"
                || *name == "component"
            {
                continue;
            }
            file_name = Some(name);
            break;
        }
        let file_name = file_name.map(ToOwned::to_owned).or_else(|| {
            self.file_name()
                .unwrap()
                .split(".")
                .next()
                .as_ref()
                .cloned()
        });

        file_name.map(CompactString::from)
    }
}

impl CaminoUtilModule for PathBuf {
    fn get_file_main_name(&self) -> Option<CompactString> {
        camino::Utf8Path::new(self.to_str().unwrap()).get_file_main_name()
    }
}

impl CaminoUtilModule for Path {
    fn get_file_main_name(&self) -> Option<CompactString> {
        camino::Utf8Path::new(self.to_str().unwrap()).get_file_main_name()
    }
}

/// Utility trait for converting generic or `anyhow` results into `eyre::Result`.
pub trait ResultUtil<T> {
    /// Converts the result into an `eyre::Result`.
    fn to_eyre(self) -> eyre::Result<T>;
}

// https://github.com/eyre-rs/eyre/issues/31
impl<T> ResultUtil<T> for anyhow::Result<T> {
    fn to_eyre(self) -> eyre::Result<T> {
        self.map_err(|e| {
            eyre::eyre!(Box::<dyn std::error::Error + Send + Sync + 'static>::from(
                e
            ))
        })
    }
}

impl<T, I: Iterator> ResultUtil<T> for Result<T, itertools::ExactlyOneError<I>> {
    fn to_eyre(self) -> eyre::Result<T> {
        self.map_err(|e| eyre::eyre!(e.to_string()))
    }
}



/// Provides unified Function ID resolution for different markers (like tuples of module/name).


/// Provides the actual lookup operations for FID resolution.


/// Marker for looking up FIDs by `FunctionId` values.
pub struct FunctionIdMarker;
/// Marker for looking up FIDs by string names.
pub struct StrMarker;
/// Marker for looking up FIDs using `UniqueName` constants.
pub struct UniqueMarker;
/// Marker for looking up FIDs by a module-name and item-name tuple.
pub struct DoubleStrMarker;
/// Marker for looking up FIDs using a string module-name and `UniqueName` item.
pub struct StrAndUniqueNameMarker;

















// pub fn init_data_set(buff: &mut walrus::ModuleData, offset: u32, data: &[u8]) -> eyre::Result<()> {
//     let data_ids = buff.iter().map(|data| data.id()).collect::<Vec<_>>();

//     for id in data_ids {
//         let data = buff.get_mut(id);
//         if let walrus::DataKind::Active(walrus::ActiveData {
//             memory: _,
//             offset: walrus::ir::Value::I32(current_offset),
//             ..
//         }) = &data.kind
//         {
//             let current_offset = *current_offset as u32;
//             if current_offset <= offset && offset < current_offset + data.value.len() as u32 {
//                 let start = (offset - current_offset) as usize;
//                 let end = std::cmp::min(start + data.value.len(), start + data.len());
//                 data.value[start..end].copy_from_slice(&data[..(end - start)]);
//                 return Ok(());
//             }
//         }
//     }

//     Ok(())
// }

/// A container holding global statics for tracking Wasm Names.
#[derive(Debug)]
pub struct WasmNameHolder(&'static [compact_str::CompactString], &'static AtomicUsize);

impl WasmNameHolder {
    /// Creates a new `WasmNameHolder`, leaking the provided names into static memory.
    pub fn new(strings: Box<[compact_str::CompactString]>) -> Self {
        let count = Box::leak(Box::new(AtomicUsize::new(0)));

        let strings = Box::leak(strings);
        WasmNameHolder(strings, count)
    }

    /// Returns an iterator over the underlying tracked `WasmName` items.
    pub fn iter(&self) -> impl Iterator<Item = WasmName> {
        self.0.iter().map(|s| WasmName::new(s.as_str(), self.1))
    }
}

impl Drop for WasmNameHolder {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.0 as *const _ as *mut [compact_str::CompactString]);
            let count = self.1.load(std::sync::atomic::Ordering::SeqCst);
            let _ = Box::from_raw(self.1 as *const _ as *mut AtomicUsize);
            if count != 0 {
                panic!(
                    "WasmNameHolder dropped while there are still {count} WasmName instances alive"
                );
            }
        }
    }
}

/// Context-aware wrapper for a copied string slice with lifecycle tracking.
pub struct WasmName(&'static str, &'static AtomicUsize);
impl WasmName {
    /// Creates a new tracked `WasmName`.
    pub fn new(s: &'static str, counter: &'static AtomicUsize) -> Self {
        counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        WasmName(s, counter)
    }
}
impl Drop for WasmName {
    fn drop(&mut self) {
        self.1.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}
impl Clone for WasmName {
    fn clone(&self) -> Self {
        self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        WasmName(self.0, self.1)
    }
}
impl std::hash::Hash for WasmName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const str).hash(state);
    }
}
impl PartialEq for WasmName {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self.0 as *const _, other.0 as *const _)
    }
}
impl Eq for WasmName {}
impl std::fmt::Debug for WasmName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::fmt::Display for WasmName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl AsRef<str> for WasmName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl Borrow<str> for WasmName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

/// Generates a standardized static import string for WASM component architectures.
pub fn gen_component_name(namespace: &str, name: &str) -> String {
    format!("[static]{namespace}.{}-import", name.replace("_", "-"))
}

/// Iterator for enumerating combinations of boolean features.
#[derive(Debug)]
pub struct BitIterator {
    current: FeatureCombinationIteratorInnerBits,
    skip: FeatureCombinationIteratorInnerBits, // if this bit is set, skip this iteration
    kind: u8,
}

impl BitIterator {
    const MAX_KIND: u8 = core::mem::size_of::<FeatureCombinationIteratorInnerBits>() as u8 * 8;

    /// Constructs a new `BitIterator` for a given feature count.
    pub fn new(kind: u8) -> Self {
        if kind >= Self::MAX_KIND {
            panic!("Kind must be between 0 and {}", Self::MAX_KIND - 1);
        }

        BitIterator {
            current: FeatureCombinationIteratorInnerBits::ZERO,
            kind,
            skip: FeatureCombinationIteratorInnerBits::ZERO,
        }
    }

    /// Retrieves the current bit state representation.
    pub fn now(&self) -> FeatureCombinationIteratorInnerBits {
        self.current
    }

    /// Registers a specific bit index to be skipped during iteration.
    pub fn register_skip(&mut self, bit: u8) {
        if bit >= Self::MAX_KIND {
            panic!("Bit must be between 0 and {}", Self::MAX_KIND - 1);
        }
        self.skip.set(bit as usize, true);
    }

    /// Unregisters a raw mask of underlying bits from being skipped.
    pub fn skip_raw(&mut self, mask: FeatureCombinationIteratorInnerBits) {
        self.skip |= mask;
    }

    /// Unregisters a specific skipped bit index.
    pub fn unregister_skip(&mut self, bit: u8) {
        if bit >= Self::MAX_KIND {
            panic!("Bit must be between 0 and {}", Self::MAX_KIND - 1);
        }
        self.skip.set(bit as usize, false);
    }

    /// Unregisters an exact sequence of raw iteration bits.
    pub fn unregister_skip_raw(&mut self, mask: FeatureCombinationIteratorInnerBits) {
        self.skip &= !mask;
    }

    /// Clears any registered skip states.
    pub fn clear_skip(&mut self) {
        self.skip = FeatureCombinationIteratorInnerBits::ZERO;
    }
}

/// Underlying bit representations for the iterator generator.
pub mod bits {
    use bitvec::prelude::*;
    type FeatureCombinationIteratorInnerBitsInner = BitArray<[u64; 2]>;

    #[derive(Copy, Clone, Debug)]
    /// Strongly typed inner bit representation optimized for combination tracking.
    pub struct FeatureCombinationIteratorInnerBits(FeatureCombinationIteratorInnerBitsInner);

    impl core::ops::BitAnd for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn bitand(self, rhs: Self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: self.0 & rhs.0 }
        }
    }

    impl core::ops::BitAndAssign for FeatureCombinationIteratorInnerBits {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 &= rhs.0;
        }
    }

    impl core::ops::BitOr for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: self.0 | rhs.0 }
        }
    }

    impl core::ops::BitOrAssign for FeatureCombinationIteratorInnerBits {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0;
        }
    }

    impl core::ops::BitXor for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: self.0 ^ rhs.0 }
        }
    }

    impl core::ops::Not for FeatureCombinationIteratorInnerBits {
        type Output = Self;

        fn not(self) -> Self::Output {
            FeatureCombinationIteratorInnerBits { 0: !self.0 }
        }
    }

    impl core::ops::AddAssign for FeatureCombinationIteratorInnerBits {
        fn add_assign(&mut self, rhs: Self) {
            // arbitrary-precision integer
            let raw_lhs = &mut self.0.data;
            let raw_rhs = &rhs.0.data;
            let mut carry = 0u64;

            for i in 0..raw_lhs.len() {
                let (sum1, carry1) = raw_lhs[i].overflowing_add(raw_rhs[i]);
                let (sum2, carry2) = sum1.overflowing_add(carry);
                raw_lhs[i] = sum2;
                carry = (carry1 as u64) + (carry2 as u64);
            }
        }
    }

    impl core::cmp::PartialOrd for FeatureCombinationIteratorInnerBits {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl core::cmp::Ord for FeatureCombinationIteratorInnerBits {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }

    impl core::cmp::PartialEq for FeatureCombinationIteratorInnerBits {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl core::cmp::Eq for FeatureCombinationIteratorInnerBits {}

    impl core::ops::Index<usize> for FeatureCombinationIteratorInnerBits {
        type Output = bool;

        fn index(&self, index: usize) -> &bool {
            &self.0[index]
        }
    }

    impl FeatureCombinationIteratorInnerBits {
        /// Represents zero bits or no combinations active.
        pub const ZERO: Self = FeatureCombinationIteratorInnerBits {
            0: FeatureCombinationIteratorInnerBitsInner::ZERO,
        };
        /// Represents a standard first iteration or bit-start active.
        pub const ONE: Self = Self::from_number(1);

        /// Sets a specific position in the underlying representation.
        pub fn set(&mut self, index: usize, value: bool) {
            self.0.set(index, value);
        }

        /// Checks if this completely represents zeros.
        pub fn is_zero(&self) -> bool {
            self.0 == FeatureCombinationIteratorInnerBitsInner::ZERO
        }

        /// Checks if this is fully saturated.
        pub fn is_full(&self) -> bool {
            self.0.all()
        }

        /// Counts empty zeroes towards the most significant bit.
        pub fn leading_zeros(&self) -> usize {
            self.0.leading_zeros()
        }

        /// Counts empty zeroes towards the least significant bit.
        pub fn trailing_zeros(&self) -> usize {
            self.0.trailing_zeros()
        }

        /// Retrieves memory layout mapping to underlying pointers.
        pub fn as_raw_slice(&self) -> &[u64] {
            self.0.as_raw_slice()
        }

        /// Wraps static number initializations.
        pub const fn from_number(num: u64) -> Self {
            let mut bits: FeatureCombinationIteratorInnerBitsInner =
                FeatureCombinationIteratorInnerBitsInner::ZERO;
            bits.data[0] = num;
            FeatureCombinationIteratorInnerBits { 0: bits }
        }

        /// Generates an instance securely masked to the indicated position.
        pub fn from_one_pos(pos: usize) -> Self {
            let mut bits: FeatureCombinationIteratorInnerBitsInner =
                FeatureCombinationIteratorInnerBitsInner::ZERO;
            bits.set(pos, true);
            FeatureCombinationIteratorInnerBits { 0: bits }
        }

        /// Rapidly progresses to the next combination variant manually.
        pub fn increment(&mut self) {
            // arbitrary-precision integer
            let raw = &mut self.0.data;
            let mut carry = 1u64;

            for i in 0..raw.len() {
                let (new_value, new_carry) = raw[i].overflowing_add(carry);
                raw[i] = new_value;
                carry = if new_carry { 1 } else { 0 };
                if carry == 0 {
                    break;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::FeatureCombinationIteratorInnerBits;

        #[test]
        fn test_increment() {
            let mut s = FeatureCombinationIteratorInnerBits::ZERO;
            assert_eq!(s.leading_zeros(), 128);
            s.set(1, true);
            println!("{:?}", s.as_raw_slice());
            assert_eq!(s.leading_zeros(), 1);

            let mut a = FeatureCombinationIteratorInnerBits::ONE;
            a.increment();
            assert_eq!(a.as_raw_slice()[0], 2);

            let mut b = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            b.increment();
            assert_eq!(b.as_raw_slice()[0], 0);
            assert_eq!(b.as_raw_slice()[1], 1);

            let mut c = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            c.set(64, true);
            c.increment();
            assert_eq!(c.as_raw_slice()[0], 0);
            assert_eq!(c.as_raw_slice()[1], 2);
        }

        #[test]
        fn test_add_assign() {
            let mut a = FeatureCombinationIteratorInnerBits::from_number(1);
            let b = FeatureCombinationIteratorInnerBits::from_number(2);
            a += b;
            assert_eq!(a.as_raw_slice()[0], 3);

            let mut c = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            let d = FeatureCombinationIteratorInnerBits::from_number(1);
            c += d;
            assert_eq!(c.as_raw_slice()[0], 0);
            assert_eq!(c.as_raw_slice()[1], 1);

            let mut e = FeatureCombinationIteratorInnerBits::from_number(u64::MAX);
            e.set(64, true);
            let f = FeatureCombinationIteratorInnerBits::from_number(1);
            e += f;
            assert_eq!(e.as_raw_slice()[0], 0);
            assert_eq!(e.as_raw_slice()[1], 2);
        }
    }
}
pub use bits::FeatureCombinationIteratorInnerBits;

impl Iterator for BitIterator {
    type Item = FeatureCombinationIteratorInnerBits;

    fn next(&mut self) -> Option<Self::Item> {
        let count = core::mem::size_of::<FeatureCombinationIteratorInnerBits>() * 8
            - self.current.trailing_zeros();
        if count > self.kind as usize {
            None
        } else {
            let result = self.current;
            self.current.increment();

            loop {
                let flag = self.current & self.skip;
                if flag.is_zero() {
                    break;
                } else {
                    self.current +=
                        FeatureCombinationIteratorInnerBits::from_one_pos(flag.leading_zeros());
                }
            }
            Some(result)
        }
    }
}

#[derive(Debug)]
/// Facilitates multi-state combination testing iteration mapping arrays.
pub struct FeatureCombinationIterator<C: Borrow<T>, T: ?Sized> {
    features: Vec<(
        C,
        FeatureCombinationIteratorInnerBits,
        FeatureCombinationIteratorInnerBits,
    )>,
    current: BitIterator,
    __marker: std::marker::PhantomData<T>,
}

impl<'a, T: 'a + ?Sized, B: Borrow<T>, C: Borrow<T>, I: IntoIterator<Item = B>> FromIterator<(C, I)>
    for FeatureCombinationIterator<C, T>
where
    for<'c> &'c T: std::cmp::Eq + std::hash::Hash,
{
    fn from_iter<U: IntoIterator<Item = (C, I)>>(iter: U) -> Self {
        // What T refers to
        let data = iter
            .into_iter()
            .map(|(v, includes)| (v, includes.into_iter().collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        // Referring to T
        let num = {
            let mut counts = HashMap::new();
            // Initialize counts for all items to 0
            for (v, _) in &data {
                counts.insert(v.borrow(), 0isize);
            }
            // Count references
            for (_, inc) in &data {
                for v in inc {
                    if let Some(c) = counts.get_mut(&v.borrow()) {
                        *c += 1;
                    }
                }
            }

            data.iter()
                .map(|(v, _)| -*counts.get(&v.borrow()).unwrap_or(&0))
                .collect::<Vec<_>>()
        };

        let data = data.into_iter().zip(num).collect::<Vec<_>>();

        // TODO!(); fix with behavior change
        // data.sort_by_key(|(_, v)| *v);
        // Remove 'num' from data, but keep dependencies
        let mut data: Vec<(C, Vec<B>)> = data.into_iter().map(|(v, _)| v).collect();

        // Check for non-trivial cycles (mutual references)
        {
            let index_map: std::collections::HashMap<&T, usize> = data
                .iter()
                .enumerate()
                .map(|(i, (v, _))| (v.borrow(), i))
                .collect();

            let mut visited = vec![0u8; data.len()]; // 0: White, 1: Gray, 2: Black
            let mut stack = Vec::new();

            for i in 0..data.len() {
                if visited[i] != 0 {
                    continue;
                }

                stack.push((i, 0));
                visited[i] = 1; // Gray

                while let Some((u, dep_idx)) = stack.last_mut() {
                    let u = *u;
                    let deps = &data[u].1;
                    if *dep_idx < deps.len() {
                        let dep = &deps[*dep_idx];
                        *dep_idx += 1;

                        if let Some(&v) = index_map.get(&dep.borrow()) {
                            if u == v {
                                continue;
                            }
                            if visited[v] == 1 {
                                panic!("Mutual reference detected!");
                            }
                            if visited[v] == 0 {
                                visited[v] = 1;
                                stack.push((v, 0));
                            }
                        }
                    } else {
                        // Finished processing u
                        visited[u] = 2; // Black
                        stack.pop();
                    }
                }
            }
        }

        // Ensure that values referencing themselves are always placed on the right.
        // if sortable, swap with a value that does not reference itself so we check it.
        let mut count = 0i32;
        loop {
            let mut changed = false;
            for i in 0..data.len() {
                let mut swap_idx = None;
                {
                    let (ref_value, ref_includes) = &data[i];
                    if ref_includes
                        .iter()
                        .any(|v| v.borrow() == ref_value.borrow())
                    {
                        // Find a value to swap with
                        for j in (i + 1)..data.len() {
                            let (_, swap_includes) = &data[j];
                            if !swap_includes
                                .iter()
                                .any(|v| v.borrow() == ref_value.borrow())
                            {
                                swap_idx = Some(j);
                                break;
                            }
                        }
                    }
                }

                if let Some(j) = swap_idx {
                    data.swap(i, j);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
            if count.checked_add(1).is_none() {
                break;
            }
            count += 1;
            if count as u128 > 1u128.checked_shl(data.len() as u32).unwrap_or(u128::MAX) {
                panic!("Mutual reference detected!");
            }
        }

        let (data, _): (Vec<_>, Vec<_>) = data.into_iter().map(|v| (v, ())).unzip();

        let len = data.len();

        let map = data
            .iter()
            .enumerate()
            .map(|(i, (v, _))| (v.borrow(), i))
            .collect::<HashMap<&T, usize>>();

        // Rebuild dependents map from sorted data
        let dependents_map = {
            let mut dmap = data
                .iter()
                .map(|(v, _)| (v.borrow(), vec![]))
                .collect::<HashMap<&T, Vec<&C>>>();

            for (t, inc) in &data {
                for v in inc {
                    if let Some(list) = dmap.get_mut(&v.borrow()) {
                        list.push(t);
                    }
                }
            }
            dmap
        };

        let mut features_base = Vec::with_capacity(data.len());
        let mut dependencies_masks = Vec::with_capacity(data.len());
        for (v, inc) in &data {
            let deps = dependents_map
                .get(&v.borrow())
                .map(|v| v.as_slice())
                .unwrap_or(&[]);
            let mut mask = FeatureCombinationIteratorInnerBits::ZERO;
            let mut indices = Vec::new();

            for dep in deps {
                let idx = map[&(*dep).borrow()];
                mask.set(idx, true);
                indices.push(idx);
            }
            features_base.push((indices, mask));

            // Compute dependencies mask
            let mut dep_mask = FeatureCombinationIteratorInnerBits::ZERO;
            for d in inc {
                if let Some(idx) = map.get(&d.borrow()) {
                    dep_mask.set(*idx, true);
                }
            }
            dependencies_masks.push(dep_mask);
        }

        let mut features_masks: Vec<FeatureCombinationIteratorInnerBits> =
            features_base.iter().map(|(_, m)| *m).collect();

        for _ in 0..data.len() {
            let mut changed = false;
            for i in 0..data.len() {
                let (indices, _) = &features_base[i];
                let mut mask = features_masks[i];
                for &dep_idx in indices {
                    mask |= features_masks[dep_idx];
                    if mask != features_masks[i] { // Check against current stored mask, not 'old' local var if we updated it?
                        // Logic: mask |= dep_mask.
                        // If mask grew, changed=true.
                    }
                }
                if mask != features_masks[i] {
                    features_masks[i] = mask;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        let features = features_masks
            .into_iter()
            .zip(dependencies_masks)
            .zip(data)
            .map(|((mask, dep_mask), (v, _))| (v, mask, dep_mask))
            .collect::<Vec<_>>();

        FeatureCombinationIterator {
            features,
            current: BitIterator::new(len as u8),
            __marker: std::marker::PhantomData,
        }
    }
}

impl<C: Borrow<T> + std::cmp::Eq + std::hash::Hash + Clone, T: ?Sized> Iterator
    for FeatureCombinationIterator<C, T>
{
    type Item = std::collections::HashSet<C>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let bits = self.current.now();

            // Check termination
            let count = core::mem::size_of::<FeatureCombinationIteratorInnerBits>() * 8
                - bits.trailing_zeros();
            if count > self.current.kind as usize {
                return None;
            }

            // Compute skip mask (dependents of absent features)
            let mut _mask = FeatureCombinationIteratorInnerBits::ZERO;
            for (i, (_, feature_bits, _)) in self.features.iter().enumerate() {
                if !bits[i] {
                    // Feature i is absent
                    _mask |= *feature_bits; // Dependents of i are forbidden
                }
            }

            // Compute skip mask (dependents of absent features)
            // AND resolve violations (features present without dependencies)
            let _mask = FeatureCombinationIteratorInnerBits::ZERO;

            // Check for Forbidden features (because dependency is missing)
            // This is equivalent to checking "Present features have all dependencies".

            // Strategy: Iterate all features.
            // If feature i is PRESENT:
            //    Check dependencies_masks[i].
            //    If ANY dependency d is ABSENT -> Violation.
            //    violation |= (1 << i).
            //    To resolve: Either Clear i (add 1<<i) OR Set d (add 1<<d).
            //    We should pick the one that adds the LEAST to current.
            //    If d < i: Set d.
            //    If d > i: Clear i.
            //    Also if Clear i, we might need to carry.

            // We can compute minimum jump.

            let mut min_jump = None;

            for (i, (_, _, dependencies)) in self.features.iter().enumerate() {
                if bits[i] {
                    // Feature i is Present.
                    // Dependencies must be Present.
                    // Missing = dependencies & !bits.
                    let missing = *dependencies & !bits;
                    if !missing.is_zero() {
                        // Violation! Feature i needs missing dependencies.
                        // Options:
                        // 1. Clear i (add 1<<i).
                        // 2. Set d (for each d in missing). (add distance to d).

                        // Option 1: Clear i.

                        // Option 2: Set d.
                        // For each d in missing:
                        //   d_pos = trailing_zeros(d)?
                        //   d_jump = 1<<d_pos - (bits & low_mask)?
                        //   Wait. next_valid(bits, d) = (bits | (1<<d)) & !((1<<d)-1).
                        //   jump = next_valid - bits.
                        //   Or simplistically: 1<<d?
                        //   If bits has lower bits set, 1<<d might not be enough or too much?
                        //   Actually, we want to reach the *next* state where d=1.
                        //   (bits | (1<<d)) & !((1<<d)-1).

                        // Let's implement calculate_jump(current, target_bit).
                        // But FeatureCombinationInnerBits doesn't expose arithmetic easily.
                        // It has `from_one_pos`.
                        // It has `+`.

                        // If d < i.
                        // Then bits[d]=0. bits[i]=1.
                        // We want d=1.
                        // Since d < i, d is a lower bit.
                        // If we increment, d will toggle soon.
                        // E.g. d=0. 0->1 is +1.
                        // d=1. 00->10 is +2 (if 00).

                        // If we just track "smallest bit that needs to change".
                        // If any d < i.
                        // Then we assume natural increment will handle it?
                        // But we want to SKIP invalid states.
                        // If d < i. We can jump to next d=1.
                        // If d > i. We MUST clear i. (Jump to next i=0).

                        // If d > i. Jump = 1<<i (Clear i).
                        // If d < i. Jump = Next d=1.
                        // Next d=1 is <= 1<<d (relative to cleared lower).

                        // Let's optimize:
                        // If ANY d > i: We MUST clear i.
                        // Jump = 1<<i.

                        // If ALL d < i:
                        // We can wait for d.
                        // Can we skip to d?
                        // Yes. Jump to smallest d.
                        // But we can't easily compute "Jump to d" with abstract bits.
                        // But if d < i, and we assume Lsb0.
                        // 1<<d is smaller than 1<<i.
                        // So jump is smaller?
                        // If we iterate violations and pick minimum 1<<pos.
                        // If we pick 1<<i (Clear i).
                        // If we pick 1<<d (Set d? No, 1<<d might not set d correctly if lower bits are messy).
                        // But generally, adding 1<<d will toggle d (0->1) and clear lower.
                        // So adding 1<<d IS correct to jump to next d=1.

                        // So:
                        // Candidates:
                        // 1. 1<<i.
                        // 2. 1<<d (for all d in missing).

                        // Pick the SMALLEST candidate (lowest index).
                        // And apply it.

                        // If we find MULTIPLE violations.
                        // We should pick the global minimum jump.

                        // So loop over all i.
                        // Collect candidates.
                        // Pick min.

                        // Candidate from i: i.
                        // Candidates from missing: d's.

                        // Wait. If d < i.
                        // Should we set d or clear i?
                        // If we set d (jump 1<<d), we keep i set. Result valid (i=1, d=1).
                        // If we clear i (jump 1<<i), we get i=0. Result valid (i=0, d=0).
                        // Which is next?
                        // 1<<d is smaller. So we set d.

                        // If d > i.
                        // Set d (jump 1<<d). Keep i set.
                        // Clear i (jump 1<<i).
                        // 1<<i is smaller. So we clear i.

                        // So strategy:
                        // Collect all `i` (violation bits) and all `d` (missing dependencies).
                        // Find the MINIMUM index `m` among them.
                        // Add `1 << m`.

                        // Example: i=1 (B). d=0 (A).
                        // Min(1, 0) = 0.
                        // Add 1<<0 (1).
                        // 0010 + 1 = 0011 (A=1, B=1). Correct.

                        // Example: i=0 (A). d=2 (C). (If A depended on C).
                        // Min(0, 2) = 0.
                        // Add 1<<0 (1).
                        // 0001 + 1 = 0010 (A=0). Correct.

                        // This logic is beautiful.
                        // Just find the lowest bit involved in any violation (either the feature itself or its missing dependency).
                        // And add 1 << that bit.

                        // Iterate bits of missing.

                        // Let's accumulate a "jump_mask".
                        // jump_mask |= (1 << i).
                        // jump_mask |= missing.

                        if min_jump.is_none() {
                            min_jump = Some(FeatureCombinationIteratorInnerBits::ZERO);
                        }
                        if let Some(ref mut j) = min_jump {
                            j.set(i, true);
                            *j |= missing;
                        }
                    }
                }
            }

            if let Some(jump_mask) = min_jump {
                // Found violations.
                // We want smallest bit in jump_mask.
                // Wait. trailing_zeros counts from MSB in BitArray (Lsb0)???
                // NO. I decided earlier it counted from MSB.
                // But leading_zeros counted from LSB?
                // `from_one_pos` uses index.
                // If I want index of lowest bit.
                // If `Lsb0`: Index 0 is lowest.
                // If `trailing_zeros` counts from End (127).
                // `leading_zeros` counts from Start (0).
                // So I want `leading_zeros`.

                // Let's use `leading_zeros`.
                let bit = jump_mask.leading_zeros(); // Index of first set bit (lowest index).
                self.current.current += FeatureCombinationIteratorInnerBits::from_one_pos(bit);

                // Continue loop
            } else {
                // Valid!
                // ... return result ...
                let mut result = std::collections::HashSet::new();
                for (i, (feature, _, _)) in self.features.iter().enumerate() {
                    if bits[i] {
                        result.insert(feature.clone());
                    }
                }
                self.current.current.increment();
                return Some(result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use super::*;

    #[test]
    fn test_get_file_main_name() {
        let path = camino::Utf8Path::new("name.opt.adjusted.wasm");
        let file_name = path.get_file_main_name();
        assert_eq!(file_name.unwrap(), "name");
    }

    #[test]
    fn test_bit_generator() {
        let bits = BitIterator::new(3).collect::<Vec<_>>();

        assert_eq!(
            &bits
                .iter()
                .map(|b| b.as_raw_slice()[0])
                .collect::<Vec<u64>>(),
            &[0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111]
        );

        let count = BitIterator::new(10).count();
        assert_eq!(count, 1024);

        let mut generator = BitIterator::new(5);
        generator.register_skip(1);
        generator.register_skip(3);
        let bits = generator.collect::<Vec<_>>();

        assert_eq!(
            &bits
                .iter()
                .map(|b| b.as_raw_slice()[0])
                .collect::<Vec<u64>>(),
            &[
                0b00000, 0b00001, 0b00100, 0b00101, 0b10000, 0b10001, 0b10100, 0b10101
            ]
        );
    }

    #[test]
    fn test_feature_combination_iterator() {
        let data = vec![
            ("A", vec![]),
            ("B", vec!["A"]),
            ("C", vec!["A"]),
            ("D", vec!["B", "C"]),
        ];

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, str>>();

        println!("Iterator created: {:?}", iterator);

        let combinations = iterator.collect::<Vec<_>>();

        let expected = vec![
            HashSet::from([]),
            HashSet::from(["A"]),
            HashSet::from(["A", "B"]),
            HashSet::from(["A", "C"]),
            HashSet::from(["A", "B", "C"]),
            HashSet::from(["A", "B", "C", "D"]),
        ];

        assert_eq!(combinations, expected);

        let data = vec![
            (String::from("A"), vec![]),
            (String::from("B"), vec![String::from("A")]),
            (String::from("C"), vec![String::from("A")]),
            (
                String::from("D"),
                vec![String::from("B"), String::from("C")],
            ),
        ];
        let data_ref = data
            .iter()
            .map(|(v, inc)| (v, inc.iter().map(|s| s).collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        let iterator = data
            .clone()
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        let iterator2 = data_ref
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        println!("Iterator created: {:?}", iterator);

        let combinations = iterator.collect::<Vec<_>>();
        let _combinations2 = iterator2.collect::<Vec<_>>();

        let expected = vec![
            HashSet::from([]),
            HashSet::from([String::from("A")]),
            HashSet::from([String::from("A"), String::from("B")]),
            HashSet::from([String::from("A"), String::from("C")]),
            HashSet::from([String::from("A"), String::from("B"), String::from("C")]),
            HashSet::from([
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
            ]),
        ];

        assert_eq!(combinations, expected);

        let data = data
            .iter()
            .map(|(u, v)| (Arc::new(u.clone()), v))
            .collect::<Vec<_>>();

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        println!("Iterator created: {:?}", iterator);

        let _combinations = iterator.collect::<Vec<_>>();
    }

    #[test]
    #[should_panic(expected = "Mutual reference detected!")]
    fn test_feature_combination_iterator_mutual_ref() {
        // A includes B
        // B includes A
        // Cycle!
        let data = vec![
            (String::from("A"), vec![String::from("B")]),
            (String::from("B"), vec![String::from("A")]),
        ];

        let _iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();
    }

    #[test]
    #[should_panic(expected = "Mutual reference detected!")]
    fn test_feature_combination_iterator_complex_cycle() {
        // A -> B -> C -> A
        let data = vec![
            (String::from("A"), vec![String::from("B")]),
            (String::from("B"), vec![String::from("C")]),
            (String::from("C"), vec![String::from("A")]),
        ];

        let _iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();
    }

    #[test]
    fn test_feature_combination_iterator_complex_valid() {
        // Valid graph:
        // A -> B, C
        // B -> D
        // C -> D
        // D -> []
        // Order should handle this (D comes first, then B/C, then A)

        let data = vec![
            (
                String::from("A"),
                vec![String::from("B"), String::from("C")],
            ),
            (String::from("B"), vec![String::from("D")]),
            (String::from("C"), vec![String::from("D")]),
            (String::from("D"), vec![]),
        ];

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        let combinations = iterator.collect::<Vec<_>>();
        // If it didn't panic and produced combinations, the topological sort/check worked for this valid case.
        assert!(combinations.len() > 0);
    }

    #[test]
    fn test_feature_combination_iterator_complex_diamond() {
        use std::collections::{HashMap, HashSet};
        // Diamond:
        // Root -> Left, Right
        // Left -> Base
        // Right -> Base
        // Base -> []

        let data = vec![
            (
                String::from("Root"),
                vec![String::from("Left"), String::from("Right")],
            ),
            (String::from("Left"), vec![String::from("Base")]),
            (String::from("Right"), vec![String::from("Base")]),
            (String::from("Base"), vec![]),
        ];

        let iterator = data
            .into_iter()
            .collect::<FeatureCombinationIterator<_, String>>();

        let combinations: Vec<HashSet<String>> = iterator.collect();

        // Verify that for every combination, if a feature is present, its dependencies are also present.
        // We can check this property for all combinations.

        // Dependency map for checking
        let deps: HashMap<String, Vec<String>> = HashMap::from([
            (
                String::from("Root"),
                vec![String::from("Left"), String::from("Right")],
            ),
            (String::from("Left"), vec![String::from("Base")]),
            (String::from("Right"), vec![String::from("Base")]),
            (String::from("Base"), vec![]),
        ]);

        for combo in &combinations {
            for feature in combo {
                if let Some(required) = deps.get(feature) {
                    for req in required {
                        assert!(
                            combo.contains(req),
                            "Combination {:?} invalid: {} requires {}",
                            combo,
                            feature,
                            req
                        );
                    }
                }
            }
        }

        // Also verify some known valid combinations are present
        assert!(combinations.contains(&HashSet::from([])));
        assert!(combinations.contains(&HashSet::from([String::from("Base")])));
        assert!(
            combinations.contains(&HashSet::from([String::from("Base"), String::from("Left")]))
        );
        assert!(combinations.contains(&HashSet::from([
            String::from("Base"),
            String::from("Left"),
            String::from("Right"),
            String::from("Root")
        ])));

        // Verify invalid ones are NOT present
        // Root only
        assert!(!combinations.contains(&HashSet::from([String::from("Root")])));
        // Left only
        assert!(!combinations.contains(&HashSet::from([String::from("Left")])));
    }
}
