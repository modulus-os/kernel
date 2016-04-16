use core::ptr;
use memory::PAGE_SIZE;
use memory::frame::Frame;
use memory::alloc::FrameAlloc;

pub const PRESENT: u64 = 1 << 0;
pub const WRITABLE: u64 = 1 << 1;
pub const USER_ACCESSIBLE: u64 = 1 << 2;
pub const WRITE_THROUGH_CACHING: u64 = 1 << 3;
pub const DISABLE_CACHE: u64 = 1 << 4;
pub const ACCESSED: u64 = 1 << 5;
pub const DIRTY: u64 = 1 << 6;
pub const HUGE: u64 = 1 << 7;
pub const GLOBAL: u64 = 1 << 8;
pub const NO_EXECUTE: u64 = 1 << 63;

pub const NO_FLAGS: u64 = 0o1777777777777777770000;

pub const ENTRY_COUNT: u64 = 512;
pub const ENTRY_SIZE: u64 = 8;

pub const P4: u64 = 0o1777777777777777770000;

pub struct Page {
    number: u64,
}

impl Page {
    pub fn new(number: u64) -> Self {
        Page { number: number }
    }

    pub fn virt_addr(&self) -> u64 {
        self.number * PAGE_SIZE
    }

    fn get_p4() -> *mut u64 {
        P4 as *mut u64
    }

    pub fn create_tables() {
        let p4_entry = unsafe { *Page::get_p4().offset(0) };
        print!("{:0b}", p4_entry & NO_FLAGS);
    }
}
