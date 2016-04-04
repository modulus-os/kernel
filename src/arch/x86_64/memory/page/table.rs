use memory::{ ENTRY_COUNT };
use memory::frame::*;

/// A page table.
pub struct Table {
	entries: [Entry; ENTRY_COUNT],
}

impl Table {
	pub fn clear(&mut self) {
		for entry in self.entries.iter_mut() {
			entry.clear();
		}
	}

    /// Returns address of the next table with the given index
	/// 
    /// Because of recursive mapping, the next page table
	/// can be accessed by shifting 9 bits left.
	pub fn next_table_address(&self, index: usize) -> Option<usize> {
		let flags = self.entries[index].flags();
		// Huge pages not currently supported
		if flags.contains(PRESENT) && !flags.contains(HUGE) {
			let address = self as *const _ as usize;
			Some((address << 9) | (index << 12))
		} else {
			None
		}
	}

	pub fn next_table(&self, index: usize) -> &Table {
		let address = self.next_table_address(index).expect("Page not present or is huge");
		unsafe {&*(address as *const _)}
	}

	pub fn next_table_mut(&self, index: usize) -> &mut Table {
		let address = self.next_table_address(index).expect("Page not present or is huge");
		unsafe {&mut *(address as *mut _)}
	}

}

/// A page table entry
///
/// Used in page tables to map memory.
pub struct Entry(u64);

bitflags! {
    /// Page table entry flags
    ///
	/// Special flags in table entries used for privileges, cache, etc.
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

impl Entry {
	pub fn clear(&mut self) {
		self.0 = 0;
	}

	pub fn is_used(&self) -> bool {
		self.0 != 0
	}

	pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
		assert!(frame.to_address() & !0x000fffff_fffff000 == 0);
		self.0 = frame.to_address() as u64 | flags.bits();
	}

	pub fn flags(&self) -> EntryFlags {
		EntryFlags::from_bits_truncate(self.0)
	}

	pub fn address(&self) -> u64 {
		self.0 & 0x000fffff_fffff000
	}

	pub fn frame(&self) -> Option<Frame> {
		// Make sure this page is PRESENT
		if self.flags().contains(PRESENT) {
			Some(Frame::from_address(self.address() as usize))
		} else {
			None
		}
	}
}
