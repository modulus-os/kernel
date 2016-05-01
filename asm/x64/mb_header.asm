;;------------------------------------------------------------------------------------------------
;;`arch/x64/mb_header.asm`
;;
;;Declares multiboot header and tags
;;------------------------------------------------------------------------------------------------

section .mb_header
mb_start:
	dd 0xe85250d6
	dd 0
	dd mb_end - mb_start

	dd 0x100000000 - (0xe85250d6 + 0 + (mb_end - mb_start))


	dw 0
	dw 0
	dd 8

	;Video mode
	;dw 5
	;dw 0
	;dd 20
	;dd 1024
	;dd 768
	;dd 32

mb_end:
