use std::env;
use cbindgen;

fn main(){



    cbindgen::generate("../ptolemy")
        .expect("!!!New bindings were NOT generated!!!")
        .write_to_file("headers/ptolemy.h");

}