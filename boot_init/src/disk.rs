use core::mem::size_of;
use core::arch::asm;
use crate::print;

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
    offset_target: u16, // basically, where in memory to load in the data from disk. corresponds to
    // the transfer_offset.
    lba: u64,
}

impl DiskReader {
    pub fn new(offset_target: u16, lba: u64) -> Self {
        Self { offset_target, lba }
    }

    fn read_sector(&self) {
        let dap = DiskAddressPacket::new(1, self.offset_target, self.lba);
        let dap_address = &dap as *const DiskAddressPacket;

        unsafe {
            asm!(
                "mov {backup_si:x}, si",      
                "mov si, {dap_ptr:x}",       
                "int 0x13",
                "jc bootload_error",
                "mov si, {backup_si:x}",    
                dap_ptr = in(reg) dap_address as u16,
                backup_si = out(reg) _,
                in("ax") 0x4200 as u16,
                in("dx") 0x0080 as u16,
            );
        }
    }

    pub fn read_sectors(&mut self, sectors: u16) {
        let mut remaining_sectors: u16 = sectors;

        while remaining_sectors > 0 {
            self.read_sector();

            self.offset_target += SECTOR_SIZE;
            self.lba += 1;

            remaining_sectors -= 1;
        }

    }
}
