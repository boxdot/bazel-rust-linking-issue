extern "C" {
    fn rust_function();
}

fn main() {
    unsafe { rust_function() };
}
