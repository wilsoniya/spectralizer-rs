#![feature(libc)]

mod pulse; 

extern crate libc;

use std::io::stdin;

fn main() {
    let mut pa = pulse::pa_simple::new("pa_name", "stream_name");
//  print!("type something: ");
//  let mut _stdin = stdin();
//  let mut read_line = String::new();
//  _stdin.read_line(&mut read_line);
    println!("FART");
    pa.read_loop(); 
}
