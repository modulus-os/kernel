global lm_start
extern kmain

section .text
bits 64

lm_start:
	call kmain
	hlt
