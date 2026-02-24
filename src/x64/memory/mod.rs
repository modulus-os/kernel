use multiboot2;

pub mod alloc;
pub mod paging;

pub const PAGE_SIZE: u64 = 4096;

/// Initialize area frame allocator
///
/// This function retrieves memory map information from multiboot and then creates an
///`AreaFrameAlloc` with the largest memory area (will be modified to support multiple areas).

pub fn init_area_frame_alloc(mb_info_address: usize) -> alloc::area::AreaFrameAlloc {
    let boot_info = unsafe {
        multiboot2::BootInformation::load(mb_info_address as *const multiboot2::BootInformationHeader).unwrap()
    };

    let memory_map_tag = boot_info.memory_map_tag()
                                  .expect("Bootloader did not provide a memory map tag");

    let mut greatest_area_base: u64 = 0;
    let mut greatest_area_len: u64 = 0;
    let mut total_memory: u64 = 0;

    // TODO: Let AreaFrameAlloc use multiple memory areas and remove largest area selector

    for area in memory_map_tag.memory_areas() {
        if area.size() > greatest_area_len {
            greatest_area_base = area.start_address();
            greatest_area_len = area.size();
        }

        total_memory = area.start_address() + area.size();
    }

    print!("Memory: {}MB\n", total_memory / (1024 * 1024));

    let area = alloc::area::Area::new(greatest_area_base + 0x15000,
                                      greatest_area_base + greatest_area_len);
    alloc::area::AreaFrameAlloc::new(area)
}
