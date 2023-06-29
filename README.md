#Ptolemy

Ptolemy is the library for types shared between the 
Thoth server and Ceres client.

build.rs generates c bindings with cbindgen on every invokation of "cargo build"
These bindings cna be found in target/cbindgen/ptolemy.h


At the moment, cbindgen will not index structs or type definitions that are not
used by functions in the API. You can manually submit these for generation
in cbindgen.toml under export > include.

A potential feature to forcibly include unused structs can be found here: https://github.com/mozilla/cbindgen/issues/852