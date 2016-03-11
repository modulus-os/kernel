//!-----------------------------------------------------------------------------------------------
//!`src/io/interrupts/exceptions.rs`
//!
//!Exception handlers for the CPU
//!-----------------------------------------------------------------------------------------------

use error;

#[no_mangle]
pub fn divzero() {
	error::exception("Divide By Zero");
}

#[no_mangle]
pub fn page_fault() {
	error::exception("Divide By Zero");
}
