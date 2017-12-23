#[macro_use]
extern crate lazy_static;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::slice;
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

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub fn fill(pointer: *mut u32, max_width: usize, max_height: usize, time: f64) {
    // pixels are stored in RGBA, so each pixel is 4 bytes
    let sl = unsafe { slice::from_raw_parts_mut(pointer, max_width*max_height) };

    for i in 0..max_width*max_height {
        // get the position of current pixel
        let height = i / max_width;
        let width  = i % max_width;

        let len = ((height*height + width*width) as f64).sqrt();
        let nb = time  + len / 4.0;
        let r = 128.0 + nb.cos() * 128.0;

        let width = 500 - width;
        let len = ((height*height + width*width) as f64).sqrt();
        let nb = time  + len / 4.0;
        let b = 128.0 + nb.cos() * 128.0;

        sl[i] = 0xff << 24 | (b as u8 as u32) << 16 | 0x00 << 8 | (r as u8 as u32);
    }
}