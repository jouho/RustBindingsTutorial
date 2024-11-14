use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "hello")]
extern "C" {
    fn hello() -> *const c_char;
}

fn main() {
    // We need to mark the call to the C functions as unsafe, since Rust cannot
    // guarantee the safety of external C functions.
    let result = unsafe { CStr::from_ptr(hello()).to_str() };
    println!("{:#?}", result.unwrap());
}
