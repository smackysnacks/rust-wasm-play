use std::ffi::CString;
use std::os::raw::c_char;

mod js;

#[no_mangle]
pub fn example() -> i32 {
    42
}

#[no_mangle]
pub fn hello() -> *mut c_char {
    CString::new("Hello, world!").unwrap().into_raw()
}

#[no_mangle]
pub fn call_js() {
    js::alert("Hello, world!");
    js::log("Hello, world!");
}
