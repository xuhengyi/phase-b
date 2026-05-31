OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80200000;

SECTIONS
{
  . = BASE_ADDRESS;
  skernel = .;

  .text : {
    *(.text.entry)
    *(.text.trap)
    *(.text.user)
    *(.text .text.*)
  }

  . = ALIGN(4K);
  .rodata : {
    *(.rodata .rodata.*)
  }

  . = ALIGN(4K);
  .data : {
    *(.sdata .sdata.*)
    *(.data .data.*)
  }

  . = ALIGN(4K);
  sbss = .;
  .bss : {
    *(.sbss .sbss.*)
    *(.bss .bss.*)
    *(COMMON)
  }
  . = ALIGN(4K);
  ebss = .;

  .stack (NOLOAD) : {
    *(.stack)
  }

  ekernel = .;

  /DISCARD/ : {
    *(.eh_frame)
  }
}
