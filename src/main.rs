extern crate sdl2;
pub mod drivers;
use crate::drivers::emulator::Emulator;
use std::fs;
use std::io;

fn main() {
    let mut emulator = Emulator::new();
    let files = fs::read_dir("./roms").unwrap();

    println!("Available files: ");
    for file in files {
        println!("{}", file.unwrap().path().display());
    }

    let mut input = String::new();
    let prepend = "./roms/";
    let stdin = io::stdin();

    stdin.read_line(&mut input).expect("Input error");

    let file_name = prepend.to_owned() + &input;
    println!("{}", file_name);
    emulator.run(file_name.trim());
}
