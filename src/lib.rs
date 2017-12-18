use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub fn example() -> i32 {
    42
}

#[no_mangle]
pub fn hello() -> *mut c_char {
    CString::new("Hello, world!").unwrap().into_raw()
}
