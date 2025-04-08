#![allow(dead_code)]

use core::arch::asm;
use core::mem::size_of;

pub static GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

const DESCRIPTOR_SIZE: u16 = size_of::<SegmentDescriptor>() as u16;
const CODE_SEGMENT_INDEX: u16 = 1;
const DATA_SEGMENT_INDEX: u16 = 2;

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    null: SegmentDescriptor,
    code: SegmentDescriptor,
    data: SegmentDescriptor,
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        Self {
            null: SegmentDescriptor::null(),
            code: SegmentDescriptor::code_segment(),
            data: SegmentDescriptor::data_segment(),
        }
    }

    pub fn load(&self) {
        let descriptor = GDTDescriptor {
            size: (size_of::<Self>() as u16) - 1,
            base: self,
        };

        unsafe {
            asm!(
            "lgdt [{0}]",
            in(reg) &descriptor as *const _);
        }
    }

    pub fn code_segment_selector(&self) -> u16 {
        CODE_SEGMENT_INDEX * DESCRIPTOR_SIZE
    }

    pub fn data_segment_selector(&self) -> u16 {
        DATA_SEGMENT_INDEX * DESCRIPTOR_SIZE
    }
}

#[repr(C, packed)]
pub struct GDTDescriptor {
    size: u16,
    base: *const GlobalDescriptorTable,
}

pub struct SegmentDescriptor {
    segment_limit_low: u16,
    base_address_low: u16,
    base_address_middle: u8,
    access_byte: u8,
    segment_limit_high_flags: u8,
    base_address_high: u8,
}

impl SegmentDescriptor {
    const fn null() -> Self {
        Self {
            segment_limit_low: 0,
            base_address_low: 0,
            base_address_middle: 0,
            access_byte: 0,
            segment_limit_high_flags: 0,
            base_address_high: 0,
        }
    }

    pub const fn code_segment() -> Self {
        SegmentDescriptor::null()
            .with_base(0x00000)
            .with_limit(0xFFFFF)
            .with_access_byte(
                access::PRESENT
                    | access::DPL_RING0
                    | access::DESCRIPTOR_TYPE
                    | access::EXECUTABLE
                    | access::READABLE,
            )
            .with_flags(flags::GRANULARITY | flags::SIZE)
    }

    pub const fn data_segment() -> Self {
        SegmentDescriptor::null()
            .with_base(0x00000)
            .with_limit(0xFFFFF)
            .with_access_byte(
                access::PRESENT | access::DPL_RING0 | access::DESCRIPTOR_TYPE | access::WRITABLE,
            )
            .with_flags(flags::GRANULARITY | flags::SIZE)
    }

    const fn with_base(mut self, base: u32) -> Self {
        self.base_address_low = (base & 0xFFFF) as u16;
        self.base_address_middle = ((base >> 16) & 0xFF) as u8;
        self.base_address_high = ((base >> 24) & 0xFF) as u8;
        self
    }

    const fn with_limit(mut self, limit: u32) -> Self {
        self.segment_limit_low = (limit & 0xFFFF) as u16;
        self.segment_limit_high_flags =
            (self.segment_limit_high_flags & 0xF0) | ((limit >> 16) & 0x0F) as u8;
        self
    }

    const fn with_access_byte(mut self, access: u8) -> Self {
        self.access_byte = access;
        self
    }

    const fn with_flags(mut self, flags: u8) -> Self {
        self.segment_limit_high_flags = (self.segment_limit_high_flags & 0x0F) | (flags & 0xF0);
        self
    }
}

pub mod access {
    pub const PRESENT: u8 = 1 << 7;
    pub const DPL_RING0: u8 = 0 << 5;
    pub const DPL_RING1: u8 = 1 << 5;
    pub const DPL_RING2: u8 = 2 << 5;
    pub const DPL_RING3: u8 = 3 << 5;
    pub const DESCRIPTOR_TYPE: u8 = 1 << 4; // 1 for code/data segments
    pub const EXECUTABLE: u8 = 1 << 3;
    pub const DIRECTION_DOWN: u8 = 1 << 2; // For data segments
    pub const CONFORMING: u8 = 1 << 2; // For code segments
    pub const READABLE: u8 = 1 << 1; // For code segments
    pub const WRITABLE: u8 = 1 << 1; // For data segments
    pub const ACCESSED: u8 = 1 << 0;
}

pub mod flags {
    pub const GRANULARITY: u8 = 1 << 7; // 1 = limit in 4KB units
    pub const SIZE: u8 = 1 << 6; // 1 = 32-bit protected mode
    pub const LONG_MODE: u8 = 1 << 5; // 1 = 64-bit code segment
    pub const AVAILABLE: u8 = 1 << 4; // Available for system use
}
