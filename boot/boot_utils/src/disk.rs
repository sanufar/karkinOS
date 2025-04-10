// boot/boot_utils/src/disk.rs

use core::arch::asm;
use core::mem::size_of;

const SECTOR_SIZE: u16 = 512;

#[repr(C, packed)]
struct DiskAddressPacket {
    packet_size: u8,
    zero: u8,
    sectors: u16,
    transfer_offset: u16,
    transfer_segment: u16,
    lba: u64,
}

impl DiskAddressPacket {
    fn new(sectors: u16, transfer_offset: u16, lba: u64) -> Self {
        Self {
            packet_size: size_of::<DiskAddressPacket>() as u8,
            zero: 0,
            sectors,
            transfer_offset,
            transfer_segment: 0x0000,
            lba,
        }
    }
}

pub struct DiskReader {
    offset_target: u16, // Default memory offset for reads (used as fixed buffer in copy)
    lba: u64, // Starting LBA for the next read operation
}

impl DiskReader {
    pub fn new(offset_target: u16, lba: u64) -> Self {
        Self { offset_target, lba }
    }

    /// Reads multiple sectors contiguously into memory starting at `offset_target`.
    /// Attempts to read all requested sectors in a single BIOS call.
    /// (This function was fixed previously, used by boot_init)
    pub fn read_sectors(&mut self, sectors_to_read: u16) -> bool {
        if sectors_to_read == 0 {
            return true;
        }
        let dap = DiskAddressPacket::new(sectors_to_read, self.offset_target, self.lba);
        let dap_address = &dap as *const DiskAddressPacket;
        let success: u16;
        unsafe {
            asm!(
                "push si",
                "mov si, {dap_ptr:x}",
                "mov ah, 0x42",
                "mov dl, 0x80",
                "int 0x13",
                "pop si",
                "jc 2f",
                "mov {success_out:x}, 1",
                "jmp 3f",
                "2:",
                "mov {success_out:x}, 0",
                "3:",
                dap_ptr = in(reg) dap_address as u16,
                success_out = out(reg) success,
                options(nostack, preserves_flags)
            );
        }
        if success != 0 {
            let bytes_read = sectors_to_read.saturating_mul(SECTOR_SIZE);
            self.offset_target = self.offset_target.saturating_add(bytes_read);
            self.lba += sectors_to_read as u64;
            return true;
        } else {
            return false;
        }
    }

    /// Reads sectors one by one into a fixed buffer (`self.offset_target`)
    /// and copies them to a potentially different memory location (`target`).
    /// (This function is used by the bootloader to load the kernel)
    pub fn read_and_copy_sectors(&mut self, sectors: u16, target: u32) -> bool {
        let mut remaining_sectors: u16 = sectors;
        let mut current_target: u32 = target; // Destination address for copy

        // Use a fixed temporary buffer for reading sectors one by one.
        // This uses the offset the DiskReader was initialized with (e.g., KERNEL_BUFFER).
        let read_buffer_offset: u16 = self.offset_target;

        while remaining_sectors > 0 {
            // Read 1 sector from current LBA into the *fixed* read_buffer_offset
            let dap = DiskAddressPacket::new(1, read_buffer_offset, self.lba);
            let dap_address = &dap as *const DiskAddressPacket;
            let success: u16;
            unsafe {
                asm!(
                    "push si",
                    "mov si, {dap_ptr:x}",
                    "mov ah, 0x42", // Extended Read
                    "mov dl, 0x80", // Drive 0
                    "int 0x13",     // Call BIOS
                    "pop si",
                    "jc 2f",        // Jump on error (Carry Flag set)
                    "mov {success_out:x}, 1", // Success path
                    "jmp 3f",
                    "2:",           // Error path
                    "mov {success_out:x}, 0",
                    "3:",           // End
                    dap_ptr = in(reg) dap_address as u16,
                    success_out = out(reg) success,
                    // Clobbers: ax, flags (BIOS call)
                    options(nostack, preserves_flags)
                );
            }

            if success == 0 {
                // Optional: Add a print here for debugging disk errors
                // boot_utils::println!("[!] Disk read error at LBA {}", self.lba);
                return false; // Error occurred during read
            }

            // Copy the sector data from the fixed read buffer (DS:read_buffer_offset)
            // to the current target address (ES:EDI or flat 32-bit).
            // Use rep movsd for 32-bit copy, assuming Unreal Mode allows flat addressing.
            unsafe {
                asm!(
                    "cld",                          // Clear direction flag (copy forwards ESI -> EDI)
                    "mov ecx, {count:e}",           // Count: Number of dwords (512 / 4 = 128)
                    // Source: Assumes DS base is 0. ESI holds 32-bit offset.
                    "mov esi, {src_addr:e}",
                    // Destination: Assumes ES base allows writing to target. EDI holds 32-bit offset.
                    "mov edi, {dst_addr:e}",
                    "rep movsd",                    // Repeat copy dword (DS:[ESI] -> ES:[EDI])
                    count = in(reg) (SECTOR_SIZE / 4) as u32, // 128 dwords
                    src_addr = in(reg) read_buffer_offset as u32, // Read buffer addr
                    dst_addr = in(reg) current_target, // Destination addr
                    // Clobbers: ecx, esi, edi, flags. ES must be valid for writes.
                    options(nostack)
                );
            }

            // --- Correction Applied ---
            // DO NOT advance self.offset_target here. It should remain fixed.
            // Advance LBA and the *destination* memory pointer for the next sector.
            self.lba += 1;
            current_target = current_target.saturating_add(SECTOR_SIZE as u32);
            remaining_sectors -= 1;
        }
        true // All sectors read and copied successfully
    }
}
