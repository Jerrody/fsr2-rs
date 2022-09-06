mod ffi;

fn main() {
    let mut x = std::ptr::null_mut();
    let y = std::ptr::null();

    unsafe {
        let x = ffi::ffxFsr2ContextCreate(x, y);

        println!("{x}");
    }
}
