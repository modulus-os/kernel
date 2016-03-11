use core::fmt::Write;

use io::display::*;

use common_color;

pub fn panic(file: &str, line: u32) {
	let mut term = terminal::Terminal::new();

	term.set_color(common_color::RED);
	write!(term, "\n\n\n\n");
	error_symbol(&mut term);
	write!(term, "    !PANIC!    \n\n");

	term.set_color(common_color::WHITE);

	write!(term, "At {}:{}", file, line);
}

pub fn exception(err: &str) {
	let mut term = terminal::Terminal::new();

	term.set_color(common_color::RED);
	write!(term, "\n\n\n\n");
	error_symbol(&mut term);
	write!(term, "  !EXCEPTION!  \n\n");

	term.set_color(common_color::WHITE);

	write!(term, "{}", err);
}


fn error_symbol(term: &mut terminal::Terminal) {
	write!(term, "       x       \n");
	write!(term, "      x x      \n");
	write!(term, "     x | x     \n");
	write!(term, "    x  |  x    \n");
	write!(term, "   x       x   \n");
	write!(term, "  x    *    x  \n");
	write!(term, " x===========x \n");
}
