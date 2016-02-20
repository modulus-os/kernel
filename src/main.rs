#![feature(lang_items)]
#![no_std]

mod vga;
//mod rlibc;


#[no_mangle]
pub extern fn kmain()
{
	vga::init();
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
