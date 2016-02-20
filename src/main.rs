#![feature(lang_items)]
#![no_std]

mod vga;

#[no_mangle]
pub extern fn kmain()
{
	vga::init();
}

#[lang = "eh_personality"]
extern fn eh_personality()
{
    vga::init();
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> !
{
	loop{}
}
