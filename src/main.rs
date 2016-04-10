use cpu::CPU;
use screen::Screen;
use keyboard::Keyboard;

mod cpu;
mod screen;
mod keyboard;

fn main() {

  let mut screen = Screen::new();
  let mut keyboard = Keyboard::new();
  let mut cpu = CPU::new(keyboard, screen);

  // TODO read args
  cpu.load("roms/BLITZ");

  loop {
    cpu.execute();
  }
}
