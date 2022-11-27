use super::chip8::Chip8;
use std::collections::VecDeque;

pub struct Rewind {
    states: VecDeque<Chip8>,
    max_states: usize,
}

impl Rewind {
    pub fn new() -> Self {
        Self { 
            states: VecDeque::new(), 
            max_states: 5,
        }
    }

    pub fn capture(&mut self, chip: &Chip8) {
        let mut clonedchip = chip.clone();
        
        for i in 0..clonedchip.keypad.len() {
            clonedchip.keypad[i] = false;
        }

        if self.states.len() >= self.max_states {
            self.states.pop_front();
            self.states.push_back(clonedchip);
        } else {
            self.states.push_back(clonedchip);
        }
    }

    pub fn step_back(&mut self) -> Chip8 {
        if self.states.len() <= 1 {
            let cloned = self.states[0].clone();
            self.states.push_back(cloned);
        }
        return self.states.pop_back().unwrap();
    }
}