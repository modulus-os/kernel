//!-----------------------------------------------------------------------------------------------
//!`src/arch/x86_64/memory/mod.rs`
//!
//!Important memory management features such as page management and page frame allocation.
//!-----------------------------------------------------------------------------------------------

pub mod frame_alloc;

use multiboot2;

pub const PAGE_SIZE: usize = 4096;

pub fn init_frame_alloc(mb_info_address: usize) -> frame_alloc::FrameAlloc {
	let boot_info = unsafe { multiboot2::load(mb_info_address) };

	let memory_map_tag = boot_info.memory_map_tag().expect(
		"Bootloader did not provide a memory map tag");

	let mut greatest_area_base: usize = 0;
	let mut greatest_area_len: usize = 0;
	let mut total_memory: u64 = 0;

	for area in memory_map_tag.memory_areas() {
		//print!("Area base: {}, Area end: {}\n", area.base_addr, area.base_addr + area.length);
		if area.length as usize > greatest_area_len {
			greatest_area_base = area.base_addr as usize;
			greatest_area_len = area.length as usize;
		}

		total_memory += area.length;
	}

	print!("Total memory: {} bytes\n", total_memory);

	frame_alloc::FrameAlloc::new(greatest_area_base, greatest_area_base + greatest_area_len)
}
