use memory::paging;

pub fn test() {
	print!("translate(0) = {}", paging::translate(0));
}
