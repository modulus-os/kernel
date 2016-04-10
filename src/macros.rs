macro_rules! print {
	($($arg:tt)*) => ({
			use core::fmt::Write;
			$crate::io::display::terminal::TERM.lock().write_fmt(format_args!($($arg)*)).unwrap();
	});
}
