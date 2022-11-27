extern crate sdl2;
pub mod drivers;
use crate::drivers::emulator::Emulator;

fn main() {
    println!("Hello, world!");
    let mut emulator = Emulator::new();
    emulator.run(String::from("./roms/flightrunner.ch8"));
}
