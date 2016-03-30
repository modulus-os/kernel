pub mod alloc;
pub mod frame;
pub mod page;

use multiboot2;

pub const PAGE_SIZE: usize = 4096;
const ENTRY_COUNT: usize = 512;


pub fn init_area_frame_alloc(mb_info_address: usize) -> alloc::area::AreaFrameAlloc {
	let boot_info = unsafe { multiboot2::load(mb_info_address) };

	let memory_map_tag = boot_info.memory_map_tag().expect(
		"Bootloader did not provide a memory map tag");

	let mut greatest_area_base: usize = 0;
	let mut greatest_area_len: usize = 0;
	let mut total_memory: u64 = 0;

	for area in memory_map_tag.memory_areas() {
		if area.length as usize > greatest_area_len {
			greatest_area_base = area.base_addr as usize;
			greatest_area_len = area.length as usize;
		}

		total_memory += area.length;
	}

	print!("Total memory: {} bytes\n", total_memory);

	let area = alloc::area::Area::new(greatest_area_base, greatest_area_base + greatest_area_len);
	alloc::area::AreaFrameAlloc::new(area)
}
