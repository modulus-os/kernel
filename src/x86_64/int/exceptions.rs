#[no_mangle]
#[naked]
/// Divide by zero; Interrupt 0
pub fn de() {
    print!("Division by 0\n");
}

/// Debug; Interrupt 1
#[no_mangle]
#[naked]
pub fn db() {
    print!("Debug\n");
}

/// NMI; Interrupt 2
#[no_mangle]
#[naked]
pub fn nmi() {
    print!("NMI interrupt\n");
}

/// Breakpoint; Interrupt 3
#[no_mangle]
#[naked]
pub fn bp() {
    print!("Breakpoint\n");
}

/// Overflow; Interrupt 4
#[no_mangle]
#[naked]
pub fn of() {
    print!("Overflow\n");
}

/// Bound; Interrupt 5
#[no_mangle]
#[naked]
pub fn br() {
    print!("Bound range exceeded\n");
}

/// Undefined opcode; Interrupt 6
#[no_mangle]
#[naked]
pub fn ud() {
    print!("Undefined opcode\n");
}

/// No math coprocessor; Interrupt 7
#[no_mangle]
#[naked]
pub fn nm() {
    print!("No math coprocessor\n");
}

/// Double fault; Interrupt 8
#[no_mangle]
#[naked]
pub fn df() {
    print!("Double fault\n");
}

/// Coprocessor segment overrun; Interrupt 9
#[no_mangle]
#[naked]
pub fn cmf() {
    print!("Coprocessor segment overrun\n");
}

/// Invalid TSS; Interrupt 10
#[no_mangle]
#[naked]
pub fn ts() {
    print!("Invalid TSS\n");
}

/// Segment not present; Interrupt 11
#[no_mangle]
#[naked]
pub fn np() {
    print!("Segment not present\n");
}

/// Stack segment fault; Interrupt 12
#[no_mangle]
#[naked]
pub fn ss() {
    print!("Stack segment fault\n");
}

/// General protection fault; Interrupt 13
#[no_mangle]
#[naked]
pub fn gp() {
    print!("General protection fault\n");
}

/// Page fault; Interrupt 14
#[no_mangle]
#[naked]
pub fn pf() {
    print!("Page fault\n");
}

// Interrupt 15 is reserved


/// Floating-point error (math fault); Interrupt 16
#[no_mangle]
#[naked]
pub fn mf() {
    print!("Floating-point error (math fault)\n");
}

/// Alignment check; Interrupt 17
#[no_mangle]
#[naked]
pub fn ac() {
    print!("Alignment check\n");
}

/// Machine check; Interrupt 18
#[no_mangle]
#[naked]
pub fn mc() {
    print!("Machine check\n");
}

/// SIMD floating-point exception; Interrupt 19
#[no_mangle]
#[naked]
pub fn xm() {
    print!("SIMD floating-point exception\n");
}

/// Virtualization exception; Interrupt 20
#[no_mangle]
#[naked]
pub fn ve() {
    print!("Virtualization exception\n");
}

// Interrupts 21-31 are reserved
