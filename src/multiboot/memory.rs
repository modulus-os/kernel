//!-----------------------------------------------------------------------------------------------
//!`src/mutliboot/memory.rs`
//!
//!Retrieves memory map from multiboot boot info.
//!-----------------------------------------------------------------------------------------------

pub struct MemIter {
	current: usize,
}

impl Iter for MemIter {
	type Item = &'static 
}
