// File modules are pretty simple - anything pub is importable
pub struct FileMod;

// Submodules can also be explicitly specified in the code, like so:
mod subfile {
    pub struct SubFile;
}

pub use subfile::SubFile;
