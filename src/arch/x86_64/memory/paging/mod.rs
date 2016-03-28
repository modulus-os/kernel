//!-----------------------------------------------------------------------------------------------
//!`src/arch/x86_64/memory/paging/mod.rs`
//!
//!Paging management
//!-----------------------------------------------------------------------------------------------

use memory::{frame_alloc, PAGE_SIZE};

const ENTRY_COUNT: usize = 512;

pub const P4: *mut Table = 0xffffffff_fffff000 as *mut _;

///An entry in a page table
pub struct PageEntry(u64);

bitflags! {
	flags EntryFlags: u64 {
		const PRESENT = 1 << 0,
		const WRITABLE = 1 << 1,
		const USER_ACCESSIBLE = 1 << 2,
		const WRITE_THROUGH_CACHING = 1 << 3,
		const DISABLE_CACHE = 1 << 4,
		const ACCESSED = 1 << 5,
		const DIRTY = 1 << 6,
		const HUGE = 1 << 7,
		const GLOBAL = 1 << 8,
		const NO_EXECUTE = 1 << 63,
	}
}

impl PageEntry {
	pub fn flags(&self) -> EntryFlags {
		EntryFlags::from_bits_truncate(self.0)
	}

	pub fn frame(&self) -> Option<frame_alloc::Frame> {
		if self.flags().contains(PRESENT) {
			Some(frame_alloc::Frame::address((self.0 & 0x000fffff_fffff000) as usize))
		} else {
			None
		}
	}

	pub fn set(&mut self, frame: frame_alloc::Frame, flags: EntryFlags) {
		let address = frame.number * PAGE_SIZE;
		assert!(address & 0xfff00000_00000fff == 0);
		self.0 = address as u64 | flags.bits();
	}

	pub fn used(&self) -> bool {
		self.0 != 0
	}

	pub fn clear(&mut self) {
		self.0 = 0;
	}
}


pub struct PageTable {
	entries: [PageEntry; ENTRY_COUNT],
}

impl PageTable {
	pub fn clear(&mut self) {
		for entry in self.entries.iter_mut() {
			entry.clear();
		}
	}
}
