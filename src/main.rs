extern crate libc;
extern crate time;

mod pulse;

fn main() {
    let mut pa = pulse::PulseAudio::new("Spectralizer", "visualizer sink");

//  print!("type something: ");
//  let mut _stdin = stdin();
//  let mut read_line = String::new();
//  _stdin.read_line(&mut read_line);
    pa.read_loop();
}
