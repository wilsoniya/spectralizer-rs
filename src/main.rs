extern crate num;
extern crate libc;
extern crate time;
extern crate sdl2;

mod pulse;
mod fft;
mod vis;

use std::fmt::Display;

const BUF_SIZE: usize = 1024;

fn main() {
    let mut pa = pulse::PulseAudio::new("Spectralizer", "visualizer sink");

    let mut buf   = [0i16; BUF_SIZE];
    let mut f_buf = [0f64; BUF_SIZE];
    let mut res   = [0f64; BUF_SIZE];

    let mut visualizer = vis::Visualizer::new();

    loop {
        pa.sample(&mut buf[..]);

        for (i, &n) in buf.iter().enumerate() {
            f_buf[i] = n as f64;
        }

        fft::real_fft(&f_buf, &mut res);

//      print_mono(&buf);
//      print_mono(&res);
//      print_pair(&buf, &res);
        visualizer.draw_hist(&res);
//      visualizer.draw_hist(&f_buf);
    }
}

fn print_stereo<T: Display>(buf: &[T]) {
    for i in 0..(buf.len() / 2) {
        println!("{}: {}, {}", i, buf[i * 2], buf[i * 2 + 1]);
    }
}

fn print_mono<T: Display>(buf: &[T]) {
    for (i, n) in buf.iter().enumerate() {
        println!("{}: {}", i, n);
    }
}

fn print_pair<T: Display, U: Display>(buf1: &[T], buf2: &[U]) {
    assert!(buf1.len() == buf2.len());
    for (i, (a, b)) in buf1.iter().zip(buf2).enumerate() {
        println!("{}: ({}, {})", i, a, b);
    }
}
