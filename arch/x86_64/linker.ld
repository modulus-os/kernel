ENTRY(start)

SECTIONS
{
	. = 1M;

	.boot :
	{
		*(.mb_header)
	}

	.text :
	{
		*(.text)
	}
}
