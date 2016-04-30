#![macro_use]

use io::display;
use core::fmt;
use spin::Mutex;

pub struct Terminal {
    writer: display::Writer,
    color: u8,
    x: usize,
    y: usize,
}

pub static TERM: Mutex<Terminal> = Mutex::new(Terminal {
    writer: display::Writer { ptr: 0xb8000 },
    color: display::Color::new(display::Color::White, display::Color::Black),
    x: 0,
    y: 0,
});

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            writer: display::Writer::new(0xb8000),
            color: display::Color::new(display::Color::White, display::Color::Black),
            x: 0,
            y: 0,
        }
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
        if self.y > display::VIDEO_HEIGHT {
            self.scroll();
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(display::VIDEO_WIDTH * display::VIDEO_HEIGHT) {
            self.writer.write_index(display::Entry::new(b' ', self.color), i);
        }
    }

    pub fn scroll(&mut self) {
        for i in 0..(display::VIDEO_WIDTH * display::VIDEO_HEIGHT) {
            self.writer.write_index(self.writer.at(i + display::VIDEO_WIDTH), i);
        }
    }

    pub fn backspace(&mut self) {
        self.writer.write_index(display::Entry::new(b' ', self.color),
                                self.y * display::VIDEO_WIDTH + self.x - 1);
        self.x -= 1;
    }
}

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                b'\n' => self.newline(),
                b'\x08' => self.backspace(),
                byte => {
                    self.writer.write_index(display::Entry::new(byte, self.color),
                                            self.y * display::VIDEO_WIDTH + self.x);
                    self.advance();
                }
            }
        }
        Ok(())
    }
}
