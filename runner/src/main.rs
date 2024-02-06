// For the first shared library
mod libfirst_shared_lib {
    extern "C" {
        pub fn entry_point();
    }
}

// For the second shared library
mod libsecond_shared_lib {
    extern "C" {
        pub fn entry_point();
    }
}

fn main() {
    println!("Executing runner entry point");

    unsafe {
        libfirst_shared_lib::entry_point();
        libsecond_shared_lib::entry_point();
    }
}
