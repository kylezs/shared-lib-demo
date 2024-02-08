// For the first shared library
#[repr(C)]
pub enum MyEnum {
    A,
    B,
    C,
}

extern "C" {
    pub fn entry_point(n: u32, e: MyEnum) -> MyEnum;
}

fn main() {
    println!("Executing runner entry point");

    unsafe {
        entry_point(23, MyEnum::A);
    }
}
