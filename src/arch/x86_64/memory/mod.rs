//!-----------------------------------------------------------------------------------------------
//!`src/memory/mod.rs`
//!
//!Important memory management features such as page management and page frame allocation.
//!-----------------------------------------------------------------------------------------------

use multiboot2;
use core::fmt::Write;
use io::display::*;

pub fn init_frame_alloc(term: &mut terminal::Terminal, mb_info_address: usize) {
	let boot_info = unsafe { multiboot2::load(mb_info_address) };

	let memory_map_tag = boot_info.memory_map_tag().expect(
		"Bootloader did not provide a memory map tag");

	for area in memory_map_tag.memory_areas() {
		write!(term, "Area base: {}, Area length: {}\n", area.base_addr, area.length);
	}
}
