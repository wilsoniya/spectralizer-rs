//! Main module. Contains system entry point.

extern crate libc;
extern crate num;
extern crate sdl2;
extern crate time;

pub mod pulse;
pub mod fft;
pub mod vis;

use std::time::Duration;
use std::thread::sleep;

const BUF_SIZE: usize = 512;
const WIN_WIDTH: u32 = 256;
const WIN_HEIGHT: u32 = 192;
const SAMPLE_RATE: usize = 16384;
const FRAME_RATE: u64 = 60;

fn main() {
    let mut pa = pulse::PulseAudio::new("Spectralizer", "visualizer sink",
                                        SAMPLE_RATE);

    let mut buf   = [0i16; BUF_SIZE];
    let mut f_buf = [0f64; BUF_SIZE];
    let mut res   = [0f64; BUF_SIZE];

    let mut visualizer = vis::Visualizer::new("spectralizer", WIN_WIDTH,
                                              WIN_HEIGHT);

    let mut frame_start_ns: u64 = 0;

    loop {
        frame_start_ns = time::precise_time_ns();

        pa.sample(&mut buf[..]);

        for (i, &n) in buf.iter().enumerate() {
            f_buf[i] = n as f64;
        }

        fft::hamming_window(&mut f_buf);
        fft::real_fft(&f_buf, &mut res);

        // merge negative and positive component of frequency
        for i in 0..res.len() / 2 {
            res[i] = res[i].abs();
        }

        visualizer.draw_hist(&res[0..BUF_SIZE/2]);
        visualizer.handle_events();

        let sleep_nanos = 1_000_000_000i64 / FRAME_RATE as i64 - (time::precise_time_ns() - frame_start_ns) as i64;

        if sleep_nanos > 0 {
            sleep(Duration::from_nanos(sleep_nanos as u64));
        }
    }
}
