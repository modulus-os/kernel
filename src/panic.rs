use crate::io::display::*;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    terminal::TERM.lock().set_color(RED);
    print!("\n    !PANIC!    \n");
    terminal::TERM.lock().set_color(WHITE);
    if let Some(location) = info.location() {
        print!("  at {}:{}\n", location.file(), location.line());
    }
    if let Some(message) = info.message().as_str() {
        print!("  {}\n", message);
    } else {
        print!("  {}\n", info.message());
    }
    loop {}
}
