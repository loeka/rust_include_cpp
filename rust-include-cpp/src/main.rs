use std::ffi::{CStr, CString};
use libc::c_char;

extern crate libc;

extern "C" {
    fn add_one(a: libc::c_int) -> libc::c_int;
    fn hello_cout();
    fn hello_from_c() -> *const c_char;
    fn reply(str: *const c_char) -> *const c_char;
    fn free_string_reply(str: *const c_char);
    fn reply2(str: *const c_char) -> *const c_char;
    fn reply3(buffer: *mut c_char, len: libc::c_uint);
}

fn main() {
    println!("Return value of add_one(): {}", unsafe{ add_one(1) });

    unsafe{ hello_cout(); }

    let c_str_ptr = unsafe { hello_from_c() };
    let s: &CStr = unsafe { CStr::from_ptr(c_str_ptr) };
    println!("{:?}", s.to_str());

    println!("########## string 1 ##########");
    let input = CString::new("Hello!").expect("CString::new failed");
    let c_str = unsafe {
        reply(input.as_ptr())
    };
    let tmp: String = unsafe {
        let x = CStr::from_ptr(c_str).to_str().unwrap().to_owned();
        free_string_reply(c_str);
        println!("Pointer to c_str, created by reply(), freed. c_str: {:?} - {:?}", c_str, CStr::from_ptr(c_str));
        x
    };
    println!("{:?}", tmp);

    println!("########## string 2 ##########");
    let c_str = unsafe {
        reply2(input.as_ptr())
    };
    let tmp: &CStr = unsafe { CStr::from_ptr(c_str) };
    println!("{:?}", tmp);

    println!("########## string 3 ##########");
    const LEN: u32 = 16;
    let buffer = [0u8; LEN as usize];
    unsafe {
        reply3(buffer.as_ptr() as *mut i8, LEN);
        let c_str = CStr::from_bytes_until_nul(&buffer[..]).unwrap();
        println!("{:?}", c_str);
    }
}
