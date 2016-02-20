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

impl Color
{
    pub fn make_color(fg: Color, bg: Color) -> u8
    {
        return (bg as u8) << 4 | (fg as u8);
    }
}

pub struct Entry(u16);

impl Entry
{
    pub fn new(c: u8, color: u8) -> Entry
    {
        return Entry((color as u16) << 8 | (c as u16));
    }
}

pub struct Writer
{
    ptr: usize,
}

impl Writer
{
    pub fn new(ptr: usize) -> Writer
    {;
        return Writer{ptr: ptr};
    }

    pub fn set_ptr(&mut self, ptr: usize)
    {
        self.ptr = ptr;
    }

    fn make_entry(&mut self, color: u8, c: u8) -> u16
    {
       return (color as u16) << 8 | c as u16;
    }

    pub unsafe fn write_index(&self, entry: Entry, index: usize)
    {
        //Each entry is 2 bytes
        let buf = (self.ptr + (0xf * index)) as *mut u16;
        *buf = entry.0 as u16;
    }

    pub unsafe fn write_pos(&self, entry: Entry, x: usize, y: usize)
    {

    }
}

//Initialize VGA
pub fn init()
{

}
