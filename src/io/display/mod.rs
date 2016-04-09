pub mod terminal;
pub mod common_color;

pub const VIDEO_WIDTH: usize = 80;
pub const VIDEO_HEIGHT: usize = 25;

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

impl Color {
    pub const fn new(fg: Color, bg: Color) -> u8 {
        (bg as u8) << 4 | (fg as u8)
    }
}

pub struct Entry(u16);

impl Entry {
    pub fn new(c: u8, color: u8) -> Entry {
        Entry((color as u16) << 8 | (c as u16))
    }
	
	pub fn from_u16(data: u16) -> Entry {
		Entry(data)
	}
}

pub struct Writer {
    ptr: usize,
}

impl Writer {
	pub fn new(ptr: usize) -> Writer {
		Writer{ptr: ptr}
	}

	unsafe fn get_buffer(&self) -> *mut u16 {
		self.ptr as *mut u16
	}
	
	pub fn at(&self, index: usize) -> Entry {
		Entry::from_u16( unsafe { *self.get_buffer().offset(index as isize) } )
	}

	pub fn write_index(&self, entry: Entry, index: usize) {
		unsafe{ *self.get_buffer().offset(index as isize) = entry.0 as u16; }
	}

	pub fn write_pos(&self, entry: Entry, x: usize, y: usize) {
		let index = y * VIDEO_WIDTH + x;
		unsafe{ *self.get_buffer().offset(index as isize)  = entry.0 as u16; }
	}
}
