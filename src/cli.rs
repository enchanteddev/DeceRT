use std::{
    fs::{self, create_dir, File},
    io::{self, Write},
    path::Path,
};

fn write_input_port(port_name: &str, ports_hpp: &mut File) -> io::Result<()> {
    let input_port_snippet = include_str!("../cpp_snippets/input_port.cpp");
    ports_hpp
        .write(format!("\n{}\n", input_port_snippet.replace("NAME", &port_name)).as_bytes())?;

    Ok(())
}

fn write_output_port(port_name: &str, ports_hpp: &mut File) -> io::Result<()> {
    let output_port_snippet = include_str!("../cpp_snippets/output_port.cpp");
    ports_hpp
        .write(format!("\n{}\n", output_port_snippet.replace("NAME", &port_name)).as_bytes())?;

    Ok(())
}

fn write_sensor(sensor_name: &str, ports_hpp: &mut File) -> io::Result<()> {
    let sensor_snippet = include_str!("../cpp_snippets/sensor.cpp");
    ports_hpp.write(format!("\n{}\n", sensor_snippet.replace("NAME", &sensor_name)).as_bytes())?;

    Ok(())
}

pub fn create_project(project_name: &str) -> std::io::Result<()> {
    let path_dir = Path::new(project_name);
    fs::create_dir(path_dir)?;
    File::create(path_dir.join("sensors.json"))?;
    Ok(())
}

pub fn add_obc(id: u32) -> io::Result<()> {
    create_dir(format!("obc{id}"))?;
    create_dir(format!("obc{id}/entry"))?;
    create_dir(format!("obc{id}/lib"))?;
    File::create(format!("obc{id}/ports.hpp"))?;

    let mut port_file = File::create(format!("obc{id}/ports.hpp"))?;
    port_file.write(
        b"// not to be touched by user\n// will be regenerated to ensure correctness on each build",
    )?;
    File::create(format!("tasks.conf"))?;
    Ok(())
}

pub fn update_tasks() -> Result<(), String> {
    let conf = confparse::get_conf("tasks.conf")?;

    let mut ports_hpp = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("port.hpp")
        .map_err(|e| e.to_string())?;

    ports_hpp.write(
        b"// not to be touched by user\n// will be regenerated to ensure correctness on each build\n",
    ).map_err(|e| e.to_string())?;

    for inports in conf.inports {
        write_input_port(&inports, &mut ports_hpp).map_err(|e| e.to_string())?;
    }
    for outports in conf.outports {
        write_output_port(&outports, &mut ports_hpp).map_err(|e| e.to_string())?;
    }
    let task_snippet = include_str!("../cpp_snippets/task.cpp");

    for task in &conf.tasks {
        let mut file = match File::create_new(format!("/entry/{}.cpp", task.name)) {
            Ok(x) => x,
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e.to_string()),
        };

        let task_code = task_snippet
            .replace("TASKNAME", &task.name)
            .replace("ARGS", &task.args.join(", "));

        file.write(task_code.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    for file in Path::new("entry").read_dir().map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let filename = file.file_name();
        let filename = filename.to_str().ok_or("Filename is not valid UTF-8")?;
        if !conf.tasks.iter().any(|f| &*f.name == filename) {
            Err(format!("'{filename}' is not the name of any task."))?
        }
    }

    Ok(())
}
