extern crate rand;

use keyboard::Keyboard;
use screen::Screen;

pub struct CPU {
  regs: [u8; 16],
  memory: [u8; 0x1000],
  pc: u16, // TODO or usize?
  stack: [u16; 16],
  sp: usize,
  i: usize,
  delay_timer: u8,
  sound_timer: u8,

  keyboard: Keyboard,
  screen: Screen
}

impl CPU {

    pub fn new(keyboard: Keyboard, screen: Screen) -> CPU {
      CPU {
        regs: [0; 16],
        memory: [0; 0x1000],
        pc: 0x200,
        stack: [0; 16],
        sp: 0,
        i: 0,
        delay_timer: 0,
        sound_timer: 0,

        keyboard: keyboard,
        screen: screen
      }
    }

    pub fn load(&mut self, filename: &str) {

      use std::fs::File;
      use std::io::Read;

      let mut file = match File::open(filename) {
        Err(reason) => panic!("Could not open {}: {}", filename, reason),
        Ok(file) => file
      };

      file.read(&mut self.memory[0x200..0x400]);

      // Copy font data
      let fonts = [
        0xF0, 0x90, 0x90, 0x90, 0xF0,
        0x20, 0x60, 0x20, 0x20, 0x70,
        0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0,
        0x90, 0x90, 0xF0, 0x10, 0x10,
        0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0,
        0xF0, 0x10, 0x20, 0x40, 0x40,
        0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0,
        0xF0, 0x90, 0xF0, 0x90, 0x90,
        0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0,
        0xE0, 0x90, 0x90, 0x90, 0xE0,
        0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80
      ];
      for i in 0..fonts.len() {
        self.memory[i] = fonts[i];
      }
    }

    pub fn execute(&mut self) {

      let hi = self.memory[self.pc as usize];
      let lo = self.memory[self.pc as usize + 1];
      let opcode = (hi as u16) << 8 | lo as u16;

      let (a, b, c, d) = (
        (opcode & 0xF000) / 0xFFF,
        (opcode & 0xF00) / 0xFF,
        (opcode & 0xF0) / 0xF,
         opcode & 0xF);

      // TODO how to have the correct type sooner?
      let a: usize = a as usize;
      let b: usize = b as usize;
      let c: usize = c as usize;
      let d: usize = d as usize;

      println!("{:x} @ {:x}", opcode, self.pc);

      match (a, b, c, d) {

        // Clear the screen
        (0, 0, 0xE, 0) => { self.screen.clear(); },

        // Return from a subroutine
        (0, 0, 0xE, 0xE) => {
          self.sp -= 1;
          self.pc = self.stack[self.sp];
        },

        // Jump to NNN
        (1, _, _, _) => { self.pc = opcode & 0xFFF - 2; },

        // Call NNN
        (2, _, _, _) => {
          self.stack[self.sp] = self.pc;
          self.sp += 1;
          self.pc = (hi as u16 & 0xF) << 8 | lo as u16;
          self.pc -= 2; // Offset PC to compensate for the incoming increment
        },

        // Skip if regB == CD
        (3, _, _, _) => {
          if self.regs[b] == opcode as u8 & 0xFF {
            self.pc += 2
          }
        },

        // Skip if regB != CD
        (4, _, _, _) => {
          if self.regs[b] != opcode as u8 & 0xFF {
            self.pc += 2
          }
        },

        // Skip if regB == regC
        (5, _, _, 0) => {
          if self.regs[b] == self.regs[c] {
            self.pc += 2
          }
        },

        // Set regB to CD
        (6, _, _, _) => { self.regs[b] = lo; },

        // Add CD to regB
        (7, _, _, _) => { self.regs[b].wrapping_add(lo); },

        // Set regB to the value of regC
        (8, _, _, 0) => { self.regs[b] = self.regs[c]; },

        // Set regB to regB | regC
        (8, _, _, 1) => { self.regs[b] |= self.regs[c]; },

        // Set regB to regB & regC
        (8, _, _, 2) => { self.regs[b] &= self.regs[c]; },

        // Set regB to regB ^ regC
        (8, _, _, 3) => { self.regs[b] ^= self.regs[c]; },

        // Skip if regB != regC
        (9, _, _, 0) => {
          if self.regs[b] != self.regs[c] {
            self.pc += 2
          }
        },

        // Set I to BCD
        (0xA, _, _, _) => { self.i = (opcode & 0xFFF) as usize; },

        // Jump to address BCD + reg0
        (0xB, _, _, _) => { self.pc = opcode & 0xFFF + self.regs[0] as u16; },

        // Set regB to the value of regB & a random value
        (0xC, _, _, _) => { self.regs[b] &= rand::random::<u8>(); },

        // Draw D rows of the current sprite at position (B,C)
        (0xD, _, _, _) => {
          let hit = self.screen.draw(&self.memory[self.i..self.i+d], b as u8, c as u8);
          if hit {
            self.regs[0xF] = 1;
          }
        },

        // Set delay timer to regB
        (0xF, _, 1, 5) => { self.delay_timer = self.regs[b]; },

        // Set delay timer to regB
        (0xF, _, 1, 8) => { self.sound_timer = self.regs[b]; },

        // Await for a keypress and then store the key in regB

        // Add regB to I
        (0xF, _, 1, 0xE) => { self.i += self.regs[b] as usize },

        _ => { panic!("Unimplemented opcode: {}", opcode) }
      }

      self.pc += 2;
    }
}
