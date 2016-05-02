;;------------------------------------------------------------------------------------------------
;;`arch/x64/interrupt_handlers.asm`
;;
;;Assembly wrappers for IDT functions.
;;------------------------------------------------------------------------------------------------

global asm_lidt

global asm_sys
global asm_pit
global asm_kb
global asm_primary_ata
global asm_secondary_ata

extern sys
extern pit
extern kb
extern primary_ata
extern secondary_ata

section .text
bits 64

asm_lidt:
	mov [idtr + 2], rdi
	lidt [idtr]
	sti
	ret

idtr:
	dw 4095
	dq 0

asm_sys:
	mov qword [temp], sys
	jmp isr_stub

asm_pit:
	mov qword [temp], pit
	jmp isr_stub

asm_kb:
	mov qword [temp], kb
	jmp isr_stub

asm_primary_ata:
	mov qword [temp], primary_ata
	jmp isr_stub

asm_secondary_ata:
	mov qword [temp], secondary_ata
	jmp isr_stub

isr_stub:
	push rax
	push rcx
	push rdx
	push rbx
	push rsp
	push rbp
	push rsi
	push rdi

	mov rax, rsp
	push rax

	call [temp]

	pop rax

	pop rdi
	pop rsi
	pop rbp
	pop rsp
	pop rbx
	pop rdx
	pop rcx
	pop rax

	iretq

temp:
	dq 0
