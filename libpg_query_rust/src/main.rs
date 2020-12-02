#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn parseQuery(query: &str) -> String {
    let c_str = CString::new(query).unwrap();
    let c_ptr: *const c_char = c_str.as_ptr() as *const c_char;
    let parse_tree: *mut ::std::os::raw::c_char;
    unsafe {
        parse_tree = pg_query_parse(c_ptr).parse_tree;
    }

    let c_str: &CStr = unsafe { CStr::from_ptr(parse_tree) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    return str_buf;
}

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("{}", parseQuery("SELECT 1"));
}
