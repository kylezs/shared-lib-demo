// For the first shared library

// Does a runtime check of the types. If the types are not the same, it will panic.
// so it panics *before* letting you call it.
#[stabby::import(name = "first_shared_lib")]
extern "C" {
    pub fn entry_point() -> u32;
}

fn main() {
    println!("Executing runner entry point");

    let x = entry_point();
    println!("Result from shared1: {}", x);
}
