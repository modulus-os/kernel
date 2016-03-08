//!-----------------------------------------------------------------------------------------------
//!`src/display/terminal.rs`
//!
//!Implements more advanced terminal features, built on top of `src/display/mod.rs`.
//!-----------------------------------------------------------------------------------------------

use io::display;
use core::fmt;

pub struct Terminal {
    writer: display::Writer,
	color: u8,
	x: usize,
	y: usize,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal{writer: display::Writer::new(0xb8000),
			color: display::Color::new(display::Color::White, display::Color::Black),
			x: 0, y: 0}
    }

	pub fn set_color(&mut self, color: u8) {
		self.color = color;
	}

	pub fn advance(&mut self) {
		if self.x <= display::VIDEO_WIDTH {
			self.x += 1;
		} else {
			self.newline();
		}
	}

	pub fn newline(&mut self) {
		self.x = 0;
		self.y += 1;
	}
}

impl fmt::Write for Terminal {
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
		for byte in s.bytes() {
			self.writer.write_index(display::Entry::new(byte, self.color),
				self.y * display::VIDEO_WIDTH + self.x);
			self.advance();
		}
		Ok(())
	}
}
