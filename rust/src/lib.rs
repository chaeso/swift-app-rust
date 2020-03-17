use std::os::raw::{c_char};
use std::ffi::{CString, CStr};


#[no_mangle]
pub extern fn rust_hello(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_check_encrypt_request() -> *mut c_char {
    CString::new("it_is_plain_text_for_encryption".to_owned()).unwrap().into_raw()
}

#[no_mangle]
pub extern fn rust_hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        let kk = CString::from_raw(s);
        println!("{}", kk.to_str().unwrap());
    };
}

#[cfg(test)]
mod tests {
    use crate::rust_hello;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("done");

        // rust_hello(CString::from("aaa").as_c_str());
    }
}

