//!-----------------------------------------------------------------------------------------------
//!`src/arch/x86_64/memory/frame_alloc.rs`
//!
//!Physical frame allocator
//!-----------------------------------------------------------------------------------------------

///A single page frame

use memory::PAGE_SIZE;

pub struct Frame {
	pub number: usize,
}

impl Frame {
	pub fn new(number: usize) -> Frame {
		Frame {
			number: number,
		}
	}

	pub fn address(address: usize) -> Frame {
		Frame {
			number: address / PAGE_SIZE,
		}
	}
}

///A structure for managing page frames
pub struct FrameAlloc {
	next: Frame,
	min: usize,
	max: usize,
}

impl FrameAlloc {
	pub fn new(min: usize, max: usize) -> FrameAlloc {
		FrameAlloc {
			next: Frame::address(min),
			min: min,
			max: max,
		}
	}

	pub fn alloc(&mut self) -> Option<Frame> {
		let frame = Frame::new(self.next.number);
		let last_frame = Frame::address(self.max);
		
		if self.next.number < last_frame.number {
			self.next.number += 1;

			return Some(frame);
		} else {
			None
		}
	}

	pub fn dealloc() {
		unimplemented!();
	}
}
