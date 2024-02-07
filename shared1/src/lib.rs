use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use sharedinterface::{Shared1Lib, Shared1LibRef};

#[export_root_module]
pub fn get_library() -> Shared1LibRef {
    Shared1Lib { entry_point }.leak_into_prefix()
}

#[sabi_extern_fn]
fn entry_point() {
    println!("Executing shared1 stable entry point");
}
