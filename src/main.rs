extern "C" {
    fn start_namespace();
}

fn main() {
    unsafe {
        start_namespace();
    }
    println!("Hello, world!");
}
