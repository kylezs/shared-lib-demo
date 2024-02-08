#[repr(C)]
#[derive(Debug)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[no_mangle]
pub extern "C" fn entry_point(n: u32, e: MyEnum) -> MyEnum {
    println!(
        "Executing shared1 entry point from REALLY NEW rust version. n: {}, e: {:?}",
        n, e
    );
    MyEnum::B
}
