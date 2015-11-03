//! Main module. Contains system entry point.

extern crate libc;
extern crate num;
extern crate sdl2;
extern crate time;

mod pulse;
mod fft;
mod vis;

const BUF_SIZE: usize = 256;
const WIN_WIDTH: u32 = 1024;
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

    let mut last_frame_ns: u64 = 0;

    loop {
        pa.sample(&mut buf[..]);
        if time::precise_time_ns() - last_frame_ns < 1000000000 / FRAME_RATE {
            visualizer.handle_events();
            continue;
        }

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
        last_frame_ns = time::precise_time_ns();
    }
}
