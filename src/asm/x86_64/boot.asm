;;------------------------------------------------------------------------------------------------
;;`arch/x86_64/boot.asm`
;;
;;Kernel entry file, performs basic system checks and enables paging and long mode.
;;------------------------------------------------------------------------------------------------

global start
extern lm_start

section .boot
bits 32

start:
	;Set up stack
	mov esp, stack_t

	;Store boot info address
	mov edi, ebx

	;Call checks
	call check_mb
	call check_cpuid
	call check_long_mode

	;Setup paging
	call paging_setup_tables
	call paging_enable
	call sse_enable

	;Load GDT
	lgdt[gdt.pointer]

	mov ax, 16
	mov ss, ax ;stack
	mov ds, ax ;data
	mov es, ax ;extra

	;Long jmp to 64 bit code
	jmp gdt.code:lm_start

	hlt

;Multiboot header check:
check_mb:
	cmp eax, 0x36d76289
	jne .no_mb

	ret
.no_mb:
	mov ah, "0"
	jmp error

;CPUID check:
check_cpuid:
	;Copy FLAGS in to EAX via stack
	pushfd
	pop eax
	;Copy to ECX as well for comparing later on
	mov ecx, eax
	;Flip the ID bit
	xor eax, 1 << 21
	;Copy EAX to FLAGS via the stack
	push eax
	popfd
	;Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
	pushfd
	pop eax
	;Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit
	;back if it was ever flipped).
	push ecx
	popfd
	;Compare EAX and ECX. If they are equal then that means the bit wasn't
	;flipped, and CPUID isn't supported.
	cmp eax, ecx
	je .no_cpuid

	ret
.no_cpuid:
	mov ah, "1"
	jmp error

;Long mode check
check_long_mode:
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	jb .no_long_mode

	mov eax, 0x80000001
	cpuid
	test edx, 1 << 29
	jz .no_long_mode

	ret
.no_long_mode:
	mov ah, "2"
	jmp error

;Enable paging and flat map the fist 1G of memory
paging_setup_tables:
	;Map last P4 entry to P4 for recursive mapping
	mov eax, p4_table
	or eax, 0b11
	mov [p4_table + 511 * 8], eax

	;Map first P4 entry to P3
	mov eax, p3_table
	or eax, 0b11
	mov dword [p4_table], eax

	;Map first P3 entry to P2
	mov eax, p2_table
	or eax, 0b11
	mov dword [p3_table], eax

	mov ecx, 0
.map_p2_table:
	mov eax, 0x200000
	mul ecx
	or eax, 0b10000011
	mov [p2_table + ecx * 8], eax

	inc ecx
	cmp ecx, 512
	jne .map_p2_table

	ret

paging_enable:
	;Point cr3 to P4
	mov eax, p4_table
	mov cr3, eax

	;Enable PAE
	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	;Enable long mode
	mov ecx, 0xc0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	;Enable paging
	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	ret

sse_enable:
    mov eax, 0x1
    cpuid
    test edx, 1 << 25

    mov eax, cr0
    and ax, 0xfffb
    or ax, 0x2
    mov cr0, eax
    mov eax, cr4
    or ax, 3 << 9
    mov cr4, eax

    ret
error:
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte [0xb800a], ah
	hlt

section .bss
align 4096

p4_table:
	resb 4096

p3_table:
	resb 4096

p2_table:
	resb 4096

stack_b:
	resb 16384

stack_t:

section .rodata

gdt:
	dq 0

.code: equ $ - gdt
	dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)

.data: equ $ - gdt
	dq (1<<44) | (1<<47) | (1<<41)

.pointer:
	dw $ - gdt - 1
	dq gdt
