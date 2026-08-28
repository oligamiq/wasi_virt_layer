use std::collections::HashMap;

/// Tracks indices for a specific Wasm entity space (e.g., functions, memories, globals, types).
/// When we inject new elements (like VFS imports or new globals), the original indices
/// in the Wasm file shift. This tracker maps original indices to new indices.
#[derive(Debug, Default)]
pub struct IndexTracker {
    /// Number of elements that existed in the original module.
    pub original_count: u32,
    /// Number of injected elements so far.
    pub injected_count: u32,
    /// Maps an original index to its new index if it was explicitly remapped.
    /// If not in the map, the new index is `original_index + injected_count_before_it`.
    /// For simplicity, we can assume all injections happen BEFORE existing elements
    /// (e.g. injecting imports pushes all internal functions up), or at the END.
    /// Let's support both.
    pub shift_offset: u32,
    /// Explicit mapping of original index to new index.
    pub explicit_map: HashMap<u32, u32>,
}

impl IndexTracker {
    /// Creates a new index tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark that we've seen `count` original elements.
    pub fn set_original_count(&mut self, count: u32) {
        self.original_count = count;
    }

    /// Register a new injected element. Returns its new assigned index.
    pub fn inject(&mut self) -> u32 {
        let new_idx = self.original_count + self.injected_count;
        self.injected_count += 1;
        new_idx
    }

    /// Remap an original index to its new index.
    pub fn remap(&self, original_index: u32) -> u32 {
        if let Some(&new_idx) = self.explicit_map.get(&original_index) {
            new_idx
        } else {
            original_index + self.shift_offset
        }
    }
}
