#[macro_use]
extern crate lazy_static;

use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicUsize, Ordering};

mod js;

lazy_static! {
    static ref CLICK_COUNT: AtomicUsize = AtomicUsize::new(1);
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
pub fn call_js() {
    js::alert("Hello, world!");
    js::log("Hello, world!");
}

#[no_mangle]
pub fn clickme_clicked() {
    let count = CLICK_COUNT.fetch_add(1, Ordering::Relaxed);
    js::log(&format!("Clicked {} time{}",
                     count,
                     if count == 1 { "" } else { "s" }));
}
