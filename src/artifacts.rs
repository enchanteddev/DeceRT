use std::{fmt::format, fs::create_dir_all, path::PathBuf, process::Command};

pub fn compile_entry_cpp(obc_id: u32) -> std::io::Result<()> {
    println!("Compiling entry cpp for obc{obc_id}\n\n\n");

    let path_to_obc = PathBuf::from(format!("./obc{obc_id}"));
    let dist_folder = path_to_obc.join("dist/");
    create_dir_all(&dist_folder)?;

    // let entry = path_to_obc.join("entry");
    // let fill = entry.read_dir().unwrap().map(|f| f.unwrap().path());
    // for f in fill {
    //     println!("{:?}", f);
    // }

    cc::Build::new()
        .target("x86_64-unknown-linux-gnu")
        .opt_level(2)
        .host("x86_64-unknown-linux-gnu")
        .out_dir(&dist_folder)
        .cpp(true)
        .file(path_to_obc.join("entry.cpp"))
        .file(path_to_obc.join("ports.cpp"))
        .files(
            path_to_obc
                .join("entry")
                .read_dir()
                .unwrap()
                .map(|f| f.unwrap().path()),
        )
        .files(
            path_to_obc
                .join("lib")
                .read_dir()
                .unwrap()
                .map(|f| f.unwrap().path()),
        )
        .compile(&format!("obc{obc_id}"));

    let refresh_command = Command::new("rm")
        .arg(dist_folder.join(format!("libobc{obc_id}.a")))
        .arg(dist_folder.join(format!("obc{obc_id}.o")))
        .output();

    println!("Removed previosly generated files: {:?}", refresh_command);

    let ld_output = Command::new("ld")
        .arg("-r")
        .arg("-o")
        .arg(dist_folder.join(format!("obc{obc_id}.o")))
        .args(dist_folder.read_dir().unwrap().filter_map(|object_file| {
            let fp = object_file.ok()?.path();
            if fp.extension()? == "o" {
                fp.to_str().map(|s| s.to_string())
            } else {
                None
            }
        }))
        .output();

    println!("Linked object files: {:?}", ld_output);

    let clean_output = Command::new("rm")
        .args(dist_folder.read_dir().unwrap().filter_map(|object_file| {
            let fp = object_file.ok()?.path();
            if fp.file_name()?.to_str()?.to_string() != format!("obc{obc_id}.o")
                && fp.extension()? == "o"
            {
                fp.to_str().map(|s| s.to_string())
            } else {
                None
            }
        }))
        .output();

    println!("Clean remaining object files: {:?}", clean_output);

    println!("Compilation successful for obc{obc_id}\n\n\n");

    Ok(())
}
