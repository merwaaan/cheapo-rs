pub struct Screen {
  pixels: [[bool; 64]; 32]
}

impl Screen {

  pub fn new() -> Screen {
    Screen {
      pixels: [[false; 64]; 32]
    }
  }

  pub fn clear(&self) {

  }

  pub fn draw(&self, rows: &[u8], x: u8, y: u8) -> bool {
    false
  }
}
