use core::ptr;
use memory::PAGE_SIZE;
use memory::frame::Frame;
use memory::alloc::FrameAlloc;

pub const PRESENT: usize = 1 << 0;
pub const WRITABLE: usize = 1 << 1;
pub const USER_ACCESSIBLE: usize = 1 << 2;
pub const WRITE_THROUGH_CACHING: usize = 1 << 3;
pub const DISABLE_CACHE: usize = 1 << 4;
pub const ACCESSED: usize = 1 << 5;
pub const DIRTY: usize = 1 << 6;
pub const HUGE: usize = 1 << 7;
pub const GLOBAL: usize = 1 << 8;
pub const NO_EXECUTE: usize = 1 << 63;

pub const NO_FLAGS: usize = 0o1777777777777777770000;

pub const ENTRY_COUNT: usize = 512;
pub const ENTRY_SIZE: usize = 8;

pub const P4: usize = 0o1777777777777777770000;
pub const P3: usize = 0o1777777777777770000000;
pub const P2: usize = 0o1777777777770000000000;
pub const P1: usize = 0o1777777770000000000000;

pub struct Page {
    pub number: usize,
}

impl Page {
    pub fn new(virtual_addr: usize) -> Self {
        Page { number: virtual_addr / PAGE_SIZE }
    }

    pub fn number(number: usize) -> Self {
        Page { number: number }
    }

    pub fn virtual_addr(&self) -> usize {
        self.number * PAGE_SIZE
    }

    pub fn entry_addr(&self) -> usize {
        let table = self.number / ENTRY_COUNT;
        let entry = self.number % ENTRY_COUNT;

        P1 + (table * ENTRY_COUNT + entry) * ENTRY_SIZE
    }

	#[cfg_attr(rustfmt, rustfmt_skip)]
    pub unsafe fn clear(&self) {
        asm!("invlpg [$0]"
			:
        	: "{rax}"(self.virtual_addr())
        	: "memory"
        	: "intel", "volatile");
    }
    /// Returns index of page in P4 table
    pub fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    /// Returns index of page in P3 table
    pub fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    /// Returns index of page in P2 table
    pub fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    /// Returns index of page in P1 table
    pub fn p1_index(&self) -> usize {
        self.number & 0o777
    }

    pub unsafe fn map<T: FrameAlloc>(&self,
                                     physical_addr: usize,
                                     flags: usize,
                                     allocator: &mut T) {
        let p4 = (P4 + (self.p4_index() * ENTRY_SIZE)) as *mut usize;

        if (*p4 & PRESENT) != 0 {
            print!("P3 present");
        } else {
            print!("P3 not present; creating");
            let address = allocator.alloc().expect("ENOMEM").number * PAGE_SIZE;
            *p4 = address | flags;
        }

        // ptr::write(self.entry_addr() as *mut usize,
        // 	(physical_addr & NO_FLAGS) | flags);
        // self.clear();
    }
}
