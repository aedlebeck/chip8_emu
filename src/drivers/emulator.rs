extern crate sdl2;
use crate::drivers::chip8::Chip8;
use crate::drivers::configs::defaults::*;
use crate::drivers::input_driver::InputDriver;
use crate::drivers::video_driver::VideoDriver;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use std::{thread, time};
use super::rewind::Rewind;

pub struct Emulator {
    chip8: Chip8,
    video_driver: VideoDriver,
    input_driver: InputDriver,
    paused: bool,
    rewind: Rewind,
}

impl Emulator {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        Self {
            chip8: Chip8::new(),
            video_driver: VideoDriver::new(&sdl_context),
            input_driver: InputDriver::new(&sdl_context),
            paused: false,
            rewind: Rewind::new(),
        }
    }

    pub fn run(&mut self, rom:String) {
        self.chip8.load_rom(rom).expect("Unable to load rom");
        // let mut clock_hertz = Instant::now();
        let mut timer = Instant::now();
        let mut ticks_per_frame = 9;
        let mut counter = 0;

        println!("Ticks per frame: {ticks_per_frame}");
        println!("Clock Delay: {}", BUFFER_DELAY);
        let mut clock_counter = 0;
        let mut timer_counter = 0;
        

        let mut rewind_ctr = 0;
        self.rewind.capture(&self.chip8);

        let mut frame_buffer = Instant::now();
        'runner: loop {
            // Get input
            for event in self.input_driver.poll() {
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
                            if !self.paused {
                                self.chip8.set_key(k, true);
                            }
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
                        // Pause game
                        if key == Keycode::Space {
                            if self.paused {
                                self.paused = false;
                            } else {
                                self.paused = true;
                            }
                        }
                        // Rewind
                        if key == Keycode::Left {
                            self.chip8 = self.rewind.step_back();
                            rewind_ctr = 0;
                        }
                    }

                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        if let Some(k) = match_key(key) {
                            if !self.paused {
                                self.chip8.set_key(k, false);
                            }
                        }
                    }
                    _ => (),
                }
            }
            if !self.paused {
                if rewind_ctr > 250 {
                    self.rewind.capture(&self.chip8);
                    rewind_ctr = 0;
                }
                rewind_ctr += 1;

                if timer.elapsed() >= time::Duration::from_secs(1) {
                    println!(
                        "Clock Hz: {}, Timer Hz: {}",
                        clock_counter as f32 / 1.0,
                        timer_counter as f32 / 1.0
                    );
                    clock_counter = 0;
                    timer_counter = 0;
                    timer = Instant::now();
                }
    
                self.chip8.cycle();
                counter += 1;
    
                if counter >= ticks_per_frame {
                    self.chip8.timer_tick();
                    if self.chip8.vram_change {
                        self.video_driver.draw(&self.chip8.vram);
                        self.chip8.vram_change = false;
                    }
                    timer_counter += 1;
                    counter = 0;
                    thread::sleep(
                        time::Duration::from_millis(BUFFER_DELAY)
                            .saturating_sub(frame_buffer.elapsed()),
                    );
                    frame_buffer = Instant::now();
                }
    
                clock_counter += 1;
            }
            }
    }
}

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
