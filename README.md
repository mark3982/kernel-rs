kernel-rs
=========

_This code may make rather large changes from time to time because I am still ironing out the best way to
do things as I move along!_

Ok, you are likely wondering what in the world this is, right? Well, I decided to use the language Rust
to make a kernel, but not just any ordinary kernel. I wanted to deviate from the norm by a few factors:

* minimal and easy tool dependancies
* minimal kernel footprint
* easy cross-compiling
* explore Rust

To accomplish minimal tool dependancies I try to either use or force Rust to do everything that it can do
and if that fails then I try to use the least painful tool to get the job done. If a tool requires more
dependancies and a headache then I avoid it because my motto is minimal and easy. I know you want to play
with the kernel and I do not want you to quit before you actually get started so my goal is to make it easy.

For minimal kernel footprint I will do as much as possible to prevent unnessary increase in image size because
this makes it more attractive for lower memory machines but still makes it usable for large memory machines. At
the moment I use very minimal Rust.

I wanted it to be easy to cross-compile to a target architecture that is different than the host you are compiling
the kernel on. This is a challenge because some tools, if used, must be cross-compiled. I currently use binutils for
working with the object files and archives. It would be nice to not have to use binutils but at the moment I am
unable to find a nice way to make Rust do all the work!

Also, by going with a minimal setup I feel it is nice in that it really lets you dig into Rust from the inside out and learn
some of the internal mechanisms. As you progress you will undoubtly have to implement more things that make your
code look more like Rust code.
