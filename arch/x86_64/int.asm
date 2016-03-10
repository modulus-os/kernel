;;------------------------------------------------------------------------------------------------
;;`arch/x86_64/interrupt_handlers.asm`
;;
;;Assembly wrappers for IDT functions.
;;------------------------------------------------------------------------------------------------

global asm_kb_handler
global asm_lidt;
extern kb_handler

section .text
bits 64

asm_kb_handler:
	call kb_handler
	iretd

asm_lidt:
	mov rdx, [esp + 8]
	lidt[rdx]
	sti
	ret
