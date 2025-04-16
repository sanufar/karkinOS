#![allow(dead_code)]

use core::arch::asm;
use core::mem::size_of;

// ```text
//                      GDT
//
//    offset   segment descriptor
//             ┌─────────────────────┐
//    0x00     │ Null Segment        │ ◄──┬── GDTR Base
//             ├─────────────────────┤    │
//    0x08     │ Kernel Code Segment │    │
//             ├─────────────────────┤    ├── GDTR Limit
//    0x10     │ Kernel Data Segment │    │
//             ├─────────────────────┤    │
//    0x18     │ User Code Segment   │    │
//             ├─────────────────────┤    │
//    0x20     │ User Data Segment   │ ◄──┘
//             └─────────────────────┘
//      ▲
//      │
//      ├─────┐
//   ┌──┴─┐ ┌─┴──┐
//   │ CS │ │ DS │ ◄── segment registers
//   └────┘ └────┘
// ```
//
#[repr(C, packed)]
struct GlobalDescriptorTable<const MAX: usize = 8> {
    table: [SegmentDescriptor; MAX],
    next_free: usize,
}

/// impl add descriptor
/// add descriptor: -> segmentselector
impl<const MAX: usize> GlobalDescriptorTable<MAX> {
    pub fn new() -> Self {
        let table = [SegmentDescriptor::null(); MAX];
        GlobalDescriptorTable {
            table,
            next_free: 1,
        }
    }

    pub fn add_descriptor(
        &mut self,
        descriptor: SegmentDescriptor,
        privilege: PrivilegeLevel,
    ) -> SegmentSelector {
        if self.next_free >= MAX {
            panic!("GDT is full!");
        }

        let index = self.next_free;
        self.table[index] = descriptor;
        self.next_free += 1;
        SegmentSelector::new(index as u16, privilege)
    }

    pub fn load(&'static self) {
        let pointer = GDTPointer {
            limit: (size_of::<Self>() - 1) as u16,
            // Calculate base: address of the GDT structure
            base: self as *const _ as u64,
        };

        unsafe {
            asm!("lgdt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));
        }
    }
}

#[repr(C, packed)]
struct GDTPointer {
    limit: u16, // Size of the GDT minus 1
    base: u64,  // Linear address of the GDT
}

#[repr(transparent)]
pub struct SegmentSelector(u16);

impl SegmentSelector {
    pub fn new(index: u16, privilege: PrivilegeLevel) -> Self {
        SegmentSelector((index << 3) | (privilege as u16))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    limit_flags: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    fn null() -> Self {
        SegmentDescriptor {
            limit_low: 0,
            base_low: 0,
            base_mid: 0,
            access: 0,
            limit_flags: 0,
            base_high: 0,
        }
    }

    fn kernel_code() -> Self {
        SegmentDescriptor::null()
            .with_limit(0xFFFFF)
            .with_base(0)
            .with_access(access::KERNEL_CODE_ACCESS)
            .with_flags(flags::KERNEL_CODE_FLAGS)
    }

    pub fn kernel_data() -> Self {
        SegmentDescriptor::null()
            .with_limit(0xFFFFF)
            .with_base(0)
            .with_access(access::KERNEL_DATA_ACCESS)
            .with_flags(flags::DATA_FLAGS)
    }

    pub fn user_code() -> Self {
        SegmentDescriptor::null()
            .with_limit(0xFFFFF)
            .with_base(0)
            .with_access(access::USER_CODE_ACCESS)
            .with_flags(flags::USER_CODE_FLAGS)
    }

    pub fn user_data() -> Self {
        SegmentDescriptor::null()
            .with_limit(0xFFFFF)
            .with_base(0)
            .with_access(access::USER_DATA_ACCESS)
            .with_flags(flags::USER_DATA_FLAGS)
    }

    fn with_limit(mut self, limit: u32) -> Self {
        self.limit_low = (limit & 0xFFFF) as u16;
        self.limit_flags = (self.limit_flags & 0xF0) | (((limit >> 16) & 0x0F) as u8);
        self
    }

    fn with_base(mut self, base: u32) -> Self {
        self.base_low = (base & 0xFFFF) as u16;
        self.base_mid = ((base >> 16) & 0xFF) as u8;
        self.base_high = ((base >> 24) & 0xFF) as u8;
        self
    }

    fn with_access(mut self, access: u8) -> Self {
        self.access = access;
        self
    }

    fn with_flags(mut self, flags: u8) -> Self {
        self.limit_flags = (self.limit_flags & 0x0F) | (flags & 0xF0);
        self
    }
}

mod access {
    const PRESENT: u8 = 1 << 7;
    const DPL0: u8 = 0;
    const DPL1: u8 = 1 << 5;
    const DPL2: u8 = 2 << 5;
    const DPL3: u8 = 3 << 5;
    const DESCRIPTOR_SET: u8 = 1 << 4;
    const EXECUTABLE: u8 = 1 << 3;
    const DIRECTION_DOWN: u8 = 1 << 2;
    const CONFORMING: u8 = 1 << 2;
    const READABLE_WRITABLE: u8 = 1 << 1;
    const ACCESSED: u8 = 1;

    pub const KERNEL_CODE_ACCESS: u8 =
        PRESENT | DPL0 | DESCRIPTOR_SET | EXECUTABLE | READABLE_WRITABLE | ACCESSED;
    pub const KERNEL_DATA_ACCESS: u8 =
        PRESENT | DPL0 | DESCRIPTOR_SET | READABLE_WRITABLE | ACCESSED;
    pub const USER_CODE_ACCESS: u8 =
        PRESENT | DPL3 | DESCRIPTOR_SET | EXECUTABLE | READABLE_WRITABLE | ACCESSED;
    pub const USER_DATA_ACCESS: u8 = PRESENT | DPL3 | DESCRIPTOR_SET | READABLE_WRITABLE | ACCESSED;
}

mod flags {
    const GRANULARITY: u8 = 1 << 7;
    const DB_SIZE: u8 = 1 << 6;
    const LONG_MODE: u8 = 1 << 5;

    pub const KERNEL_CODE_FLAGS: u8 = GRANULARITY | LONG_MODE;
    pub const DATA_FLAGS: u8 = GRANULARITY;
    pub const USER_CODE_FLAGS: u8 = GRANULARITY | LONG_MODE;
    pub const USER_DATA_FLAGS: u8 = GRANULARITY;
}

#[repr(u16)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}
