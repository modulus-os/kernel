;;------------------------------------------------------------------------------------------------
;;`arch/x86_64/interrupt_handlers.asm`
;;
;;Assembly wrappers for IDT functions.
;;------------------------------------------------------------------------------------------------

global asm_lidt

section .text
bits 64

asm_lidt:
	mov [idtr + 2], rdi
	lidt [idtr]
	sti
	ret

.hang:
	hlt
	jmp .hang

idtr:
	dw 4095
	dq 0
