use std::{
    collections::HashMap, env::set_current_dir, fs::create_dir_all, path::PathBuf,
    process::Command, sync::Arc,
};

pub fn compile_entry_cpp(obc_id: u32) -> std::io::Result<()> {
    // println!("Compiling entry cpp for obc{obc_id}");

    let path_to_obc = PathBuf::from(format!("./obc{obc_id}")).canonicalize()?;
    let dist_folder = path_to_obc.join("dist/").canonicalize()?;
    create_dir_all(&dist_folder)?;

    let curr_dir = std::env::current_dir()?;

    let temp_dir = std::env::temp_dir();
    set_current_dir(temp_dir)?;

    let compilation_command = Command::new("g++")
        .arg("-O2") // optimisation level 2
        .arg("-c")
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
                .filter(|f| {
                    f.as_ref()
                        .unwrap()
                        .path()
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        == "cpp"
                })
                .map(|f| f.unwrap().path()),
        )
        // .arg("-o")
        // .arg(dist_folder.join(format!("obc{obc_id}.o")))
        .output();

    match compilation_command {
        Ok(x) => {
            if x.status.success() {
                println!("Compiled obc{obc_id} files Successfully");
                // println!("Compilation output: [{x:?}]");
            } else {
                println!("Compiling obc{obc_id} files Failed");
                println!("Error message: [{x:?}]");
            }
        }
        Err(e) => {
            println!("Compiling obc{obc_id} files Failed");
            println!("Error message: [{e:?}]");
        }
    }

    let linking_command = Command::new("ld")
        .arg("-r")
        .arg("entry.o")
        .arg("ports.o")
        .args(path_to_obc.join("entry").read_dir().unwrap().map(|f| {
            f.unwrap()
                .path()
                .with_extension("o")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }))
        .args(
            path_to_obc
                .join("lib")
                .read_dir()
                .unwrap()
                .filter(|f| {
                    f.as_ref()
                        .unwrap()
                        .path()
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        == "cpp"
                })
                .map(|f| {
                    f.unwrap()
                        .path()
                        .with_extension("o")
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                }),
        )
        .arg("-o")
        .arg(dist_folder.join(format!("obc{obc_id}.o")))
        .output();

    set_current_dir(&curr_dir)?;

    match linking_command {
        Ok(x) => {
            if x.status.success() {
                println!("Compiled obc{obc_id} Successfully");
                // println!("Compilation output: [{x:?}]");
            } else {
                println!("Compiling obc{obc_id} Failed");
                println!("Error message: [{x:?}]");
            }
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
            &format!(
                "{{{}}}",
                &get_names_array(sensor_names)
                    .into_iter()
                    .map(|n| format!("\"{n}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
        .replace(
            "{PORT_NAMES}",
            &format!(
                "{{{}}}",
                &get_names_array(port_names)
                    .into_iter()
                    .map(|n| format!("\"{n}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );

    let temp_dir = std::env::temp_dir();
    let rtos_cpp_path = temp_dir.join("rtos.cpp");
    std::fs::write(&rtos_cpp_path, rtos_cpp)?;

    let rtos_dir = PathBuf::from("./rtos");
    create_dir_all(&rtos_dir)?;

    let compilation_command = Command::new("g++")
        .arg("-O2") // optimisation level 2
        .arg("-c")
        .arg(rtos_cpp_path)
        .arg("-o")
        .arg(rtos_dir.join("rtos.o"))
        .output();

    match compilation_command {
        Ok(x) => {
            if x.status.success() {
                println!("Compiled demo rtos Successfully");
                // println!("Compilation output: [{x:?}]");
            } else {
                println!("Compiling demo rtos files Failed");
                println!("Error message: [{x:?}]");
            }
        }
        Err(e) => {
            println!("Compiling demo rtos Failed");
            println!("Error message: [{e:?}]");
        }
    }
    Ok(())
}
