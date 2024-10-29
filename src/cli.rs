

use std::{fs::{self, File}, path::Path};

pub fn create_project(project_name: String) -> std::io::Result<()> {
    let path_dir = Path::new(project_name.as_str());
    fs::create_dir(path_dir)?;
    File::create(path_dir.join("sensors.json"))?;
    Ok(())
}