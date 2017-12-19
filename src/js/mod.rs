use std::ffi::CString;

mod imports;

pub fn alert(s: &CString) {
    unsafe {
        imports::alert(s.as_ptr());
    }
}
