//! Pulseaudio rust abstraction

use libc::{c_int, c_char, size_t, free, c_void};
use std::ptr;
use std::ffi::{CString, CStr};
use std::str::from_utf8;

#[link(name = "pulse-simple")]
#[link(name = "pulse")]
#[allow(improper_ctypes)]
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
    fn pa_simple_flush(pa: *mut pa_simple, error: *mut c_int) -> c_int;
}

#[repr(C)]
/// Counterpart to the connection object in pulse/simple.h
struct pa_simple;

// see pulse/def.h
#[repr(C)]
struct pa_sample_spec {
    format: c_int,
    rate: u32,
    channels: u8
}

// see pa_sample_format
static PA_SAMPLE_S16LE: c_int = 3_i32;

// defined as enum pa_stream_direction
#[allow(unused)]
static PA_STREAM_NODIRECTION: c_int = 0_i32;
#[allow(unused)]
static PA_STREAM_PLAYBACK:    c_int = 1_i32;
#[allow(unused)]
static PA_STREAM_UPLOAD:      c_int = 3_i32;
static PA_STREAM_RECORD:      c_int = 2_i32;

/// Rust wrapper over simple pulseaudio structure.
pub struct PulseAudio {
    ptr: *mut pa_simple,
}

impl PulseAudio {
    /// Creates a new PulseAudio.
    pub fn new(
        pa_name: &str,
        stream_name: &str,
        sample_rate: usize,
    ) -> PulseAudio {
        let mut err: c_int = 0;

        let mut s_spec = pa_sample_spec{
            format: PA_SAMPLE_S16LE,
            rate: sample_rate as u32,
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

    /// Reads samples into *buf*.
    pub fn sample(&mut self, buf: &mut [i16]) {
        let mut err: c_int = 0;

        unsafe {
            pa_simple_read(self.ptr, buf.as_mut_ptr(),
                           (buf.len() * 2) as size_t, &mut err);
            PulseAudio::handle_error(err);
        }
    }

    /// Supposedly flushes all read and write buffers.
    pub fn flush(&mut self) {
        let mut err: c_int = 0;
        unsafe {
            assert!(0 == pa_simple_flush(self.ptr, &mut err));
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
