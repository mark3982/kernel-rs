ENTRY (_start)

SECTIONS
{	
    //. = 0x10000;
	_BOI = .;
    .blob : { 
    			*(.data*) 
    			*(.rodata*) 
				*(.boot*)
				*(.text*) 
				*(.bss*)
	}
	_EOI = .;
}