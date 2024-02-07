use std::path::Path;

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    sabi_types::VersionStrings,
    StableAbi,
};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = Shared1LibRef)))]
pub struct Shared1Lib {
    pub entry_point: extern "C" fn(),
}

/// The RootModule trait defines how to load the root module of a library.
impl RootModule for Shared1LibRef {
    abi_stable::declare_root_module_statics! {Shared1LibRef}

    const BASE_NAME: &'static str = "shared1lib";
    const NAME: &'static str = "shared1lib";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

/// This loads the root from the library in the `directory` folder.
pub fn load_root_module_in_directory(directory: &Path) -> Result<Shared1LibRef, LibraryError> {
    println!("Loading shared1lib from directory: {:?}", directory);
    Shared1LibRef::load_from_directory(directory)
}
