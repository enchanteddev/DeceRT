use std::path::{Path, PathBuf};

pub fn compile_entry_cpp(obc_id: u32) {
    println!("Compiling entry cpp for obc{obc_id}");
    let path_to_obc = PathBuf::from(format!("./obc{obc_id}"));
    cc::Build::new()
        .cpp(true)
        .file(path_to_obc.join("entry.cpp"))
        .file(path_to_obc.join("ports.cpp"))
        .file("/home/krawat/coding/DeceRT/RTOS/rtos.cpp")
        .include(path_to_obc.join("entry"))
        .include(path_to_obc.join("lib"))
        .compile(&format!("obc{obc_id}"));
    println!("Compilation successful for obc{obc_id}");
}
