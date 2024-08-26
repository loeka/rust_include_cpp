extern crate libc;

extern {
    fn add_one(a: libc::c_int) -> libc::c_int;
    fn add_two(a: libc::c_int) -> libc::c_int;
    fn hello();
}

fn main() {
    unsafe{ hello(); }

    println!("test1: {}", unsafe{ add_one(1) });
    println!("test2: {}", unsafe{ add_two(1) });
}