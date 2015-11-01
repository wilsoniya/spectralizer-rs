//! Main module. Contains system entry point.

extern crate num;
extern crate libc;
extern crate sdl2;

mod pulse;
mod fft;
mod vis;

use std::thread::sleep_ms;

const BUF_SIZE: usize = 512;
const WIN_WIDTH: u32 = 256;
const WIN_HEIGHT: u32 = 192;
const SAMPLE_RATE: usize = 16384;

fn main() {
    let mut pa = pulse::PulseAudio::new("Spectralizer", "visualizer sink",
                                        SAMPLE_RATE);

    let mut buf   = [0i16; BUF_SIZE];
    let mut f_buf = [0f64; BUF_SIZE];
    let mut res   = [0f64; BUF_SIZE];

    let mut visualizer = vis::Visualizer::new("spectralizer", WIN_WIDTH,
                                              WIN_HEIGHT);

    loop {
        pa.sample(&mut buf[..]);
//      i += 1;
//      if i % ((SAMPLE_RATE / BUF_SIZE / 48) as u64) != 0 {
//          continue;
//      }

        for (i, &n) in buf.iter().enumerate() {
            f_buf[i] = n as f64;
        }

        fft::hamming_window(&mut f_buf);
        fft::real_fft(&f_buf, &mut res);

        // merge negative and positive component of frequency
        for i in 0..res.len() {
            res[i] = res[i].abs();
        }

        visualizer.draw_hist(&res[0..BUF_SIZE/2]);
        visualizer.handle_events();
    }
}
