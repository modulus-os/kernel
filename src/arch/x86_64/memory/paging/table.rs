use memory::{ENTRY_COUNT, PAGE_SIZE};
use memory::paging::*;
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
