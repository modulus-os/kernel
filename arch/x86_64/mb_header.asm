section .mb_header
mb_start:
	dd 0xe85250d6
	dd 0
	dd mb_end - mb_start

	dd 0x100000000 - (0xe85250d6 + 0 + (mb_end - mb_start))

	dw 0
	dw 0
	dd 8
mb_end:
