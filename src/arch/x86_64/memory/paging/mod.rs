//!-----------------------------------------------------------------------------------------------
//!`src/arch/x86_64/memory/paging/mod.rs`
//!
//!Paging management
//!-----------------------------------------------------------------------------------------------

pub mod test;

use memory::{frame_alloc, PAGE_SIZE};
use memory::frame_alloc::{Frame, FrameAlloc};

const ENTRY_COUNT: usize = 512;

pub const P4: *mut PageTable = 0xffffffff_fffff000 as *mut _;

///A virtual page, similar to memory::frame_alloc::Frame but representing a virtual address.
pub struct Page {
	number: usize,
}

impl Page {
	pub fn address(address: usize) -> Page {
		//Make sure address is valid
		assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000);
		Page {
			number: address / PAGE_SIZE
		}
	}

	pub fn index_p4(&self) -> usize {
		(self.number >> 27) & 0o777
	}

	pub fn index_p3(&self) -> usize {
		(self.number >> 18) & 0o777
	}

	pub fn index_p2(&self) -> usize {
		(self.number >> 9) & 0o777
	}

	pub fn index_p1(&self) -> usize {
		self.number & 0o777
	}
}

///An entry in a page table.
#[derive(Clone, Copy)]
pub struct PageEntry(u64);

bitflags! {
	///Special flags for pages
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

	pub fn frame(&self) -> Option<Frame> {
		if self.flags().contains(PRESENT) {
			Some(Frame::address((self.0 & 0x000fffff_fffff000) as usize))
		} else {
			None
		}
	}

	pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
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

///A table of pages for page mapping
pub struct PageTable {
	entries: [PageEntry; ENTRY_COUNT],
}

impl PageTable {
	pub fn clear(&mut self) {
		for entry in self.entries.iter_mut() {
			entry.clear();
		}
	}

	pub fn next_table_address(&self, index: usize) -> Option<usize>{
		let flags = self.entries[index].flags();
		if flags.contains(PRESENT) && !flags.contains(HUGE) {
			let address = self as *const _ as usize;
			Some((address << 9) | (index << 12))
		} else {
			None
		}
	}

	pub fn next_table(&self, index: usize) -> Option<&PageTable> {
		let address = self.next_table_address(index).unwrap();
		Some(unsafe {&*(address as *const _)})
	}

	pub fn next_table_mut(&self, index: usize) -> Option<&mut PageTable> {
		let address = self.next_table_address(index).unwrap();
		Some(unsafe {&mut *(address as *mut _)})
	}

	pub fn next_table_create(&mut self, index: usize, allocator: &mut FrameAlloc) -> &mut PageTable{
		if self.next_table(index).is_none() {
			assert!(!self.entries[index].flags().contains(HUGE), "Page is huge");
			let frame = allocator.alloc().expect("Unable to allocate frame");
			self.entries[index].set(frame, PRESENT | WRITABLE);
			self.next_table_mut(index).unwrap().clear();
		}
		self.next_table_mut(index).unwrap()
	}
}


///Translates a virtual address to a physical address.
pub fn translate(virtual_address: usize) -> usize{
	let offset = virtual_address % PAGE_SIZE;
	let frame = translate_page(Page::address(virtual_address)).expect("Page is huge");
	frame.number * PAGE_SIZE + offset
}

fn translate_page(page: Page) -> Option<Frame> {
	let p3 = unsafe { &*P4 }.next_table(page.index_p4());

	//Huge pages are not currently supported
	p3.and_then(|p3| p3.next_table(page.index_p3()))
		.and_then(|p2| p2.next_table(page.index_p2()))
		.and_then(|p1| p1.entries[page.index_p1()].frame())
}

pub fn map_to(page: Page, frame: Frame, flags: EntryFlags, allocator: &mut FrameAlloc) {
	let p4 = unsafe { &mut *P4 };
	//Walk down page tables
	let mut p3 = p4.next_table_create(page.index_p4(), allocator);
	let mut p2 = p3.next_table_create(page.index_p3(), allocator);
	let mut p1 = p2.next_table_create(page.index_p2(), allocator);

	assert!(!p1.entries[page.index_p1()].used());
	p1.entries[page.index_p1()].set(frame, flags | PRESENT);
}

pub fn map_free(page: Page, flags: EntryFlags, allocator: &mut FrameAlloc) {
	let frame = allocator.alloc().expect("Unable to allocate new frame");
	map_to(page, frame, flags, allocator);
}

pub fn unmap(page: Page, /*allocator: &mut FrameAlloc*/) {
	//assert!(translate(page.number * PAGE_SIZE).is_some());

	let mut p4 = unsafe { &mut *P4 };
	let p1 = p4.next_table_mut(page.index_p4())
		.and_then(|p3| p3.next_table_mut(page.index_p3()))
		.and_then(|p2| p2.next_table_mut(page.index_p2()))
		.expect("Page is huge");
	let frame = p1.entries[page.index_p1()].frame().unwrap();
	p1.entries[page.index_p1()].clear();
	//allocator.dealloc(frame);
}
