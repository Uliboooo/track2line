// mod fs_ctrl;

use clap::Parser;
use get_input::get_input;
use std::{error, fmt, path::PathBuf};
use track2line_lib::{self as t2l};

#[derive(Debug)]
enum Error {
    NoInput,
    // IoErr(io::Error),
    SomeErr,
    Cancel,
    T2L(t2l::Error),
    Config(t2l::config::Error),
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
            // Error::IoErr(error) => writeln!(f, "{}", error),
            Error::SomeErr => writeln!(f, "something error"),
            Error::Cancel => writeln!(f, "canceled."),
            Error::T2L(error) => writeln!(f, "{}", error),
            Error::Config(error) => writeln!(f, "{}", error),
        }
    }
}

#[derive(Parser, Debug)]
#[command(verbatim_doc_comment)]
struct Args {
    #[arg(
        short = 's',
        long = "set_mode",
        help = "set audio extension\n`track2line -s -a mp3(optional) -t rtf(optional)`",
        default_value_t = false
    )]
    set_mode: bool,

    /// a path of target folder
    folder_path: Option<String>,

    /// change audio extension
    #[arg(
        short = 'a',
        long = "audio",
        help = "change audio file extension. if use set-mode(-s), change config."
    )]
    audio_extension: Option<String>,

    /// change text(lines) extension
    #[arg(
        short = 't',
        long = "text",
        help = "change text file extension.  if use set-mode(-s), change config."
    )]
    txt_extension: Option<String>,
}

fn get_user_input() -> Result<String, Error> {
    let input = get_input("target folder>");
    if input.is_empty() {
        Err(Error::NoInput)
    } else {
        Ok(input)
    }
}

fn main() -> Result<(), Error> {
    let mut config = t2l::config::Config::load().map_err(Error::Config)?;

    let args = Args::parse();

    if args.set_mode {
        // set mode
        if let Some(v) = args.audio_extension {
            config.set_audio_ext(v.as_str());
            println!(
                "{}",
                match config.save() {
                    Ok(_) => format!("success. config changed to {}", v),
                    Err(e) => format!("failed... error: {}", e),
                }
            );
        }
        if let Some(v) = args.txt_extension {
            config.set_txt_ext(v.as_str());
            println!(
                "{}",
                match config.save() {
                    Ok(_) => format!("success. config changed to {}", v),
                    Err(e) => format!("failed... error: {}", e),
                }
            );
        }
    } else {
        // normal mode
        //なぜかunwrap_or()にすると常にnone判定?
        let dir = PathBuf::from(match args.folder_path {
            Some(v) => v,
            None => get_user_input()?,
        });

        let audio_ext = args.audio_extension.unwrap_or(config.audio_extension);
        let line_ext = args.txt_extension.unwrap_or(config.txt_extension);

        let mut sets = t2l::PathSets::new(&dir, audio_ext, line_ext).map_err(Error::T2L)?;
        let check_list = sets.check().map_err(|_| Error::SomeErr)?;
        println!("{}", check_list);

        if get_input::yes_no("continue?(y(or enter)/n)") {
            sets.rename().map_err(|_| Error::SomeErr)?;
            // 👆で?しているので、成功したことを確認するためのメッセージを表示する
            println!("success. all file is renamed.");
        } else {
            println!("ok, cancelled. your files has not changed.");
            return Err(Error::Cancel);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use track2line_lib::config::Config;

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
}
