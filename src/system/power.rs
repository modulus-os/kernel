use arch::x86_64::*;

///Simple system reboot using 8042 keyboard controller
pub fn reboot() {
	let port = io::cpuio::Port::new(0x64);
	let mut good: u8 = 0x02;
	while good & 0x02 != 0 {
		good = port.inb();
	}
	port.outb(0xfe);
}