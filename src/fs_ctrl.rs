use crate::Error;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

/// get the path to the config file
fn get_config_path() -> Result<PathBuf, Error> {
    Ok(home::home_dir()
        .ok_or(Error::SomeErr)?
        .join(".track2line_config")
        .join("config.toml"))
}

/// overwrite existing file
pub fn save(content: String) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_path()?)
        .map_err(Error::IoErr)?;
    writeln!(file, "{}", content).map_err(Error::IoErr)?;
    Ok(())
}

pub fn load() -> Result<String, Error> {
    let path = get_config_path()?;
    if path.exists() {
        Ok(fs::read_to_string(path).map_err(Error::IoErr)?)
    } else {
        Err(Error::ConfigNotFound)
    }
}
