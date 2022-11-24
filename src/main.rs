extern crate sdl2;
pub mod chip8;
pub mod driver;
pub mod input_driver;
pub mod configs;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use std::{thread, time};
use crate::chip8::Chip8;
use crate::driver::Driver;
use crate::input_driver::InputDriver;


fn match_key(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}


fn main() {
    println!("Hello, world!");
    // Instantiate chip8
    let mut chip = Chip8::new();
    // Load rom into chip
    chip.load_rom().expect("Unable to load rom");

    // Instantiate driver
    let sdl_context = sdl2::init().unwrap();
    let mut driver = Driver::new(&sdl_context);
    let mut input = InputDriver::new(&sdl_context);

   
  'runner:  loop {
        for event in input.poll() {
            match event {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape), ..}=> {
                    break 'runner;
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = match_key(key) {
                        chip.set_key(k, true);
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = match_key(key) {
                        chip.set_key(k, false);
                    }
                },
                _ => ()
            }
        }

        if chip.state_change {
            driver.draw(&chip.vram);
            chip.state_change = false;
        }
        chip.cycle();
        thread::sleep(time::Duration::from_millis(1));
    }
}
