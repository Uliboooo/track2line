use crate::Error;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

/// get the path to the config file
fn get_config_path() -> Result<PathBuf, Error> {
    let home_path = home::home_dir()
        .ok_or(Error::SomeErr)?;
    let tmp_path = if cfg!(target_os = "windows") {
        home_path.join("AppData").join("Local").join("track2line")
    } else if cfg!(target_os = "macos") {
        home_path
            .join("Library")
            .join("Application Support")
            .join("track2line")
    } else {
        // linux
        home_path.join(".config").join("track2line")
    };
    if !tmp_path.exists() {
        fs::create_dir(&tmp_path).map_err(Error::IoErr)?;
    }
    Ok(tmp_path.join("config.toml"))
}

/// overwrite existing file
pub fn save(content: String, overwrite: bool) -> Result<(), Error> {
    let mut file = if overwrite {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(get_config_path()?)
            .map_err(Error::IoErr)?
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(get_config_path()?)
            .map_err(Error::IoErr)?
    };
    writeln!(file, "{}", content).map_err(Error::IoErr)?;
    Ok(())
}

pub fn load_config() -> Result<String, Error> {
    let path = get_config_path()?;
    if path.exists() {
        Ok(fs::read_to_string(path).map_err(Error::IoErr)?)
    } else {
        Err(Error::ConfigNotFound)
    }
}

#[test]
fn show_config_path() {
    println!("{:?}", get_config_path());
    let b = OpenOptions::new()
        .write(true)
        .truncate(false)
        .create(true)
        .open(get_config_path().unwrap())
        .map_err(Error::IoErr)
        .unwrap();
    println!("{:?}", b);
}
