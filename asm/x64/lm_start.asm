;;------------------------------------------------------------------------------------------------
;;`arch/x64/lm_start.asm`
;;
;;First 64 bit file to be called, calls kmain().
;;------------------------------------------------------------------------------------------------

global lm_start
extern kmain
extern kb_handler

section .text
bits 64

lm_start:
	call kmain
	hlt
