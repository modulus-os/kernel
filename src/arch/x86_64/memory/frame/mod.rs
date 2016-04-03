use memory::PAGE_SIZE;

/// A representation of a physical memory frame
pub struct Frame {
	pub number: usize,
}

impl Frame {
	pub fn from_number(number: usize) -> Frame {
		Frame {
			number: number,
		}
	}

	pub fn from_address(address: usize) -> Frame {
		Frame {
			number: address / PAGE_SIZE,
		}
	}

	pub fn to_address(&self) -> usize {
		self.number * PAGE_SIZE
	}
}
