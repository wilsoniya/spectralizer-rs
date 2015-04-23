use libc::{c_int, c_char, size_t};
use std::ptr;
use std::ffi::{CString, CStr};
use std::mem::{transmute, transmute_copy};
use std::str::from_utf8;

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
    fn pa_simple_read(pa: *mut pa_simple,
                      data: *mut i16,
                      num_bytes: size_t,
                      error: *mut u8) -> c_int;
    fn pa_strerror(error: u8) -> *mut c_char;

}

// typedef struct pa_simple pa_simple 
pub struct pa_simple;

// see pulse/def.h
pub struct pa_sample_spec {
    format: c_int,
    rate: u32,
    channels: u8
} 

// see pa_sample_format
pub static PA_SAMPLE_S16LE: c_int = 3_i32;

// defined as enum pa_stream_direction
pub static PA_STREAM_NODIRECTION: c_int = 0_i32;
pub static PA_STREAM_PLAYBACK:    c_int = 1_i32;
pub static PA_STREAM_RECORD:      c_int = 2_i32;
pub static PA_STREAM_UPLOAD:      c_int = 3_i32;

impl pa_simple {
    pub fn new(pa_name: &str, stream_name: &str) -> pa_simple {
        unsafe {
            let mut err: c_int = 0;

            let mut s_spec = pa_sample_spec{
                format: PA_SAMPLE_S16LE, 
                rate: 1000, 
                channels: 2};

            let pa = pa_simple_new(ptr::null_mut::<i8>() as *mut i8, 
                                   CString::new(pa_name).unwrap().as_ptr() as *mut i8,
                                   PA_STREAM_RECORD, 
                                   ptr::null_mut::<i8>() as *mut i8, 
                                   CString::new(stream_name).unwrap().as_ptr() as *mut i8,
                                   &mut s_spec,
                                   ptr::null_mut::<u8>() as *mut u8,
                                   ptr::null_mut::<u8>() as *mut u8,
                                   &mut err);
            if ( err != 0 ) {
//              panic!("err code {} from pulse: \"{}\"", 
//                    err, std::str::raw::from_c_str(pa_strerror(err)) );
            }
            transmute_copy(&pa)
        }
    }

    pub fn read_loop(&mut self) {
        loop { 
            println!("ASDF");
            let mut data = [0i16; 512];
            let mut err: u8 = 0;
            
            unsafe {
                pa_simple_read(self, data.as_mut_ptr(), 1024, &mut err);
                if err != 0 {
                    let err_msg = CStr::from_ptr(pa_strerror(err));
                    let err_msg: &str = from_utf8(err_msg.to_bytes()).unwrap();
                    panic!("err code {} from pulse: \"{}\"", err, err_msg);
                }
            }

            for i in 0..256 {
                println!("{}: {}, {}", i, data[i * 2], data[i * 2 + 1]);
            }

            println!("END");
        }
    }
}
