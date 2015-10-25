//! Main module. Contains system entry point.

extern crate num;
extern crate libc;
extern crate sdl2;

mod pulse;
mod fft;
mod vis;

const BUF_SIZE: usize = 1024;

fn main() {
    let mut pa = pulse::PulseAudio::new("Spectralizer", "visualizer sink");

    let mut buf   = [0i16; BUF_SIZE];
    let mut f_buf = [0f64; BUF_SIZE];
    let mut res   = [0f64; BUF_SIZE];
    let mut res2  = [0f64; BUF_SIZE / 2];

    let mut visualizer = vis::Visualizer::new();

    loop {
        pa.sample(&mut buf[..]);

        for (i, &n) in buf.iter().enumerate() {
            f_buf[i] = n as f64;
        }

        fft::real_fft(&f_buf, &mut res);

        // merge negative and positive component of frequency
        for i in 0..res.len() {
            res[i] = res[i].abs();
            // XXX filter out odd samples, which are all zero; why are they zero?
            if i % 2 == 0 {
                res2[i/2] = res[i];
            }
        }

        visualizer.draw_hist(&res2[0..BUF_SIZE/4]);
    }
}
