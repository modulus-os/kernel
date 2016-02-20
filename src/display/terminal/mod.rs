use display;

pub struct Terminal {
    writer: display::Writer,
}

impl Terminal {
    pub fn new() -> Terminal {
        return Terminal{writer: display::Writer::new(0xb8000)};
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
