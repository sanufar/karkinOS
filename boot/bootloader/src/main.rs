#![no_std]
#![no_main]

use core::{arch::asm, mem::size_of};
use core::panic::PanicInfo;

static GDT: GdtProtectedMode = GdtProtectedMode::new();

#[repr(C)]
pub struct GdtProtectedMode {
    zero: u64,
    code: u64,
    data: u64,
}

impl GdtProtectedMode {
    /// Set up a flat GDT with a zero descriptor, a code descriptor, and a data descriptor.
    const fn new() -> Self {
        let limit = {
            let limit_low = 0xffff;
            let limit_high = 0xf << 48; // high 4 bits in bits 48-51
            limit_high | limit_low
        };
        let access_common = {
            let present = 1 << 47;
            let user_segment = 1 << 44;
            let read_write = 1 << 41;
            present | user_segment | read_write
        };
        let protected_mode_flag = 1 << 54;
        let granularity = 1 << 55;
        let base_flags = protected_mode_flag | granularity | access_common | limit;
        let executable = 1 << 43;
        Self {
            zero: 0,
            code: base_flags | executable,
            data: base_flags,
        }
    }

    /// Atomically disable interrupts and load the GDT.
    pub fn clear_interrupts_and_load(&'static self) {
        let pointer = GdtPointer {
            base: self as *const _,
            limit: (3 * size_of::<u64>() - 1) as u16,
        };
        unsafe {
            asm!(
                "cli",
                "lgdt [{}]",
                in(reg) &pointer,
                options(readonly, nostack, preserves_flags)
            );
        }
    }
}

#[repr(C, packed(2))]
pub struct GdtPointer {
    pub limit: u16,
    pub base: *const GdtProtectedMode,
}

fn set_protected_mode_bit() -> u32 {
    // Read CR0, set bit 0 to enable protected mode, then write it back.
    let mut cr0: u32;
    unsafe {
        asm!(
            "mov {:e}, cr0",
            out(reg) cr0,
            options(nomem, nostack, preserves_flags)
        );
    }
    let new_cr0 = cr0 | 1;
    write_cr0(new_cr0);
    cr0
}

fn write_cr0(val: u32) {
    unsafe {
        asm!(
            "mov cr0, {:e}",
            in(reg) val,
            options(nostack, preserves_flags)
        );
    }
}

/// Transition to protected mode using the enhanced approach.
pub fn protected_mode() {
    // Atomically disable interrupts and load the GDT.
    GDT.clear_interrupts_and_load();

    // Set the protected mode bit in CR0.
    let _old_cr0 = set_protected_mode_bit();

    unsafe {
        asm!(
            // Align the stack (for example, to a 256-byte boundary)
            "and esp, 0xffffff00",
            // Far jump to selector 0x8 (code segment) at label 1 to flush the prefetch queue
        );
            //
        asm!(
            "ljmp $0x8, $2f",
            "2:",
            options(att_syntax),
    );

        asm!(

            // Switch to 32-bit code and reload segment registers using 0x10 (data segment)
            ".code32",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov fs, ax",
            "mov gs, ax",
            options(nostack)
        );
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    boot_utils::println!("PANIC! Info: {}", info);

    loop {}
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {

    unsafe {
        asm!("mov sp, 0x7C00");
    }

    //unsafe { asm!("cli"); }

    boot_utils::println!("[*] Starting Stage 2 of bootloader...");

    //boot_utils::println!("[*] Enabling the A20 line...");

    //match modes::enable_a20() {
    //    Ok(()) => boot_utils::println!("[*] Successfully enabled A20 line!"),
    //    Err(err) => boot_utils::println!("[!] ERROR: {}", err),
    //}

    //boot_utils::println!("[*] Entering unreal mode...");

    //unreal_mode();

    //boot_utils::println!("[*] Loading kernel...");

    //let mut disk_reader = DiskReader::new(KERNEL_BUFFER, KERNEL_LBA);
    //
    //if !disk_reader.read_and_copy_sectors(KERNEL_SIZE, KERNEL_TARGET) {
    //    boot_utils::println!("[!] ERROR: Failed to load kernel into target.");
    //};

    boot_utils::println!("[*] Entering protected mode...");

    protected_mode();


    //boot_utils::println!("[*] Entering protected mode...");

    loop {};
}
