;;------------------------------------------------------------------------------------------------
;;`arch/x86_64/interrupt_handlers.asm`
;;
;;Assembly wrappers for IDT functions.
;;------------------------------------------------------------------------------------------------

global asm_kb_handler
extern kb_handler

section .text
bits 64

asm_kb_handler:
	call kb_handler
	hlt
