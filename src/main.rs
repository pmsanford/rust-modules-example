// Modules in Rust can be single files, like file_mod.rs, folders
// that contain a mod.rs file in the folder root, like folder_mod,
// or specified inline inside other modules, like code_mod.
//
// For file modules, the module name is the file name without the .rs
// extension. For folders, the module name is the folder name.

// I have to declare the top-level modules in the main file, which
// is typically main.rs for binaries and lib.rs for libraries.
// For the following three modules, rust implicitly looks for .rs files
// or folders with these names.
mod file_mod;
mod folder_mod;
// I have to declare modules I want to use in order for them to be
// compiled as part of the crate, even if I don't import things from them
// in the file from which I declare them.
mod other_mod;

// I can also declare a module and its contents inline in a file
mod code_mod {
    pub struct CodeMod;
}

// To see how the following are declared, check the other files in this
// repo. To avoid distraction, all the things I'm importing are unit
// structs, which means they have no fields. Following Rust convention,
// the identifiers in CamelCase are struct names, and the identifiers in
// snake_case are module names.

// I can use things from a module that's a single file
use file_mod::FileMod;
use file_mod::SubFile;

// I can use things from a module specified above
use code_mod::CodeMod;

// I can use things from the mod.rs of a folder module
use folder_mod::FolderMod;
// I can use things that are re-exported from that folder module,
// even from private submodules.
use folder_mod::OtherMod;
use folder_mod::PrivateMod;
use folder_mod::SubMod;
// I can also use things directly from public submodules that are not re-exported
use folder_mod::submod::SubMod as SubMod2;

// Or I can use a whole module and specify paths at point of use
use folder_mod::submod;

fn main() {
    // I can use them all in code
    let fim = FileMod;
    let sf = SubFile;
    let cm = CodeMod;
    let fom = FolderMod;
    let pm = PrivateMod;
    let om = OtherMod;

    // The following three are equivalent
    let sm = SubMod;
    let sm2 = SubMod2;
    let sm3 = submod::SubMod;
}
