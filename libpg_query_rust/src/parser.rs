#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct PgErrorInfo {
    pub message: String,
    pub funcname: String,
    pub filename: String,
    pub lineno: i32,
    pub cursorpos: i32,
    pub context: String,
}

fn c_str_to_str(input: *mut ::std::os::raw::c_char) -> Option<String> {
    if input.is_null() {
        return None;
    }
    let c_str: &CStr = unsafe { CStr::from_ptr(input) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    return Some(str_buf);
}

pub fn parse_query(query: String) -> Result<String, PgErrorInfo> {
    let c_str = CString::new(query).unwrap();
    let c_ptr: *const c_char = c_str.as_ptr() as *const c_char;
    let parse_tree: *mut ::std::os::raw::c_char;
    unsafe {
        let output = pg_query_parse(c_ptr);
        parse_tree = output.parse_tree;

        if output.error.is_null() {
            return Ok(c_str_to_str(parse_tree).unwrap_or(String::from("[]")));
        }
        return Err(PgErrorInfo {
            message: c_str_to_str((*output.error).message).unwrap_or(String::from("none")),
            funcname: c_str_to_str((*output.error).funcname).unwrap_or(String::from("none")),
            filename: c_str_to_str((*output.error).filename).unwrap_or(String::from("none")),
            lineno: (*output.error).lineno,
            cursorpos: (*output.error).cursorpos,
            context: c_str_to_str((*output.error).context).unwrap_or(String::from("none")),
        });
    }
}

/*fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    match parse_query("ELECT 1") {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error: {}, File: {}, Func: {}, Context: {}", e.message, e.filename, e.funcname, e.context)
    }
}*/
