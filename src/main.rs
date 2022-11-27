extern crate sdl2;
pub mod drivers;
use crate::drivers::configs::defaults::*;
use crate::drivers::input_driver::InputDriver;
use crate::drivers::video_driver::VideoDriver;
use crate::drivers::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use std::{thread, time};

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
    let mut driver = VideoDriver::new(&sdl_context);
    let mut input = InputDriver::new(&sdl_context);

    // let mut clock_hertz = Instant::now();
    let mut timer = Instant::now();
    let mut ticks_per_frame = 9;
    let mut counter = 0;
    
    println!("Ticks per frame: {ticks_per_frame}");
    println!("Clock Delay: {}", BUFFER_DELAY);
    let mut clock_counter = 0;
    let mut timer_counter = 0;
    
    let mut frame_buffer = Instant::now();
    'runner: loop {
        for event in input.poll() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'runner;
                }

                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    // Interpret keycode for chip8
                    if let Some(k) = match_key(key) {
                        chip.set_key(k, true);
                    }
                    // Increment ticks per frame
                    if key == Keycode::Up {
                        ticks_per_frame += 1;
                    }

                    // Decrement ticks per frame
                    if key == Keycode::Down {
                        if ticks_per_frame > 1 {
                            ticks_per_frame -= 1;
                        }
                    }

                }

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = match_key(key) {
                        chip.set_key(k, false);
                    }
                }
                _ => (),
            }
        }

        if timer.elapsed() >= time::Duration::from_secs(1) {
            println!("Clock Hz: {}, Timer Hz: {}", clock_counter as f32 / 1.0, timer_counter as f32 / 1.0);
            clock_counter = 0;
            timer_counter = 0;
            timer = Instant::now();
        }

        chip.cycle();
        counter += 1;

        if counter >= ticks_per_frame {
            chip.timer_tick();
            if chip.vram_change {
                driver.draw(&chip.vram);
                chip.vram_change = false;
            }
            timer_counter += 1;
            counter = 0;
            thread::sleep(time::Duration::from_millis(BUFFER_DELAY).saturating_sub(frame_buffer.elapsed()));
            frame_buffer = Instant::now();
        }

        clock_counter += 1;
    }
}
