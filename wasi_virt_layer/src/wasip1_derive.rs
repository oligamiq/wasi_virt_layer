
/// Trait for formatting WASIP1 structs into debug representation.
pub trait Wasip1DebugTrait {
    /// Formats the struct.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

/// A wrapper struct for implementing custom debug formatting.
pub struct Debug<T>(pub T);

impl<T: Wasip1DebugTrait> Wasip1DebugTrait for Debug<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

// impl Wasip1DebugTrait for wasip1:: {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         f.debug_struct("Filestat")
//             .field("dev", &self.dev)
//             .field("ino", &self.ino)
//             .field("filetype", &self.filetype)
//             .field("nlink", &self.nlink)
//             .field("size", &self.size)
//             .field("atim", &self.atim)
//             .field("mtim", &self.mtim)
//             .field("ctim", &self.ctim)
//             .finish()
//     }
// }
