use multiboot2::{BootInformation, BootInformationHeader};

pub mod alloc;
pub mod paging;

pub const PAGE_SIZE: u64 = 4096;

/// Initialize area frame allocator
///
/// This function retrieves memory map information from multiboot and then creates an
/// `AreaFrameAlloc` with the largest memory area (will be modified to support multiple areas).
pub fn init_area_frame_alloc(mb_info_address: usize) -> alloc::area::AreaFrameAlloc {
    // SAFETY: The Multiboot2 boot info address must be valid and aligned
    let boot_info = unsafe {
        let header = BootInformationHeader::from_addr(mb_info_address)
            .expect("Invalid Multiboot2 header");
        BootInformation::new(header)
            .expect("Could not read Multiboot2 boot information")
    };

    let memory_map_tag = boot_info
        .memory_map_tag()
        .expect("Bootloader did not provide a memory map tag");

    let mut greatest_area_base: u64 = 0;
    let mut greatest_area_len: u64 = 0;
    let mut total_memory: u64 = 0;

    // TODO: Let AreaFrameAlloc use multiple memory areas and remove largest area selector
    for area in memory_map_tag.memory_areas() {
        if area.length > greatest_area_len {
            greatest_area_base = area.base_addr;
            greatest_area_len = area.length;
        }

        total_memory = area.base_addr + area.length;
    }

    print!("Memory: {}MB\n", total_memory / (1024 * 1024));

    let area = alloc::area::Area::new(
        greatest_area_base + 0x15000,
        greatest_area_base + greatest_area_len,
    );
    alloc::area::AreaFrameAlloc::new(area)
}
