use std::{collections::HashMap, fs::create_dir_all, path::{Path, PathBuf}, process::Command, sync::Arc};

pub fn compile_entry_cpp(obc_id: u32) -> std::io::Result<()> {
    println!("Compiling entry cpp for obc{obc_id}\n\n\n");

    let path_to_obc = PathBuf::from(format!("./obc{obc_id}"));
    let dist_folder = path_to_obc.join("dist/");
    create_dir_all(&dist_folder)?;

    let compilation_command = Command::new("g++")
        .arg("-O2") // optimisation level 2
        .arg("-c")
        .arg("-o")
        .arg(dist_folder.join(format!("obc{obc_id}.o")))
        .arg(path_to_obc.join("entry.cpp"))
        .arg(path_to_obc.join("ports.cpp"))
        .args(
            path_to_obc
                .join("entry")
                .read_dir()
                .unwrap()
                .map(|f| f.unwrap().path()),
        )
        .args(
            path_to_obc
                .join("lib")
                .read_dir()
                .unwrap()
                .map(|f| f.unwrap().path()),
        )
        .output();

    match compilation_command {
        Ok(x) => {
            println!("Compiled obc{obc_id} Successfully");
            println!("Compilation output: [{x:?}]");
        }
        Err(e) => {
            println!("Compiling obc{obc_id} Failed");
            println!("Error message: [{e:?}]");
        }
    }
    Ok(())
}

fn get_names_array(names: HashMap<Arc<str>, u64>) -> Vec<Arc<str>> {
    let mut id_names = names
        .into_iter()
        .map(|(name, id)| (id, name))
        .collect::<Vec<_>>();

    id_names.sort();
    id_names.into_iter().map(|(_, name)| name).collect()
}

pub fn compile_demo_rtos(
    sensor_names: HashMap<Arc<str>, u64>,
    port_names: HashMap<Arc<str>, u64>,
) -> std::io::Result<()> {
    let rtos_cpp_template = include_str!("../cpp_snippets/rtos.cpp");

    let rtos_cpp = rtos_cpp_template
        .replace(
            "{SENSOR_NAMES}",
            &format!("{{{}}}", &get_names_array(sensor_names).join(", ")),
        )
        .replace(
            "{PORT_NAMES}",
            &format!("{{{}}}", &get_names_array(port_names).join(", ")),
        );

    let temp_dir = std::env::temp_dir();
    let rtos_cpp_path = temp_dir.join("rtos.cpp");
    std::fs::write(&rtos_cpp_path, rtos_cpp)?;
    

    let rtos_dir = PathBuf::from("./rtos");
    create_dir_all(&rtos_dir)?;

    let compilation_command = Command::new("g++")
        .arg("-O2") // optimisation level 2
        .arg("-c")
        .arg("-o")
        .arg("./rtos.o")
        .arg(rtos_cpp_path)
        .output();

    match compilation_command {
        Ok(x) => {
            println!("Compiled demo rtos Successfully");
            println!("Compilation output: [{x:?}]");
        }
        Err(e) => {
            println!("Compiling demo rtos Failed");
            println!("Error message: [{e:?}]");
        }
    }
    Ok(())
}
