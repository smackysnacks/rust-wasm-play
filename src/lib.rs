use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn alert(ptr: *const c_char);
}

#[no_mangle]
pub fn example() -> i32 {
    42
}

#[no_mangle]
pub fn hello() -> *mut c_char {
    CString::new("Hello, world!").unwrap().into_raw()
}

#[no_mangle]
pub fn call_alert() {
    unsafe {
        let s = CString::new("Hello, world!").unwrap();
        alert(s.as_ptr());
    }
}
