use memory::PAGE_SIZE;

use memory::alloc::*;
use memory::frame::*;

pub mod table;
pub mod test;

///A representation of a virtual memory page
pub struct Page {
	pub number: usize,
}

impl Page {
	pub fn from_number(number: usize) -> Page {
		Page {
			number: number,
		}
	}

	pub fn from_address(address: usize) -> Page {
		//Make sure address is valid
		assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000);
		Page {
			number: address / PAGE_SIZE,
		}
	}

	pub fn to_address(&self) -> usize {
		self.number * PAGE_SIZE
	}

	//Finds index of page in P4 table
	pub fn p4_index(&self) -> usize {
		(self.number >> 27) & 0o777
	}

	//Finds index of page in P3 table
	pub fn p3_index(&self) -> usize {
		(self.number >> 18) & 0o777
	}

	//Finds index of page in P2 table
	pub fn p2_index(&self) -> usize {
		(self.number >> 9) & 0o777
	}

	//Finds index of page in P1 table
	pub fn p1_index(&self) -> usize {
		self.number & 0o777
	}
}

///A page table entry
pub struct PageEntry(u64);

bitflags! {
	///Flags used by page table
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
		//Make sure this page is PRESENT
		if self.flags().contains(PRESENT) {
			Some(Frame::from_address(self.address() as usize))
		} else {
			None
		}
	}
}
