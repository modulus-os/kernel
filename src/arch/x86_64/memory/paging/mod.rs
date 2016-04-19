use memory::PAGE_SIZE;
// use memory::frame::Frame;
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

    /// Returns index of page in P4 table
    pub fn p4_index(&self) -> u64 {
        (self.number >> 27) & 0o777
    }

    /// Returns index of page in P3 table
    pub fn p3_index(&self) -> u64 {
        (self.number >> 18) & 0o777
    }

    /// Returns index of page in P2 table
    pub fn p2_index(&self) -> u64 {
        (self.number >> 9) & 0o777
    }

    /// Returns index of page in P1 table
    pub fn p1_index(&self) -> u64 {
        self.number & 0o777
    }

    fn get_table(address: u64, index: isize) -> u64 {
        unsafe { *(address as *mut u64).offset(index) }
    }

    fn get_table_mut(address: u64, index: isize) -> *mut u64 {
        unsafe { (address as *mut u64).offset(index) }
    }

    /// Tests if next table exists and allocates a new one if not
    fn create_next_table<T: FrameAlloc>(allocator: &mut T, address: u64, index: isize) -> u64 {
        let mut entry = Page::get_table(address, index);
        if (entry & PRESENT) != 0 {

        } else {
            let frame = allocator.alloc();
            unsafe {
                *Page::get_table_mut(address, index) = (frame.unwrap().number * PAGE_SIZE) |
                                                       PRESENT |
                                                       WRITABLE;
            }
            entry = Page::get_table(address, index);
        }

        entry
    }

    /// Sets a page in a PT
    fn create_page(physical_addr: u64, flags: u64, address: u64, index: isize) {
        unsafe {
            *Page::get_table_mut(address, index) = (physical_addr * PAGE_SIZE) | flags;
        }
    }

    /// Create page tables and allocate page
    ///
    /// This function walks through the page tables. If the next table is present, it jumps
    /// to it and continues. Otherwise, it allocates a frame and writes its address to the entry.
    /// Once it is done, it allocates the actual frame.
    pub fn map_page<T: FrameAlloc>(&self, address: u64, allocator: &mut T) {
        // Entry in P4 (P3 location)
        let p4_entry = Page::create_next_table(allocator, P4, self.p4_index() as isize);

        // Entry in P3 (P2 location)
        let p3_entry = Page::create_next_table(allocator,
                                               p4_entry & NO_FLAGS,
                                               self.p3_index() as isize);

        // Entry in P2 (P1 location)
        let p2_entry = Page::create_next_table(allocator,
                                               p3_entry & NO_FLAGS,
                                               self.p2_index() as isize);

        // Entry in P1 (Page or P0 location)
        Page::create_page(address,
                          (PRESENT | WRITABLE),
                          p2_entry & NO_FLAGS,
                          self.p1_index() as isize);
    }
}
