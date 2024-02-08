// For the first shared library

#[stabby::import(name = "first_shared_lib")]
extern "C" {
    pub fn entry_point();
}

fn main() {
    println!("Executing runner entry point");

    entry_point();
}
