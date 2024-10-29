use std::{
    fs::{self, create_dir, File},
    io::{self, Write},
    path::Path,
};

pub fn add_obc(id: u32) -> io::Result<()> {
    create_dir(format!("obc{id}"))?;
    create_dir(format!("obc{id}/entry"))?;
    create_dir(format!("obc{id}/lib"))?;
    File::create(format!("obc{id}/ports.hpp"))?;

    let mut port_file = File::create(format!("obc{id}/ports.hpp"))?;
    port_file.write_all(
        b"// not to be touched by user\n// will be regenerated to ensure correctness on each build",
    )?;
    File::create(format!("tasks.conf"))?;
    Ok(())
}

pub fn write_input_port(port_name: String) -> io::Result<()> {
    let input_port_snippet = include_str!("../cpp_snippets/input_port.cpp");

    let mut file = fs::OpenOptions::new()
        .create(false)
        .append(true)
        .open("port.hpp")?;

    file.write(format!("\n{}\n", input_port_snippet.replace("NAME", &port_name)).as_bytes())?;

    Ok(())
}

pub fn write_output_port(port_name: String) -> io::Result<()> {
    let output_port_snippet = include_str!("../cpp_snippets/output_port.cpp");

    let mut file = fs::OpenOptions::new()
        .create(false)
        .append(true)
        .open("port.hpp")?;

    file.write(format!("\n{}\n", output_port_snippet.replace("NAME", &port_name)).as_bytes())?;

    Ok(())
}

pub fn write_sensor(sensor_name: String) -> io::Result<()> {
    let sensor_snippet = include_str!("../cpp_snippets/sensor.cpp");

    let mut file = fs::OpenOptions::new()
        .create(false)
        .append(true)
        .open("port.hpp")?;

    file.write(format!("\n{}\n", sensor_name.replace("NAME", &sensor_name)).as_bytes())?;

    Ok(())
}

pub fn create_project(project_name: &str) -> std::io::Result<()> {
    let path_dir = Path::new(project_name);
    fs::create_dir(path_dir)?;
    File::create(path_dir.join("sensors.json"))?;
    Ok(())
}
