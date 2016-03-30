use memory::paging::table::PageTable;

pub fn test() {
	PageTable::translate(0);
}
