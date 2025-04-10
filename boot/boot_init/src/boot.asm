.section .boot, "awx"
.global _start
.code16

_start:
    # disable external interrupts
    cli

    # set data segments to zero
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov fs, ax
    mov gs, ax

    # set stack pointer to beginning of program, so it grows before the program
    # the stack grows downwards when you push, so putting the stack after the program would overwrite the program
    # rember that bios loads the program at 0x7c00 in memory, so everything before is empty (not sure about this)
    cld
    mov sp, 0x7c00

enable_a20:
    # enable A20-Line via IO-Port 92, might not work on all motherboards
    in al, 0x92
    test al, 2
    jnz enable_a20_after
    or al, 2
    and al, 0xFE
    out 0x92, al
enable_a20_after:

check_int13h_extensions:
    push 'y'    # error code
    mov ah, 0x41
    mov bx, 0x55aa
    # dl contains drive number
    int 0x13
    jnc .int13_pass
.int13_pass:
    pop ax      # pop error code again

rust:
    # push arguments
    push dx     # disk number
    call main

# spin to avoid running after the end of the program
spin:
    hlt
    jmp spin
