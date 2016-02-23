//!-----------------------------------------------------------------------------------------------
//!`src/lib/support.rs`
//!
//!Provides essential std functions memcpy(), memmove(), memset(), and memcmp().
//!-----------------------------------------------------------------------------------------------

#![feature(asm)]
#![allow(private_no_mangle_fns)]
#![allow(dead_code)]

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	let mut i = 0;
	while i < n {
		*dest.offset(i as isize) = *src.offset(i as isize);
		i += 1;
	}
	dest
}

#[no_mangle]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    if src < dest as *const u8 {
		let mut i = n;
		while i != 0 {
			i -= 1;
			*dest.offset(i as isize) = *src.offset(i as isize);
		}
    } else {
		let mut i = 0;
		while i < n {
			*dest.offset(i as isize) = *src.offset(i as isize);
			i += 1;
		}
	}
    dest
}

#[no_mangle]
pub unsafe extern fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8 {
	let mut i = 0;
	while i < n {
		*dest.offset(i as isize) = c as u8;
		i += 1;
	}
	dest
}

#[no_mangle]
pub unsafe extern fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> i32 {
	let mut i = 0;
	while i < n {
		if *src1.offset(i as isize) != *src2.offset(i as isize) {
			return *src1.offset(i as isize) as i32 - *src2.offset(i as isize) as i32;
		}
		i += 1;
	}
	0
}
