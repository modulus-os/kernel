//!-----------------------------------------------------------------------------------------------
//!`src/lib/multiboot.rs`
//!
//!Retrieves multiboot boot info.
//!-----------------------------------------------------------------------------------------------

pub struct Tag {
	pub typ: u32,
	pub size: u32,
}

pub struct BootInfo {
	pub size: u32,
	reserved: u32,
	pub first: Tag,
}

impl BootInfo {
	pub unsafe fn new(mb_info_address: usize) -> BootInfo {
		let start = mb_info_address as *const usize;
		BootInfo {size: *start as u32, reserved: 0,
			first: Tag {typ: *start.offset(3) as u32, size: *start.offset(4) as u32}}
	}

	pub fn memmap(&self) {

	}
}
