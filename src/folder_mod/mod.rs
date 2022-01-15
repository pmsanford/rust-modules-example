// This submodule is public, so it can be used from outside this
// module.
pub mod submod;
// I can also re-export things from it and they can be used without
// an additional path segment.
pub use submod::SubMod;

// This one is not public, but...
mod private_mod;
// I can make things from inside it public by re-exporting them.
pub use private_mod::PrivateMod;

// I can also reference things from other peer modules by using the
// crate path qualifier. This starts searching from the crate root.
pub use crate::other_mod::OtherMod;

// I can also have things defined and exported directly from mod.rs
pub struct FolderMod;
