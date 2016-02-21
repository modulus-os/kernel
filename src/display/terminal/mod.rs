use display;
use core::fmt;

pub struct Terminal {
    writer: display::Writer,
	color: u8,
	cur_i: usize
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal{writer: display::Writer::new(0xb8000),
			color: display::Color::make_color(display::Color::White, display::Color::Black),
			cur_i: 0}
    }

	pub fn set_color(&mut self, color: u8) {
		self.color = color;
	}

	pub fn goto_line(&mut self, line: usize) {
		self.cur_i = line * display::VIDEO_WIDTH;
	}

    pub unsafe fn write_str_index(&self, s: &str, color: u8, index: usize) {
        let mut i = 0;
        for byte in s.bytes() {
            self.writer.write_index(display::Entry::new(byte, color), index + i);
            i += 1;
        }
    }

    pub unsafe fn write_str_pos(&self, s: &str, color:u8, x: usize, y: usize) {
        let index = y * display::VIDEO_WIDTH + x;
        let mut i = 0;
        for byte in s.bytes() {
            self.writer.write_index(display::Entry::new(byte, color), index + i);
            i += 1;
        }
    }
}

impl fmt::Write for Terminal {
	fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
		for byte in s.bytes() {
			unsafe {
				self.writer.write_index(display::Entry::new(byte, self.color), self.cur_i);
			}
			self.cur_i += 1;
		}
		Ok(())
	}
}

pub fn init() {
	let mut term = Terminal::new();
}
