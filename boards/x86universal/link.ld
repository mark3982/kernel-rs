ENTRY (_start)

SECTIONS
{	
    . = 0x100000;
	_BOI = .;
    .blob : { 
    			*(.rodata*)
    			*(.data*) 
				*(.boot*)
				*(.text*) 
				*(.bss*)
	}
	_EOI = .;
}