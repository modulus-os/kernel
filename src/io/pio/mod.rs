#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn outb(port: u16, value: u8) {
	unsafe{
		asm!("outb %al, %dx"
			:
			: "{dx}"(port), "{al}"(value)
			:
			: "volatile");
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn outw(port: u16, value: u16) {
	unsafe{
		asm!("outw %ax, %dx"
			:
			: "{dx}"(port), "{ax}"(value)
			:
			: "volatile");
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn outl(port: u16, value: u32) {
	unsafe{
		asm!("outl %eax, %dx"
			:
			: "{dx}"(port), "{eax}"(value)
			:
			: "volatile");
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn inb(port: u16) -> u8{
	unsafe{
		let res: u8;
		asm!("inb %dx, %al"
			: "={al}"(res)
			: "{dx}"(port)
			:
			: "volatile");
		res
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn inw(port: u16) -> u16{
	unsafe{
		let res: u16;
		asm!("inw %dx, %ax"
			: "={ax}"(res)
			: "{dx}"(port)
			:
			: "volatile");
		res
	}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn inl(port: u16) -> u32{
	unsafe{
		let res: u32;
		asm!("inl %dx, %eax"
			: "={eax}"(res)
			: "{dx}"(port)
			:
			: "volatile");
		res
	}
}
