use crate::io::display::*;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    terminal::TERM.lock().set_color(RED);
    print!("\n    !PANIC!    \n");
    terminal::TERM.lock().set_color(WHITE);
    loop {}
}*/
