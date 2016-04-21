;;------------------------------------------------------------------------------------------------
;;`arch/x86_64/interrupt_handlers.asm`
;;
;;Assembly wrappers for IDT functions.
;;------------------------------------------------------------------------------------------------

global asm_kb_handler
global asm_lidt
global asm_int_test

extern kb_handler

section .text
bits 64

asm_divzero:
	iretq

asm_lidt:
	lidt[rdi]
	sti
	;int 0x0
	ret

.hang:
	hlt
