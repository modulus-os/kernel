ENTRY(start)

SECTIONS
{
	. = 1M;

	.boot :
	{
		KEEP(*(.mb_header))
	}

	.text :
	{
		*(.text)
	}
}
