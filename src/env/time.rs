use io::pio::*;
use spin::Mutex;

static CLOCK: Mutex<usize> = Mutex::new(0);

/// Initializes the PIT and sets the speed to HZ
pub fn init() {
    let divisor = 1193182 / 1000;
    outb(0x43, 0x34);
    outb(0x40, (divisor & 0xff) as u8);
    outb(0x40, (divisor >> 8) as u8);
}

pub fn ticks() -> usize {
    *CLOCK.lock()
}

pub fn seconds() -> usize {
    *CLOCK.lock() / 1000
}

pub fn increment() {
    *CLOCK.lock() += 1;
}
