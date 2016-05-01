use core::fmt::Write;
use io::display::*;

// Use Rust core library
extern crate core;

pub fn panic(file: &str, line: u32) {
    terminal::TERM.lock().set_color(common_color::RED);
    print!("\n    !PANIC!    \n");
    terminal::TERM.lock().set_color(common_color::WHITE);
    print!("At {}:{}", file, line);
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(file: &str, line: u32) -> ! {
    panic(file, line);
    loop {}
}
