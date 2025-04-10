use core::{arch::asm, option};

use crate::gdt::GDT;
use boot_utils::println;

/// Approach:
/// 1. Check if A20 is enabled, by writing a value to an odd megabyte address (i.e. 0x100000) and reading back a value from the same address with bit 20 cleared (i.e. 0x000000 for this example.) If the values are not equal (i.e. A20 is enabled) return from the function and don't do anything. Else, continue with step 2.
///
/// 2. Enable A20 using BIOS INT 0x15 function 0x2401. It will return zero on success. If it succeeds, jump to step 4. If the function is not supported or failed, continue with step 3.
///
/// 3. Enable A20 using the PS/2 controller method. This is described in the Wiki article about A20.
///
/// 4. Check if A20 is enabled, with the same technique in step 1. If it is enabled, return from the function. Else, continue with step 5.
///
/// 5. Enable A20 using the FAST method. Read a byte from I/O port 0x92, set bit 1, clear bit 0 and write it back to the same I/O port. A small delay (i.e. NOP or PAUSE in a loop) is useful here.
///
/// 6. Check if A20 is enabled again, with the same technique in step 1. If it is enabled, return from the function. Else, display an error message, as there is apparently no way to enable A20 on this machine.

pub fn enable_a20() -> Result<(), &'static str> {
    if a20_check_odd_mb() {
        return Ok(());
    }

    if enable_a20_bios().is_err() {
        println!("BIOS A20 failed or unsupported, trying KBC...");
        enable_a20_keyboard_controller();
    }


    if a20_check_odd_mb() {
        return Ok(());
    }

    println!("A20 still disabled after trying KBC. Trying Fast A20...");

    enable_a20_fast();

    if a20_check_odd_mb() {
        return Ok(());
    }

    return Err("Unable to enable A20.");
}

/// Enacts step 1 of approach.
/// Need to ret
fn a20_check_odd_mb() -> bool {
    let mut buffer_below_mb: u16 = 0;
    let mut buffer_above_mb: u16 = 0;

    // if a20 is enabled, result == 1, else == 0
    let mut result: u16 = 0;

    unsafe {
    // Save registers and disable interrupts as before
    asm!(
        "pushf",
        "push si",
        "push di",
        "push ds",
        "push es",
        "cli",
        options(preserves_flags),
    );

    // Set up segment registers and indexes
    asm!(
        "mov ax, 0x0000",
        "mov ds, ax",
        "mov si, 0x0500",
        options(nostack, nomem)
    );

    asm!(
        "not ax",
        "mov es, ax",
        "mov di, 0x0510",
        options(nostack, nomem)
    );

    // Save original buffer contents:
    asm!(
        "mov al, byte ptr ds:[si]",
        "mov byte ptr [{buf_below}], al",  // write to our saved variable
        "mov al, byte ptr es:[di]",
        "mov byte ptr [{buf_above}], al",
        buf_below = in(reg) &mut buffer_below_mb,
        buf_above = in(reg) &mut buffer_above_mb,
    );

    // Perform the test
    asm!(
        "mov ah, 1",
        "mov byte ptr ds:[si], 0",
        "mov byte ptr es:[di], 1",
        "mov al, byte ptr ds:[si]",
        "cmp al, byte ptr es:[di]",
        "setne al", // AL = 1 if values differ, else 0
        "mov byte ptr [{result}], al",
        result = in(reg) &mut result,
    );

    // Restore original memory contents:
    asm!(
        "mov al, byte ptr [{buf_below}]",
        "mov byte ptr ds:[si], al",      
        "mov al, byte ptr [{buf_above}]",
        "mov byte ptr es:[di], al",
        buf_below = in(reg) buffer_below_mb,
        buf_above = in(reg) buffer_above_mb,
    );

    // Restore registers and flags:
    asm!(
        "pop si",
        "pop di",
        "pop es",
        "pop ds",
        "popf",
        options(nostack, preserves_flags),
    );
}


    result == 1
}

/// Enacts step 2 -> tries to enable A20 line with BIOS functions
fn enable_a20_bios() -> Result<(), &'static str> {
    let mut a20_err: u16 = 0; // 0 if no error; 1 means either not supported or activation failed
    unsafe {
        asm!(
            // Query A20 support
            "mov ax, 2403h",
            "int 15h",
            "jb 3f",                // jump to error if CF is set
            "cmp ah, 0",
            "jnz 3f",               // jump if AH not 0 (error)

            // Query A20 status
            "mov ax, 2402h",
            "int 15h",
            "jb 3f",
            "cmp ah, 0",
            "jnz 3f",
            "cmp al, 1",            // if AL == 1 then A20 is already active
            "jz 2f",                // jump to label 2 if already activated

            // Activate A20
            "mov ax, 2401h",
            "int 15h",
            "jb 3f",
            "cmp ah, 0",
            "jnz 3f",

            "2:",                   // Label 2: A20 successfully activated or already active
            "jmp 4f",               // Jump past error code
            "3:",                   // Label 3: Error handling path
            "mov byte ptr [{err_ptr}], 1", // Set error flag to 1
            "4:",                   // Label 4: End of ASM block

            err_ptr = inout(reg) a20_err,
            options(nostack)
        );
    }

    if a20_err == 1 {
        Err("Enabling A20 with BIOS INT 15h failed or is not supported")
    } else {
        Ok(())
    }
}

fn enable_a20_keyboard_controller() {
    unsafe {
        asm!(
            "pushf",
            "cli",

            "call 2f", //wait_io1
            "mov al, 0xad",
            "out 0x64, al",

            "call 2f",
            "mov al, 0xd0",
            "out 0x64, al",

            "call 3f", // wait_io2
            "in al, 0x60",
            "push eax",

            "call 2f",
            "mov al, 0xd1",
            "out 0x64, al",

            "call 2f",
            "pop eax",
            "or al, 2",
            "out 0x60, al",

            "call 2f",
            "mov al, 0xae",
            "out 0x64, al",

            "call 2f",
            "jmp 4f",

            "2:",
            "in al, 0x64",
            "test al, 2",
            "jnz 2b",
            "ret",

            "3:",
            "in al, 0x64",
            "test al, 1",
            "jz 3b",
            "ret",

            "4:",
            "popf"
        );
    }
}

fn enable_a20_fast() {
    unsafe {
        asm!(
            "in al, 0x92",
            "test al, 2",
            "jnz 2f",
            "or al, 2",
            "and al, 0xfe",
            "out 0x92, al",
            "2:",
        )
    }
}

pub fn protected_mode() {
    unsafe {
        asm!("cli");
    }

    GDT.load();

    unsafe {
        asm!(
            "mov eax, cr0",
            "or eax, 1",
            "mov cr0, eax",
        );
        asm!(

            "ljmp $0x8, $2f",
            "2:",
            options(att_syntax)
        );

        asm!(
            ".code32",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov fs, ax",
            "mov gs, ax",
        );
    }
}

pub fn unreal_mode() {
    // Save the original real mode CS. This is the selector you want CS to have once you return.
    let orig_cs: u16;
    unsafe {
        asm!("mov {0:x}, cs", out(reg) orig_cs, options(nostack));
    }

    // Enter protected mode so that we can load flat segment descriptors.
    protected_mode();

    unsafe {
        // 1. Disable protected mode by clearing the PE bit in CR0,
        //    but do not reload DS/ES so that they remain flat.
        asm!(
            "mov eax, cr0",
            "and eax, 0xFFFFFFFE", // Clear bit 0.
            "mov cr0, eax",
            options(nostack)
        );

        // 2. Perform a far jump to restore the original (real mode) CS.
        //    This far jump (ljmp) is essential to update CS when leaving protected mode.
        asm!(
            "ljmp $0x8, $2f",
            "2:",
            options(att_syntax)
        );
    }
}
pub fn long_mode() {}
