use memory::PAGE_SIZE;

//use memory::alloc::*;
//use memory::frame::*;

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
