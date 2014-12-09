# kernel-rs

 Ok, you are likely wondering what in the world this is, right? Well, I decided
 to use the language Rust to make a kernel, but not just any ordinary kernel. I
 wanted to deviate from the norm by a few factors:

* minimal and easy tool dependancies
* minimal kernel footprint
* easy cross-compiling
* explore Rust

To accomplish minimal tool dependancies I try to either use or force Rust to do
everything that it can do and if that fails then I try to use the least painful
tool to get the job done. If a tool requires more dependancies and a headache
then I avoid it because my motto is minimal and easy. I know you want to play
with the kernel and I do not want you to quit before you actually get started so
my goal is to make it easy.

For minimal kernel footprint I will do as much as possible to prevent unnessary
increase in image size because this makes it more attractive for lower memory
machines but still makes it usable for large memory machines. At the moment I
use very minimal Rust.

I wanted it to be easy to cross-compile to a target architecture that is
different than the host you are compiling the kernel on. This is a challenge
because some tools, if used, must be cross-compiled. I currently use binutils
for working with the object files and archives. It would be nice to not have to
use binutils but at the moment I am unable to find a nice way to make Rust do all
the work!

Also, by going with a minimal setup I feel it is nice in that it really lets you
dig into Rust from the inside out and learn some of the internal mechanisms. As
you progress you will undoubtly have to implement more things that make your code
look more like Rust code.

## Targets

This section includes some helpful information on the different targets.

* ARM

 For ARM32 all symbol references by default are going to be generated as PC
 relative unless you force it to address something at an absolute address. So for
 the most simple kernel you are going to be fairly safe loading the image at any
 address including one different than it was linked for.

  * Booting

      For QEMU you can boot your image using an ELF or BIN format (see QEMU section).
      However, if you are using real hardware you may have options. The most basic
      option for smaller boards is flashing your image to the ROM where it is
      executed directly from when the board boots. In this case you will need to use
      a BIN format and have it directly flashed to the ROM and properly linked if 
      needed. You must also be aware that ROM memory will likely be read-only meaning 
      any mutable data in your image will need to be linked outside of ROM in RAM and
      also be initialized by copying it there or initialized in place at that memory
      address.

      _Some boards will have a loader built into a/the ROM which could possibly load
      your kernel in an ELF format like QEMU does but this is highly specific to your 
      board so you must consult the documentation for it._

* X86/X86-64

 For X86/X86-64 symbol references are generated absolute, unless you force the
 generation of position independant code if possible. So the kernel will need to
 be loaded at the address it is linked for using the default build.

  * Booting

        The boot process for X86/X86-64 is complicated from a bare metal stand
        point. It firstly involves the BIOS which loads from ROM and then chooses
        a boot device and uses a specific method for that device. For booting from
        floppy, harddisk, cd/dvd you will need to write a separate 16-bit loading
        stub. You can however use something such a GRUB which can load your kernel
        from an ELF format. QEMU only supports ELF32 which means you can not produce
        an ELF64 and have it properly loaded last time I checked. So if you target
        X86 you can produce an ELF and have it easily loaded by QEMU, however on
        real hardware you will need additional help in the form of GRUB, another
        loader, or your own boot loader code. There are also network boots to consider
        which actually load an image from over the network but they may also require
        the usage of 16-bit STUB code. Currently, I do not have a way to make Rust
        produce 16-bit code therefore I have no fast track to booting using these
        methods and the only easy method is to target X86 and use QEMU to load your
        ELF or second to that use GRUB. _I am hoping to come up with a nice method
        to handling these situations._

### QEMU

 QEMU can handle loading an ELF or BIN format. For an ELF format QEMU will
 attempt to load each section at the specified LMA address however the code will
 be linked for the VMA address. For a BIN format QEMU will load where it desires
 (is programmed to load) therefore your code must be position independant or be
 linked for the propery address.
