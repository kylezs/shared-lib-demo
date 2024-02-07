extern crate sharedinterface;

use sharedinterface::{load_root_module_in_directory, Shared1LibRef};

fn main() {
    println!("Executing runner entry point");

    // 2. Deleting the target directory and then compiling results in error. Second time succeeds. This is likely due to compilation order.
    let library: Shared1LibRef = load_root_module_in_directory("./target/debug".as_ref())
        .unwrap_or_else(|e| panic!("{}", e));

    library.entry_point().unwrap()();

    println!("Finished executing runner entry point");
}
