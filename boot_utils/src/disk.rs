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
    offset_target: u16,
    lba: u64,
}

impl DiskReader {
    pub fn new(offset_target: u16, lba: u64) -> Self {
        Self { offset_target, lba }
    }

    fn read_sector(&self) -> bool {
        // Return success/failure
        let dap = DiskAddressPacket::new(1, self.offset_target, self.lba);
        let dap_address = &dap as *const DiskAddressPacket;
        let success: u16 = 1;

        unsafe {
            asm!(
                "mov {backup_si:x}, si",
                "mov si, {dap_ptr:x}",
                "mov ah, 0x42",
                "mov dl, 0x80",
                "int 0x13",
                "jnc 2f",
                "mov {success:x}, 0", // Set success to false if carry is set
                "2:",
                "mov si, {backup_si:x}",

                dap_ptr = in(reg) dap_address as u16,
                backup_si = out(reg) _,
                success = inout(reg) 1 => _,  // Initialize to 1 (true)
            );
        }

        success != 0
    }

    pub fn read_sectors(&mut self, sectors: u16) -> bool {
        let mut remaining_sectors: u16 = sectors;
        while remaining_sectors > 0 {
            if !self.read_sector() {
                return false; // Error occurred
            }
            self.offset_target += SECTOR_SIZE;
            self.lba += 1;
            remaining_sectors -= 1;
        }
        true // All sectors read successfully
    }
}
