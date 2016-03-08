//!-----------------------------------------------------------------------------------------------
//!`src/mutliboot/mod.rs`
//!
//!Retrieves multiboot boot info.
//!-----------------------------------------------------------------------------------------------

pub struct BootInfo {
	pub size: u32,
	reserved: u32,
	pub first: Tag,
}

impl BootInfo {
	pub unsafe fn new(mb_info_address: usize) -> &'static BootInfo {
		let boot_info = &*(mb_info_address as *const BootInfo);
		boot_info
	}

	pub fn tags(&self) -> TagIter {
		TagIter {current: &self.first as *const _}
	}

	pub fn get_tag(&self, typ: u32) -> Option<&'static Tag> {
		self.tags().find(|tag| tag.typ == typ)
	}
}

pub struct Tag {
	pub typ: u32,
	pub size: u32,
}

struct TagIter {
	current: *const Tag,
}

impl Iterator for TagIter {
	type Item = &'static Tag;
	fn next(&mut self) -> Option<&'static Tag> {
		match unsafe{&*self.current}{
			&Tag{typ: 0, size: 8} => None,
			tag => {
				let mut tag_addr = self.current as usize;
				tag_addr += tag.size as usize;
				tag_addr = ((tag_addr-1) & !0x7) + 0x8;
				self.current = tag_addr as *const _;

				Some(tag)
			},
		}
	}
}
