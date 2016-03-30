use memory::{ENTRY_COUNT, PAGE_SIZE};
use memory::page::*;
use memory::frame::*;
use memory::alloc::*;

pub const P4: *mut PageTable = 0xffff_ffff_ffff_f000 as *mut _;

///Structure for manipulating page tables
pub struct PageTable {
	entries: [PageEntry; ENTRY_COUNT],
}

impl PageTable {
	pub fn clear(&mut self) {
		for entry in self.entries.iter_mut() {
			entry.clear();
		}
	}

	//Thanks to recursive mapping, the next page table
	//can be accessed by shifting 9 bits left
	pub fn next_table_address(&self, index: usize) -> Option<usize> {
		let flags = self.entries[index].flags();
		//Huge pages not currently supported
		if flags.contains(PRESENT) && !flags.contains(HUGE) {
			let address = self as *const _ as usize;
			Some((address << 9) | (index << 12))
		} else {
			None
		}
	}

	pub fn next_table(&self, index: usize) -> Option<&PageTable> {
		let address = self.next_table_address(index).expect("Unable to get next_table");
		Some(unsafe {&*(address as *const _)})
	}

	pub fn next_table_mut(&self, index: usize) -> Option<&mut PageTable> {
		let address = self.next_table_address(index).expect("Unable to get next_table_mut");
		Some(unsafe {&mut *(address as *mut _)})
	}

	pub fn next_table_create<A>(&mut self, index: usize, allocator: &mut A) -> &mut PageTable
		where A: FrameAlloc
	{
		if self.next_table(index).is_none() {
			assert!(!self.entries[index].flags().contains(HUGE), "Huge pages not supported");
			let frame = allocator.alloc().expect("Unable to allocate frame");
			self.entries[index].set(frame, PRESENT | WRITABLE);
			self.next_table_mut(index).unwrap().clear();
		}
		self.next_table_mut(index).unwrap()
	}

	pub fn translate(virtual_address: usize) -> Option<usize> {
		print!("Testing...");
		let offset = virtual_address % PAGE_SIZE;
		let frame = PageTable::translate_page(Page::from_address(virtual_address));
		Some(0 * PAGE_SIZE + offset)
	}

	pub fn translate_page(page: Page) -> Option<Frame> {
		//Walk down page tables to find frame
		let p3 = unsafe { &*P4 }.next_table(page.p4_index());

		let frame = p3.and_then(|p3| p3.next_table(page.p3_index()))
      		.and_then(|p2| p2.next_table(page.p2_index()))
      		.and_then(|p1| p1.entries[page.p1_index()].frame());
		Some(Frame::from_number(0))
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
