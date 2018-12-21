#!/bin/sh
deps=`pwd`/target/release/deps
if ! [ -d $deps ] ; then # prepare once:
    cargo build --release --verbose
fi
opt="-C opt-level=2"
rustc src/lib.rs --crate-type lib $opt --emit=asm \
-A dead_code -A missing_docs -A unused_imports -A unused \
-L dependency=$deps \
--extern libc=$deps/liblibc-0b3d13cd53349b87.rlib \
--extern libgit2_sys=$deps/liblibgit2_sys-4e85a42dd8d5350f.rlib \

md5sum lib.s
