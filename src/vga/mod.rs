const VIDEO_WIDTH: usize = 80;
const VIDEO_HEIGHT: usize = 25;

#[repr(u8)]

pub enum Color
{
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

pub struct ColorCode(u8);

impl ColorCode
{
	pub fn new(fg: Color, bg: Color) -> ColorCode
	{
		return ColorCode((fg as u8) << 4 | (bg as u8));
	}
}

pub struct Buffer
{
	ch: u8
}

pub struct Writer
{
	ch: u8,
	color: u8,
	buffer: Buffer,
}

impl Writer
{
	pub fn set_char(&mut self, ch: u8)
	{
		self.ch = ch;
	}

	pub fn set_color(&mut self, color: u8)
	{
		self.color = color;
	}
}

//Initialize VGA
pub fn init()
{

}
