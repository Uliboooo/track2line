mod fs_ctrl;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{error, fmt, io};
use track2line_lib as t2l;

#[derive(Debug)]
enum Error {
    NoInput,
    IoErr(io::Error),
    Toml,
    SomeErr,
    Cancel,
    ConfigNotFound,
}
// いつかちゃんと実装する
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoInput => writeln!(f, "no input, pelse folder path."),
            Error::IoErr(error) => writeln!(f, "{}", error),
            Error::Toml => writeln!(f, "toml error"),
            Error::SomeErr => writeln!(f, "something error"),
            Error::Cancel => writeln!(f, "canceled."),
            Error::ConfigNotFound => writeln!(f, "config file not found."),
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// target folder path
    folder_path: Option<String>,

    /// change audio extension
    #[arg(short = 'a', long = "audio", help = "change audio file extension")]
    audio_extension: Option<String>,

    /// change text(lines) extension
    #[arg(short = 't', long = "text", help = "change text file extension")]
    txt_extension: Option<String>,

    #[arg(
        short = 's',
        long = "set_audio",
        help = "set audio extension",
        default_value_t = false
    )]
    set_mode: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    // extensions configs
    audio_extension: String,
    txt_extension: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            audio_extension: "wav".to_string(),
            txt_extension: "txt".to_string(),
        }
    }
}
impl Config {
    // fn new<S: AsRef<str>>(audio: S, txt: S) -> Self {
    //     Config {
    //         audio_extension: audio.as_ref().to_string(),
    //         txt_extension: txt.as_ref().to_string(),
    //     }
    // }

    /// if config file exists, load it, otherwise return default(wav, txt)
    fn load() -> Result<Self, Error> {
        match fs_ctrl::load_config() {
            Ok(v) => Ok(toml::from_str(v.as_str()).map_err(|_| Error::SomeErr)?),
            Err(e) => match e {
                Error::ConfigNotFound => Ok(Config::default()),
                _ => Err(e),
            },
        }
    }

    /// save config to file. when args are some(), save them to config.
    fn change<S: AsRef<str>>(&self, audio: Option<S>, text: Option<S>) -> Result<(), Error> {
        let mut new_config = Config::load()?;
        if let Some(a) = audio {
            new_config.audio_extension = a.as_ref().to_string()
        } else if let Some(t) = text {
            new_config.txt_extension = t.as_ref().to_string()
        }
        fs_ctrl::save(toml::to_string(&new_config).map_err(|_| Error::Toml)?, true)?;
        Ok(())
    }

    fn init(&self) -> Result<(), Error> {
        fs_ctrl::save(toml::to_string(&self).map_err(|_| Error::Toml)?, false)?;
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    // init config
    Config::default().init()?;
    
    let args = Args::parse();
    let config = Config::load()?;

    if args.set_mode {
        // set mode
        // .change() return Result<(), Error>
        return config.change(args.audio_extension, args.txt_extension);
    } else {
        // normal mode
        let dir = args.folder_path.ok_or(Error::NoInput)?;
        let audio_ext = args.audio_extension.unwrap_or(config.audio_extension);
        let line_ext = args.txt_extension.unwrap_or(config.txt_extension);

        let mut sets = t2l::PathSets::new(&dir, audio_ext, line_ext).unwrap();
        let check_list = sets.check().map_err(|_| Error::SomeErr)?;
        println!("{}", check_list);

        if get_input::yes_no("continue?(y(yes or enter)/n)") {
            sets.rename().map_err(|_| Error::SomeErr)?;
            // 👆で?しているので、成功したことを確認するためのメッセージを表示する
            println!("success. all file is renamed.");
        } else {
            return Err(Error::Cancel);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn config() {
        let con = Config::load();
        println!("{:?}", con);
    }

    #[test]
    fn a() {
        let a = track2line_lib::PathSets::new(
            "/Users/yuki/Develop/track2line/assets_for_test/assets",
            "wav",
            "txt",
        );
        println!("{:?}", a);
    }
    
    #[test]
    fn test_config_init() {
        Config::default().init().unwrap();
    }
}
