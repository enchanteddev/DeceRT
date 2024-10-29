use std::{fs::{self, create_dir, File}, io, path::Path};

pub fn add_obc(id: u32) -> io::Result<()> {
    create_dir(format!("obc{id}"))?;
    create_dir(format!("obc{id}/entry"))?;
    create_dir(format!("obc{id}/lib"))?;
    File::create(format!("obc{id}/ports.hpp"))?;
    File::create(format!("obc{id}/tasks.conf"))?;
    Ok(())
}


pub fn create_project(project_name: &str) -> std::io::Result<()> {
    let path_dir = Path::new(project_name);
    fs::create_dir(path_dir)?;
    File::create(path_dir.join("sensors.json"))?;
    Ok(())
}