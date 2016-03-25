use core::fmt::Write;
use io::display::*;
use common_color;

pub fn init_log(term: &mut terminal::Terminal, string: &str, success: bool) {
	term.set_color(common_color::LCYAN);
	write!(term, " >> ");
	term.set_color(common_color::WHITE);
	write!(term, "{}", string);

	if success {
		term.set_color(common_color::GREEN);
		write!(term, " *\n");
	} else {
		term.set_color(common_color::RED);
		write!(term, " *\n");
	}
	term.set_color(common_color::WHITE);
}
