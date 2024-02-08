#[stabby::export]
pub extern "C" fn entry_point() {
    println!("Executing shared1 entry point with new rust version!");
}
