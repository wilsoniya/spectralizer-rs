use libc::{c_int, c_char, size_t, free, c_void};
use std::ptr;
use std::ffi::{CString, CStr};
use std::str::from_utf8;

use time::{Timespec, get_time};

const SAMPLE_RATE: usize = 44100 / 1;
const BUF_SIZE: usize = 1100;
//const SAMPLE_RATE: usize = 44100 / 32;

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
                      error: *mut c_int) -> c_int;
    fn pa_strerror(error: c_int) -> *mut c_char;

}

// typedef struct pa_simple pa_simple
struct pa_simple;

pub struct PulseAudio {
    ptr: *mut pa_simple,
}

// see pulse/def.h
pub struct pa_sample_spec {
    format: c_int,
    rate: u32,
    channels: u8
}

// see pa_sample_format
pub static PA_SAMPLE_S16LE: c_int = 3_i32;

// defined as enum pa_stream_direction
static PA_STREAM_NODIRECTION: c_int = 0_i32;
static PA_STREAM_PLAYBACK:    c_int = 1_i32;
static PA_STREAM_RECORD:      c_int = 2_i32;
static PA_STREAM_UPLOAD:      c_int = 3_i32;

impl PulseAudio {
    pub fn new(pa_name: &str, stream_name: &str) -> PulseAudio {
        let mut err: c_int = 0;

        let mut s_spec = pa_sample_spec{
            format: PA_SAMPLE_S16LE,
            rate: SAMPLE_RATE as u32,
            channels: 1};

        unsafe {
            let pa = pa_simple_new(ptr::null_mut::<i8>() as *mut i8,
                                   CString::new(pa_name).unwrap().as_ptr() as *mut i8,
                                   PA_STREAM_RECORD,
                                   ptr::null_mut::<i8>() as *mut i8,
                                   CString::new(stream_name).unwrap().as_ptr() as *mut i8,
                                   &mut s_spec,
                                   ptr::null_mut::<u8>() as *mut u8,
                                   ptr::null_mut::<u8>() as *mut u8,
                                   &mut err);
            PulseAudio::handle_error(err);

            PulseAudio { ptr: pa }
        }
    }

    pub fn read_loop(&mut self) {
        loop {
            let t0 = get_time();
            let mut data = [0_i16; BUF_SIZE as usize];
            let mut err: c_int = 0;

            unsafe {
                pa_simple_read(self.ptr, data.as_mut_ptr(), BUF_SIZE as u64, &mut err);
                PulseAudio::handle_error(err);
            }

            for i in 0..(BUF_SIZE / 4) {
                println!("{}: {}, {}", i, data[i * 2], data[i * 2 + 1]);
            }
            let t1 = get_time();
            let delta = t1 - t0;
            println!("Duration: {}us", delta.num_microseconds().unwrap());

        }
    }

    pub fn sample(&mut self, buf: &mut [i16]) {
        let mut err: c_int = 0;

        unsafe {
            pa_simple_read(self.ptr, buf.as_mut_ptr(), (buf.len() * 2) as u64,
                           &mut err);
            PulseAudio::handle_error(err);
        }
    }

    unsafe fn handle_error(err_code: c_int) {
        if err_code != 0 {
            let err_msg = CStr::from_ptr(pa_strerror(err_code));
            let err_msg: &str = from_utf8(err_msg.to_bytes()).unwrap();
            panic!("err code {} from pulse: \"{}\"", err_code, err_msg);
        }
    }
}

impl Drop for PulseAudio {
    fn drop(&mut self) {
        unsafe {
            ptr::read(self.ptr);
            free(self.ptr as *mut c_void);
        }
    }
}
