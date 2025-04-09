ENTRY(_start)

SECTIONS {
	. = 0x500;
	_stack_start = .;

	. = 0x7c00;
	_stack_end = .;

	.boot : {
		*(.boot .boot.*)
	}

	.text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }
    
    .data :
    {
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.got .got.*)
    }

	. = 0x7c00 + 0x1b8;

	.diskid : 
	{
		BYTE(0Xfb) 
        BYTE(0X00) 
        BYTE(0Xb1)
        BYTE(0Xe5) 
	}

	reserved : 
	{
		BYTE(0X00)
		BYTE(0X00)
	}

	.first_table : 
	{
		BYTE(0X80) /* boot indicator flag */

		BYTE(0X20) /* Starting head */
		BYTE(0X21) /* Starting sector and upper bits of starting cylinder */
		BYTE(0X00) /* Starting cylinder */

		BYTE(0X00) /* Partition type */

		BYTE(0X41) /* Ending head */
		BYTE(0X01) /* Ending sector bits and upper bits of ending cylinder */
		BYTE(0X00) /* Ending cylinder */

		BYTE(0X00) /* Relative sector (i.e starting LBA val) */
		BYTE(0X08) 
		BYTE(0X00) 
		BYTE(0X00) /* END */ 

		BYTE(0X00) /* Total number of sectors in partition */
		BYTE(0X08) 
		BYTE(0X00) 
		BYTE(0X00) /* END */ 
	}

	.second_table :
	{
		BYTE(0X00) /* Boot indicator flag */

        BYTE(0X41) /* Starting head */
        BYTE(0X02) /* Starting sector and upper bits of starting cylindrer */
        BYTE(0X00) /* Starting cylinder */

        BYTE(0X06) /* Partition type */

        BYTE(0X8a) /* Ending head */
        BYTE(0X08) /* Ending sector bits and upper bits of ending cylinder */
        BYTE(0X82) /* Ending cylinder */

        BYTE(0X00) /* Relative sector (i.e starting LBA val) */
        BYTE(0X10) 
        BYTE(0X00)
        BYTE(0X00) /* END */

        BYTE(0X00) /* Total number of sectors in partition */
        BYTE(0Xf0) 
        BYTE(0X1f)
        BYTE(0X00) /* END */

	}

	. = 0x7c00 + 0x1fe;

    .magic_number :
    {
        SHORT(0xaa55)
    }

    _bootloader_start = .;
}
