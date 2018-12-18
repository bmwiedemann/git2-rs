#!/bin/sh
deps=`pwd`/target/release/deps
if ! [ -d $deps ] ; then # prepare once:
    cargo build --release --verbose
fi
opt="-C opt-level=2"
rustc src/lib.rs --crate-type lib $opt --emit=asm -L dependency=$deps --extern bitflags=$deps/libbitflags-f8dd8e77f0ad428d.rlib --extern libc=$deps/liblibc-0b3d13cd53349b87.rlib --extern libgit2_sys=$deps/liblibgit2_sys-4e85a42dd8d5350f.rlib --extern log=$deps/liblog-b689ec162d1e17aa.rlib --extern url=$deps/liburl-75cf5240a0d5625d.rlib
md5sum lib.s
