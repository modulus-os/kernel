global lm_start
extern kmain

section .tect
bits 64

lm_start:
	call kmain
	hlt
