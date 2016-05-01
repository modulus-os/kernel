use memory::PAGE_SIZE;

pub mod area;

/// A representation of a physical memory frame
pub struct Frame {
    pub number: u64,
}

impl Frame {
    pub fn from_number(number: u64) -> Frame {
        Frame { number: number }
    }

    pub fn from_address(address: u64) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    pub fn to_address(&self) -> u64 {
        self.number * PAGE_SIZE
    }
}

pub trait FrameAlloc {
    fn alloc(&mut self) -> Option<Frame>;
    fn dealloc(&mut self, frame: Frame);
}
