use std::{fs::{create_dir, File}, io};

pub fn add_obc(id: u32) -> io::Result<()> {
    create_dir(format!("obc{id}"))?;
    create_dir(format!("obc{id}/entry"))?;
    create_dir(format!("obc{id}/lib"))?;
    File::create(format!("obc{id}/ports.hpp"))?;
    File::create(format!("tasks.conf"))?;
    Ok(())
}