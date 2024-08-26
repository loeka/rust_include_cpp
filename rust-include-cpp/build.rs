extern crate bindgen;
extern crate cc;

fn main() {
    cc::Build::new()
        .file("cpplib/cpplib.cpp")
        .cpp(true)
        .compile("cpplib.a");
}
