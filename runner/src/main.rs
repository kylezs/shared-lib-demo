// For the first shared library
#[repr(C)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[repr(C)]
#[derive(Debug)]
pub struct MyStruct {
    a: u32,
    b: f64,
}

extern "C" {
    pub fn entry_point(n: u32, e: MyEnum) -> MyStruct;
}

fn main() {
    println!("Executing runner entry point");

    unsafe {
        let x = entry_point(23, MyEnum::A);
        println!("Result from shared1: {:?}", x);
    }
}
