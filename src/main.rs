//! Main module. Contains system entry point.

extern crate num;
extern crate libc;
extern crate sdl2;

mod pulse;
mod fft;
mod vis;

use std::thread::sleep_ms;

const BUF_SIZE: usize = 1024;
const WIN_WIDTH: u32 = 256;
const WIN_HEIGHT: u32 = 192;
const SAMPLE_RATE: usize = 32768;

fn main() {
    let mut pa = pulse::PulseAudio::new("Spectralizer", "visualizer sink",
                                        SAMPLE_RATE);

    let mut buf   = [0i16; BUF_SIZE];
    let mut f_buf = [0f64; BUF_SIZE];
    let mut res   = [0f64; BUF_SIZE];

    let mut visualizer = vis::Visualizer::new("spectralizer", WIN_WIDTH,
                                              WIN_HEIGHT);

    let mut i: u64 = 0;

    loop {
        pa.sample(&mut buf[..]);
//      i += 1;
//      if i % ((SAMPLE_RATE / BUF_SIZE / 48) as u64) != 0 {
//          continue;
//      }

        for (i, &n) in buf.iter().enumerate() {
            f_buf[i] = n as f64;
        }

        fft::real_fft(&f_buf, &mut res);

        // merge negative and positive component of frequency
        for i in 0..res.len() {
            res[i] = res[i].abs();
            // XXX filter out odd freqs which are all zero; why are they zero?
            if i % 2 == 0 {
                res[i/2] = res[i];
            }
        }

        visualizer.draw_hist(&res[0..BUF_SIZE/4]);
        visualizer.handle_events();
    }
}
