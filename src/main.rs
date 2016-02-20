#![feature(lang_items)]
#![no_std]

mod display;
mod rlibc;

#[no_mangle]
pub extern fn kmain() {
    //let writer = display::Writer::new(0xb8000);
    let color: u8 = display::Color::make_color(display::Color::Green, display::Color::Black);
    let term = display::terminal::Terminal::new();

    unsafe {
        term.write_str_index("                _      _          ", color, display::VIDEO_WIDTH + 22);
        term.write_str_index("  _ __  ___  __| |_  _| |___ _ _  ", color, display::VIDEO_WIDTH * 2 + 22);
        term.write_str_index(" | '  \\/ _ \\/ _` | || | / _ \\ ' \\ ", color, display::VIDEO_WIDTH * 3 + 22);
        term.write_str_index(" |_|_|_\\___/\\__,_|\\_,_|_\\___/_||_|", color, display::VIDEO_WIDTH * 4 + 22);
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
	loop{}
}
