use rand::Rng;
use std::fs::File;
use std::io;
use std::io::Read;
use crate::configs::defaults::*;

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: usize,
    pc: usize,
    stack: [u16; 16],
    sp: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    keypad: [bool; 16],
    pub vram: [[u8; 64]; 32],
    pub vram_change: bool,
    opcode: u16,
    waiting: bool,
    wait_key: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            vram: [[0; 64]; 32],
            vram_change: false,
            opcode: 0,
            waiting: false,
            wait_key: 0,
        }
    }

    pub fn cycle(&mut self) {

        // Fetch opcode
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[self.pc as usize + 1]) as u16;

        // Increment pc
        self.pc += 2;

        // Match opcode and execute
        self.execute();

        // Decrement sound timer and delay timer
        self.timer_tick();
    }

    // Match opcode and execute
    fn execute(&mut self) {
        let nibbles = (
            (self.opcode & 0xF000) >> 12 as u8,
            (self.opcode & 0x0F00) >> 8 as u8,
            (self.opcode & 0x00F0) >> 4 as u8,
            (self.opcode & 0x000F) as u8,
        );
        // println!("OPCODE: {:?}", self.opcode);
        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(),
            (0x02, _, _, _) => self.op_2nnn(),
            (0x03, _, _, _) => self.op_3xkk(),
            (0x04, _, _, _) => self.op_4xkk(),
            (0x05, _, _, 0x00) => self.op_5xy0(),
            (0x06, _, _, _) => self.op_6xkk(),
            (0x07, _, _, _) => self.op_7xkk(),
            (0x08, _, _, 0x00) => self.op_8xy0(),
            (0x08, _, _, 0x01) => self.op_8xy1(),
            (0x08, _, _, 0x02) => self.op_8xy2(),
            (0x08, _, _, 0x03) => self.op_8xy3(),
            (0x08, _, _, 0x04) => self.op_8xy4(),
            (0x08, _, _, 0x05) => self.op_8xy5(),
            (0x08, _, _, 0x06) => self.op_8xy6(),
            (0x08, _, _, 0x07) => self.op_8xy7(),
            (0x08, _, _, 0x0e) => self.op_8xye(),
            (0x09, _, _, 0x00) => self.op_9xy0(),
            (0x0a, _, _, _) => self.op_annn(),
            (0x0b, _, _, _) => self.op_bnnn(),
            (0x0c, _, _, _) => self.op_ckkk(),
            (0x0d, _, _, _) => self.op_dxyn(),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(),
            // _ => self.cycle(),
            other => {
                print!("No matching opcode: {:?}", other);
                self.cycle();
            }
        }
    }

    pub fn load_rom(&mut self) -> io::Result<()> {
        let mut f = File::open("./roms/sweetcopter.ch8")?;
        let mut buf: Vec<u8> = Vec::with_capacity(4096);
        f.read_to_end(&mut buf)?;

        let mut counter: usize = 0;
        for i in buf {
            self.memory[counter + START_ADDRESS as usize] = i;
            counter += 1;
        }

        counter = 0;
        loop {
            if counter >= 80 {
                break;
            }
            self.memory[counter + 0x50] = FONT_SIZES[counter];
            counter += 1;
        }

        println!("{:?}", self.memory);
        Ok(())
    }

    pub fn set_key(&mut self, key: usize, state: bool) {
        self.keypad[key] = state;
    }

    pub fn gen_rand(&self) -> u8 {
        let mut rng = rand::thread_rng();
        let x: u8 = rng.gen();
        return x;
    }

    pub fn timer_tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            // TODO SOUND
            if self.sound_timer == 1 {
                // BEEP
            }
            self.sound_timer -= 1;
        }
    }

    // 00E0 - CLS, clears the display
    fn op_00e0(&mut self) {
        self.vram = [[0; 64]; 32];
        self.vram_change = true;
    }

    // 00EE - RET, return from subroutine
    // Gets address from stack
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
    }

    // 1nnn - JMP addr
    // Jump to location in memory nnn
    fn op_1nnn(&mut self) {
        // What does this do?
        let address: u16 = self.opcode & 0x0FFF;
        self.pc = address as usize;
    }

    // 2nnn - CALL addr
    // Call subroutine at nnn
    fn op_2nnn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = address as usize;
    }

    // 3xkk - SE vx, Byte
    // Skip next instruction if vx == kk
    fn op_3xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] == byte {
            self.pc += 2;
        }
    }

    // 4xkk - SNE vx, Byte
    // Skip next instruction if vx != kk
    fn op_4xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        if self.registers[vx as usize] != byte {
            self.pc += 2;
        }
    }

    // 5xy0 - SE vx, vy
    // Skip next instruction if vx == vy
    fn op_5xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    // 6xkk - LD vx, Byte
    // Load kk into vx
    fn op_6xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = byte;
    }

    // 7xkk - ADD vx, Byte
    // Add kk to vx
    fn op_7xkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        let result: u16 = self.registers[vx as usize] as u16 + byte as u16;

        self.registers[vx as usize] = result as u8;
    }

    // 8xy0 - LD vx, vy
    // Load vy into vx
    fn op_8xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    // 8xy1 - OR vx, vy
    // Sets vx = vx OR vy
    fn op_8xy1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        self.registers[vx as usize] |= self.registers[vy as usize];
        self.registers[0xF] = 0;
    }

    // 8xy2 - AND vx, vy
    // Sets vx = vx AND vy
    fn op_8xy2(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        self.registers[vx as usize] &= self.registers[vy as usize];
        self.registers[0xF] = 0;
    }

    // 8xy3 - XOR vx, vy
    // Sets vx = vx XOR vy
    fn op_8xy3(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        self.registers[vx as usize] ^= self.registers[vy as usize];
        self.registers[0xF] = 0;
    }

    // 8xy4 - ADD vx, vy
    // Sets vx = vx + vy
    fn op_8xy4(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;
        let x: u16 = self.registers[vx as usize] as u16;
        let y: u16 = self.registers[vy as usize] as u16;
        let sum = x + y;

        if sum > 0xFF {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] = sum as u8;
    }

    // 8xy5 - SUB vx, vy
    // Sets vx = vx - vy
    fn op_8xy5(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;
        let x: u8 = self.registers[vx as usize] as u8;
        let y: u8 = self.registers[vy as usize] as u8;
        let result = x.wrapping_sub(y);

        if self.registers[vx as usize] > self.registers[vy as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] = result as u8;
    }

    // 8xy6
    // Store the value of register VY shifted right one bit in register VX
    // Set register VF to the least significant bit prior to the shift
    // Bitshift right, save LSB in VF
    fn op_8xy6(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        self.registers[vx as usize] = self.registers[vy as usize];
        let lsb = self.registers[vx as usize] & 1;

        self.registers[vx as usize] = self.registers[vx as usize].overflowing_shr(1).0;
        self.registers[0xF] = lsb;
    }

    // 8xy7 - SUBN vx, vy
    // Sets vx = vy - vx
    fn op_8xy7(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;
        let x: u16 = self.registers[vx as usize] as u16;
        let y: u16 = self.registers[vy as usize] as u16;

        let result = y.wrapping_sub(x);

        if self.registers[vy as usize] > self.registers[vx as usize] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx as usize] = result as u8;
    }

    // 8xyE - SHL vx {, vy}
    // Bitshift left, save MSB in VF
    fn op_8xye(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        self.registers[vx as usize] = self.registers[vy as usize] << 1;
        self.registers[0xF] = ((self.registers[vy as usize] & 0x80) >> 7) as u8;
    }

    // 9xy0 - SNE vx, vy
    // Skip next instruction if vx != vy
    fn op_9xy0(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4u8) as u8;

        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }
    }

    // Annn - LD I, addr
    // Set I = nnn
    fn op_annn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.index = address as usize;
    }

    // Bnnn - JP V0, addr
    // Jump to nnn + V0
    fn op_bnnn(&mut self) {
        let address: u16 = self.opcode & 0x0FFF;
        self.pc = (self.registers[0] as u16 + address) as usize;
    }

    // Cxkk - RND vx, Byte
    // Set vx = random byte AND kk
    fn op_ckkk(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let byte: u8 = (self.opcode & 0x00FF) as u8;

        self.registers[vx as usize] = self.gen_rand() & byte;
    }

    // Dxyn - DRW vx, vy, nibble
    // Display n-byte sprite, starting at vx, vy
    // Set VF = collision
    fn op_dxyn(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
        let n: u8 = (self.opcode & 0x000F) as u8;

        self.registers[0xF] = 0;
   
        for byte in 0..n {
            let y = (self.registers[vy as usize] + byte) % VIDEO_HEIGHT;
            for bit in 0..8 {
                let x = (self.registers[vx as usize] + bit) % VIDEO_WIDTH; //FIXME ADD WITH OVERFLOW
                let color = (self.memory[self.index + byte as usize] >> (7 - bit)) & 1;
                self.registers[0xF] |= color & self.vram[y as usize][x as usize];
                self.vram[y as usize][x as usize] ^= color;
            }
        }
        self.vram_change = true;
    }

    // Ex9E - SKP vx
    // Skip next instruction if key of value vx is pressed
    fn op_ex9e(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] == true {
            self.pc += 2;
        }
    }

    // ExA1 - SKNP vx
    // Skip next instruction if key of value vx is not pressed
    fn op_exa1(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let key: u8 = self.registers[vx as usize];

        if self.keypad[key as usize] != true {
            self.pc += 2;
        }
    }

    // Fx07 - LD vx, DT
    // Set vx = delay timer value
    fn op_fx07(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        self.registers[vx as usize] = self.delay_timer;
    }

    // Fx0A - LD vx, k
    // Wait for keypress, store value of key in vx
    fn op_fx0a(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;

        if self.waiting {
            println!("WAITING for keyup");
            if self.keypad[self.wait_key as usize] == false {
                self.waiting = false;
                self.registers[vx as usize] = self.wait_key as u8;
                println!("NO LONGER WAITING");
                return;
            }
            self.pc -= 2;
        } else {
            println!("WAITING for keydown");
            for i in 0..self.keypad.len() {
                if self.keypad[i] {
                    // self.registers[vx as usize] = i as u8;
                    self.wait_key = i as u8;
                    self.waiting = true;
                }
            }
            // if !pressed {
            //     self.pc -= 2;
            // }
            self.pc -= 2;
        }
    }

    // Fx15 - LD DT, vx
    // Set delay timer = vx
    fn op_fx15(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        self.delay_timer = self.registers[vx as usize];
    }

    // Fx18 - LD ST, vx
    // Set sound timer = vx
    fn op_fx18(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        self.sound_timer = self.registers[vx as usize];
    }

    // Fx1E - ADD I, vx
    // Set index = index + vx
    fn op_fx1e(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        self.index += self.registers[vx as usize] as usize;
    }

    // Fx29 - LD F, vx
    // Set index = location of sprite for vx
    fn op_fx29(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let digit: u8 = self.registers[vx as usize];

        self.index = (FONT_START_ADDRESS + (5 * digit) as u32) as usize;
    }

    // Fx33 - LD B, vx
    // BCD OF vx
    fn op_fx33(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        let mut value: u8 = self.registers[vx as usize];

        self.memory[(self.index + 2) as usize] = value % 10;
        value /= 10;

        self.memory[(self.index + 1) as usize] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

    // Fx55 - LD [I], vx
    // Store registers V0 .. vx in memory at location I
    fn op_fx55(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;
        for i in 0..=vx {
            self.memory[(self.index + i as usize)] = self.registers[i as usize];
        }
        self.index = self.index + 1 + vx as usize;
    }

    // Fx65 - LD vx, [I]
    // Load registers V0 .. vx from memory at location I
    fn op_fx65(&mut self) {
        let vx: u8 = ((self.opcode & 0x0F00) >> 8u8) as u8;

        for i in 0..=vx {
            self.registers[i as usize] = self.memory[(self.index + i as usize) as usize];
        }
        self.index = self.index + 1 + vx as usize;
    }
}
