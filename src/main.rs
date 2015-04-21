#![feature(libc)]

extern crate libc;
use libc::{c_int, c_char, size_t};

#[link(name = "pulse-simple")]
#[link(name = "pulse")]
extern {
    fn pa_simple_new(server: *mut c_char, 
                     name: *mut c_char,
                     dir: c_int,
                     dev: *mut c_char,
                     steam_name: *mut c_char,
                     sample_spec: *mut pa_sample_spec,
                     channel_map: *mut u8,
                     attr: *mut u8,
                     error: *mut c_int) -> *mut pa_simple; 
}

// typedef struct pa_simple pa_simple 
pub struct pa_simple;

// see pulse/def.h
pub struct pa_sample_spec {
    format: c_int,
    rate: u32,
    channels: u8
} 

fn main() {
    let x = 10_i32;
    println!("max compressed length of a 100 byte buffer: {}", x);
}
