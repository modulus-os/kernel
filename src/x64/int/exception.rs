use core::arch::asm;

/// Division By Zero; Interrupt 0
#[unsafe(naked)]
pub fn de() {
    print!("Division By Zero\n");
    exception_end();
}

/// Debug; Interrupt 1
#[unsafe(naked)]
pub fn db() {
    print!("Debug\n");
    exception_end();
}

/// NMI Interrupt; Interrupt 2
#[unsafe(naked)]
pub fn nmi() {
    print!("NMI Interrupt\n");
    exception_end();
}

/// Breakpoint; Interrupt 3
#[unsafe(naked)]
pub fn bp() {
    print!("Breakpoint\n");
    exception_end();
}

/// Overflow; Interrupt 4
#[unsafe(naked)]
pub fn of() {
    print!("Overflow\n");
    exception_end();
}

/// Bound Range Exceeded; Interrupt 5
#[unsafe(naked)]
pub fn br() {
    print!("Bound Range Exceeded\n");
    exception_end();
}

/// Undefined Opcode; Interrupt 6
#[unsafe(naked)]
pub fn ud() {
    print!("Undefined Opcode\n");
    exception_end();
}

/// No Math CoProcessor; Interrupt 7
#[unsafe(naked)]
pub fn nm() {
    print!("No Math CoProcessor\n");
    exception_end();
}

/// Double Fault; Interrupt 8
#[unsafe(naked)]
pub fn df() {
    print!("Double Fault\n");
    exception_end();
}

/// CoProcessor Segment Overrun; Interrupt 9
#[unsafe(naked)]
pub fn cmf() {
    print!("CoProcessor Segment Overrun\n");
    exception_end();
}

/// Invalid TSS; Interrupt 10
#[unsafe(naked)]
pub fn ts() {
    print!("Invalid TSS\n");
    exception_end();
}

/// Segment Not Present; Interrupt 11
#[unsafe(naked)]
pub fn np() {
    print!("Segment Not Present\n");
    exception_end();
}

/// Stack Segment Fault; Interrupt 12
#[unsafe(naked)]
pub fn ss() {
    print!("Stack Segment Fault\n");
    exception_end();
}

/// General Protection Fault; Interrupt 13
#[unsafe(naked)]
pub fn gp() {
    print!("General Protection Fault\n");
    exception_end();
}

/// Page Fault; Interrupt 14
#[unsafe(naked)]
pub fn pf() {
    print!("Page Fault\n");
    exception_end();
}

// Interrupt 15 is reserved


/// Floating-point Error (Math Fault); Interrupt 16
#[unsafe(naked)]
pub fn mf() {
    print!("Floating-point Error (Math Fault)\n");
    exception_end();
}

/// Alignment Check; Interrupt 17
#[unsafe(naked)]
pub fn ac() {
    print!("Alignment Check\n");
    exception_end();
}

/// Machine Check; Interrupt 18
#[unsafe(naked)]
pub fn mc() {
    print!("Machine Check\n");
    exception_end();
}

/// SIMD Floating-point Exception; Interrupt 19
#[unsafe(naked)]
pub fn xm() {
    print!("SIMD Floating-point Exception\n");
    exception_end();
}

/// Virtualization Exception; Interrupt 20
#[unsafe(naked)]
pub fn ve() {
    print!("Virtualization Exception\n");
    exception_end();
}

// Interrupts 21-31 are reserved

/// Halt system
pub fn exception_end() {
    unsafe {
        asm!("hlt");
    }
}
