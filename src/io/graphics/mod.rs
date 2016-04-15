pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;
pub const DEPTH: usize = 4;

pub struct Pixel(u32);

impl Pixel {
	pub fn new(r: u32, g: u32, b: u32) -> Self{
		Pixel((r << 16 | g << 8 | b) as u32)
	}
}

pub struct Writer {
	ptr: usize,
}

impl Writer {
	pub fn new(ptr: usize) -> Writer {
		Writer { ptr: ptr }
	}

	pub fn get_buffer(&self) -> *mut u32 {
		self.ptr as *mut u32
	}

	pub fn at(&mut self, pixel: Pixel, x: usize, y: usize) {
		let index = (x * DEPTH) + (y * DEPTH * WIDTH);
		unsafe {
			*self.get_buffer().offset(index as isize) = pixel.0;
		}
	}
}
