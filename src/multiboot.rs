//!-----------------------------------------------------------------------------------------------
//!`src/multiboot.rs`
//!
//!Retrieves multiboot boot info.
//!-----------------------------------------------------------------------------------------------

#![no_std]

pub unsafe fn load(mb_info_address: usize) -> BootInfo {
	let start = mb_info_address as *const u32;
	BootInfo {size: *start, reserved: *start.offset(2)}
}

pub struct BootInfo {
	pub size: u32,
	reserved: u32,
}

pub struct Tag {
	typ: u32,
	size: u32,
}

struct TagIter {
	current: *const Tag,
}

/*impl Iterator for TagIter {
	type Item = &'static Tag;

	pub fn next(&mut self) -> Option<&'static Tag> {
		let n_tag = &self.current + *self.current.size;
		Some(Tag{typ: 1, size: 1})
	}
}*/
