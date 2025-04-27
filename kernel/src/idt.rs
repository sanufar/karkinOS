#![allow(dead_code)]

use crate::gdt::{PrivilegeLevel, SegmentSelector};
use core::marker::PhantomData;

#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: IDTEntry<HandlerFunc>,
    pub debug_exception: IDTEntry<HandlerFunc>,
    pub nonmaskable_interrupt: IDTEntry<HandlerFunc>,
    // this one's a trap
    pub breakpoint: IDTEntry<HandlerFunc>,
    pub overflow: IDTEntry<HandlerFunc>,
    pub bound_range_exceeded: IDTEntry<HandlerFunc>,
    pub invalid_opcode: IDTEntry<HandlerFunc>,
    pub device_not_available: IDTEntry<HandlerFunc>,
    pub double_fault: IDTEntry<HandlerFuncWithErrorCode>,
    pub coprocessor_segment_overrun: IDTEntry<HandlerFunc>,
    pub invalid_tss: IDTEntry<HandlerFuncWithErrorCode>,
    pub segment_not_present: IDTEntry<HandlerFuncWithErrorCode>,
    pub stack_segment_fault: IDTEntry<HandlerFuncWithErrorCode>,
    pub general_protection_fault: IDTEntry<HandlerFuncWithErrorCode>,
    pub page_fault: IDTEntry<HandlerFuncWithErrorCode>,
    intel_reserved: IDTEntry<HandlerFunc>,
    pub x87_float_error: IDTEntry<HandlerFunc>,
    pub alignment_check: IDTEntry<HandlerFuncWithErrorCode>,
    pub machine_check: IDTEntry<HandlerFunc>,
    pub simd_float_exception: IDTEntry<HandlerFunc>,
    pub virtualization_exception: IDTEntry<HandlerFunc>,
    pub control_protection_exception: IDTEntry<HandlerFuncWithErrorCode>,
    reserved_for_future: [IDTEntry<HandlerFunc>; 8],
    pub security_exception: IDTEntry<HandlerFuncWithErrorCode>,
    reserved_2: IDTEntry<HandlerFunc>,
    interrupts: [IDTEntry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    pub fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_error: IDTEntry::missing(),
            debug_exception: IDTEntry::missing(),
            nonmaskable_interrupt: IDTEntry::missing(),
            breakpoint: IDTEntry::missing(),
            overflow: IDTEntry::missing(),
            bound_range_exceeded: IDTEntry::missing(),
            invalid_opcode: IDTEntry::missing(),
            device_not_available: IDTEntry::missing(),
            double_fault: IDTEntry::missing(),
            coprocessor_segment_overrun: IDTEntry::missing(),
            invalid_tss: IDTEntry::missing(),
            segment_not_present: IDTEntry::missing(),
            stack_segment_fault: IDTEntry::missing(),
            general_protection_fault: IDTEntry::missing(),
            page_fault: IDTEntry::missing(),
            intel_reserved: IDTEntry::missing(),
            x87_float_error: IDTEntry::missing(),
            alignment_check: IDTEntry::missing(),
            machine_check: IDTEntry::missing(),
            simd_float_exception: IDTEntry::missing(),
            virtualization_exception: IDTEntry::missing(),
            control_protection_exception: IDTEntry::missing(),
            reserved_for_future: [IDTEntry::missing(); 8],
            security_exception: IDTEntry::missing(),
            reserved_2: IDTEntry::missing(),
            interrupts: [IDTEntry::missing(); 256 - 32],
        }
    }

    pub fn load(&'static self) {
        use core::arch::asm;
        use core::mem::size_of;

        let idt_ptr = IDTPointer {
            limit: (size_of::<InterruptDescriptorTable>() - 1) as u16,
            offset: self as *const _ as u64,
        };

        unsafe {
            asm!("lidt [{}]", in(reg) &idt_ptr);
        }
    }
}

#[repr(C, packed)]
struct IDTPointer {
    limit: u16,
    offset: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDTEntry<F> {
    fn_pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    fn_pointer_middle: u16,
    fn_pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerFuncWithErrorCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);

impl<F> IDTEntry<F> {
    pub const fn missing() -> Self {
        IDTEntry {
            fn_pointer_low: 0,
            gdt_selector: 0,
            options: EntryOptions::minimal(),
            fn_pointer_middle: 0,
            fn_pointer_high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }

    fn set_handler_addr(&mut self, addr: u64) {
        self.fn_pointer_low = addr as u16;
        self.fn_pointer_middle = (addr >> 16) as u16;
        self.fn_pointer_high = (addr >> 32) as u32;

        self.gdt_selector = SegmentSelector::new(1, PrivilegeLevel::Ring0).value();

        self.options.set_present(true);
    }
}

macro_rules! impl_set_handler_fn {
    ($h:ty) => {
        impl IDTEntry<$h> {
            #[inline]
            pub fn set_handler_fn(&mut self, handler: $h) {
                self.set_handler_addr(handler as u64)
            }
        }
    };
}

impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrorCode);

#[repr(transparent)]
#[derive(Clone, Copy)]
struct EntryOptions(u16);

impl EntryOptions {
    const IST_SHIFT: u8 = 0;
    const IST_MASK: u16 = 0b111 << Self::IST_SHIFT; // Bits 0-2

    const TYPE_SHIFT: u8 = 8;
    const TYPE_MASK: u16 = 0b1111 << Self::TYPE_SHIFT; // Bits 8-11

    const DPL_SHIFT: u8 = 13;
    const DPL_MASK: u16 = 0b11 << Self::DPL_SHIFT; // Bits 13-14

    const PRESENT_SHIFT: u8 = 15;
    const PRESENT_MASK: u16 = 1 << Self::PRESENT_SHIFT; // Bit 15

    const fn minimal() -> Self {
        EntryOptions(Self::PRESENT_MASK | (0b1110 << Self::TYPE_SHIFT)) // P=1, DPL=0, S=0, Type=Interrupt
    }

    fn set_type(&mut self, gtype: GateType) -> &mut Self {
        self.0 &= !(Self::TYPE_MASK | (1 << 12));

        let type_val = match gtype {
            GateType::Interrupt => 0b1110, // 0xE
            GateType::Trap => 0b1111,      // 0xF
        };
        self.0 |= type_val << Self::TYPE_SHIFT;
        self
    }

    pub fn set_privilege_level(&mut self, privilege: PrivilegeLevel) -> &mut Self {
        self.0 &= !Self::DPL_MASK;
        self.0 |= (privilege as u16) << Self::DPL_SHIFT;
        self
    }

    fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            self.0 |= Self::PRESENT_MASK; // Set bit 15
        } else {
            self.0 &= !Self::PRESENT_MASK; // Clear bit 15
        }
        self
    }

    fn set_ist(&mut self, index: u16) -> &mut Self {
        assert!(index <= 7, "IST index must be between 0 and 7");
        self.0 &= !Self::IST_MASK;
        self.0 |= (index as u16) << Self::IST_SHIFT;
        self
    }
}

enum GateType {
    Interrupt,
    Trap,
}

#[repr(C)]
pub struct InterruptStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}
