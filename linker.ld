ENTRY(start)

SECTIONS
{
    . = 0x200000;
    
    .text : {
        *(.text .text.*)
    }
    
    .data : {
        *(.data .data.*)
        *(.rodata .rodata.*)
    }
    
    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    }
    
    /DISCARD/ : {
        *(.note.*)
        *(.comment)
    }
}
