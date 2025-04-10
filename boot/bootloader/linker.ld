ENTRY(_start)

SECTIONS {
	. = 0x7c00 + 512; 

	_bootloader_start = .;

	.start : {
		*(.start)
	}

	.text : {
		*(.text .text.*)
	}

	    .bss : {
        *(.bss .bss.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }

    .eh_frame : {
        *(.eh_frame .eh_frame.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr .eh_frame_hdr.*)
    }

    . = _bootloader_start + 0x8000 - 2;

    .end_marker :
    {
        SHORT(0xdead)
    }
}
