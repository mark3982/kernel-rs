ENTRY (_start)

SECTIONS
{	
    . = 0x100000;
	_BOI = .;
    .blob : { 
    			*(.boot*)
    			*(.rodata*)
    			*(.data*) 
				*(.text*) 
				*(.bss*)
	}
	_EOI = .;
}