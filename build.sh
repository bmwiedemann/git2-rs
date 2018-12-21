#!/bin/sh
deps=`pwd`/target/release/deps
if ! [ -d $deps ] ; then # prepare once:
    cargo build --release --verbose
fi
opt="-C opt-level=2"
rustc src/lib.rs --crate-type lib $opt --emit=asm \
-A dead_code -A missing_docs -A unused_imports -A unused \
-L dependency=$deps \
--extern libc=$(echo $deps/liblibc-*.rlib) \
--extern libgit2_sys=$(echo $deps/liblibgit2_sys-*.rlib) \

md5sum lib.s
