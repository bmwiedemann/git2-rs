#!/bin/bash
opt="-C opt-level=2"
d=`pwd`
deps=$d/target/release/deps
extra=$opt
rustc --crate-name git2 src/lib.rs --color always --crate-type lib $extra --emit=asm  --cfg 'feature="curl"' --cfg 'feature="default"' --cfg 'feature="https"' --cfg 'feature="libgit2-sys"' --cfg 'feature="openssl-probe"' --cfg 'feature="openssl-sys"' --cfg 'feature="ssh"' --cfg 'feature="ssh_key_from_memory"' -C metadata=ecb936f583dc48f0 -C extra-filename=-ecb936f583dc48f0 --out-dir $deps -L dependency=$d/target/release/deps --extern bitflags=$deps/libbitflags-f8dd8e77f0ad428d.rlib --extern libc=$deps/liblibc-0b3d13cd53349b87.rlib --extern libgit2_sys=$deps/liblibgit2_sys-4e85a42dd8d5350f.rlib --extern log=$deps/liblog-b689ec162d1e17aa.rlib --extern openssl_probe=$deps/libopenssl_probe-cdf44cb1316539f6.rlib --extern openssl_sys=$deps/libopenssl_sys-7203dedf32281a1e.rlib --extern url=$deps/liburl-75cf5240a0d5625d.rlib

md5sum ./target/release/deps/git2-ecb936f583dc48f0.s
