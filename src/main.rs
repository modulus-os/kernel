#![feature(lang_items)]
#![no_std]

mod vga;

#[no_mangle]
pub extern fn kmain()
{
    let writer = vga::Writer::new(0xb8000);
    unsafe{writer.write_index(vga::Entry::new('c' as u8, vga::Color::make_color(vga::Color::White, vga::Color::Black)), 0)};
}

#[lang = "eh_personality"]
extern fn eh_personality()
{
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> !
{
	loop{}
}
