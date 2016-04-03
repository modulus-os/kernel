// !-----------------------------------------------------------------------------------------------
// !`src/io/cpuio/mod.rs`
// !
// !Declares common cpuio functions such as inb, outb, etc.
// !-----------------------------------------------------------------------------------------------

pub struct Port {
	port: u16,
}

impl Port {
	pub fn new(port: u16) -> Port{
		Port {port: port}
	}

	pub fn outb(&self, value: u8) {
		unsafe{
			asm!("outb %al, %dx" ::
        		"{dx}"(self.port), "{al}"(value) ::
        		"volatile");
		}
	}

	pub fn outw(&self, value: u8) {
		unsafe{
			asm!("outw %ax, %dx" ::
        		"{dx}"(self.port), "{ax}"(value) ::
        		"volatile");
		}
	}

	pub fn outl(&self, value: u8) {
		unsafe{
			asm!("outl %eax, %dx" ::
        		"{dx}"(self.port), "{eax}"(value) ::
        		"volatile");
		}
	}

	pub fn inb(&self) -> u8{
		unsafe{
			let res: u8;
			asm!("inb %dx, %al" :
				"={al}"(res) : "{dx}"(self.port) ::
				"volatile");
			res
		}
	}

	pub fn inw(&self) -> u16{
		unsafe{
			let res: u16;
			asm!("inw %dx, %ax" :
				"={ax}"(res) : "{dx}"(self.port) ::
				"volatile");
			res
		}
	}

	pub fn inl(&self) -> u32{
		unsafe{
			let res: u32;
			asm!("inl %dx, %eax" :
				"={eax}"(res) : "{dx}"(self.port) ::
				"volatile");
			res
		}
	}
}
