use clap::Parser;
// use colored::*;
use get_input::get_input;
use std::env;
use std::fmt;
use std::fs::{self, DirEntry, ReadDir};
use std::io::stdout;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Args {
    /// フォルダを指定
    folder_path: Option<String>,
}

enum ErrorCodeList {
    FailedGetPath,
    FailedGetTxtContent,
    FailedCreateFile,
    NoExtension,
    DirEntryError(std::io::Error),
    FailedRename,
    FailedConvert,
    FailedShowList,
    ChangeCancel,
}
impl fmt::Display for ErrorCodeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCodeList::FailedGetPath => write!(f, "Failed to get path"),
            ErrorCodeList::FailedGetTxtContent => write!(f, "Failed to get text content"),
            ErrorCodeList::FailedCreateFile => write!(f, "Failed to create file"),
            ErrorCodeList::NoExtension => write!(f, "File has no extension"),
            ErrorCodeList::DirEntryError(error) => write!(f, "Failed to read dir entry: {error}"),
            ErrorCodeList::FailedRename => write!(f, "Failed to rename"),
            ErrorCodeList::FailedConvert => write!(f, "Failed to convert"),
            ErrorCodeList::FailedShowList => write!(f, "Failed to show list"),
            ErrorCodeList::ChangeCancel => write!(f, "cancel"),
        }
    }
}

fn get_txt_content(file_path: &Path) -> Result<String, ErrorCodeList> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content.chars().take(20).collect()),
        Err(_) => {
            if cfg!(debug_assertions) {
                println!("{}", file_path.to_string_lossy());
            }
            Err(ErrorCodeList::FailedGetTxtContent)
        }
    }
}

/// 受け取ったパスにフォルダを作成
fn create_folder(input_path: &Path) -> Result<(), ErrorCodeList> {
    let path = input_path;
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorCodeList::FailedCreateFile),
    }
}

fn get_audio_path(entry: Result<DirEntry, std::io::Error>) -> Result<PathBuf, ErrorCodeList> {
    Some(entry.map_err(ErrorCodeList::DirEntryError)?.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "wav"))
        .ok_or(ErrorCodeList::NoExtension)
}

fn path_to_string(path: &Path) -> Result<String, ErrorCodeList> {
    path.file_name()
        .and_then(|filename| filename.to_str())
        .map(|s| s.to_string())
        .ok_or(ErrorCodeList::FailedConvert)
}

fn show_confirm_list(old_path: &Path, new_path: &Path) -> Result<(), ErrorCodeList> {
    println!(
        "{:width$} ---> {}",
        path_to_string(old_path).map_err(|_| ErrorCodeList::FailedShowList)?,
        path_to_string(new_path).map_err(|_| ErrorCodeList::FailedShowList)?,
        width = 20,
    );
    Ok(())
}

struct FilePathList {
    old_path: PathBuf,
    new_path: PathBuf,
}

/// パスのリスト(ReadDir)と親フォルダのパスを受け取り、全てをrename
fn rename(file_path_list: ReadDir, parent_path: &Path) -> Result<i32, ErrorCodeList> {
    let mut count = 0;
    let mut list = Vec::<FilePathList>::new();

    for entry in file_path_list {
        let audio_path = match get_audio_path(entry) {
            //オーディオファイルのパスを取得
            Ok(path) => path,
            Err(code) => match code {
                ErrorCodeList::NoExtension => continue,
                _ => return Err(code),
            },
        };
        // 新しいオーディオファイルの名前をテキストファイルから取得
        let new_audio_file_name = format!(
            "{}.wav",
            get_txt_content(&audio_path.with_extension("txt"))
                .map_err(|_| ErrorCodeList::FailedGetTxtContent)?
                .trim()
        );
        // ファイル名とフォルダパスを結合して新しいファイルパスを指定
        let new_audio_path = Path::new(parent_path)
            .join("renamed_files")
            .join(new_audio_file_name);

        list.push(FilePathList {
            old_path: audio_path.clone(),
            new_path: new_audio_path.clone(),
        });
        // 変換済みファイルを入れるフォルダを作成
        create_folder(&Path::new(parent_path).join("renamed_files"))?;
        show_confirm_list(&audio_path, &new_audio_path)?;
    }

    print!("本当に変更しますか?(y/n)>");
    stdout().flush().unwrap();
    if get_input() == "y" {
        for path_vec in list {
            fs::rename(&path_vec.old_path, &path_vec.new_path)
                .map_err(|_| ErrorCodeList::FailedRename)?;
            count += 1
        }
        Ok(count)
    } else {
        Err(ErrorCodeList::ChangeCancel)
    }
}

/// ## Examples
/// 
/// ### when place exefile in target directory.
/// 
/// ```
/// //target file list
/// user@pc target % ls
/// 1.txt   1.wav   2.txt   2.wav   3.txt   3.wav   voicefile_name_changer
/// user@pc target % cat 1.txt 2.txt 3.txt 
/// first
/// second
/// three
/// //rename with this tool
/// //this tool run
/// user@pc ~ % ~/voicefile_name_changer
/// 1.wav                ---> first.wav
/// 3.wav                ---> three.wav
/// 2.wav                ---> second.wav
/// 本当に変更しますか?(y/n)>y
/// ```
/// 
/// ### when set the argument as a path
/// 
/// ```
/// user@pc~ % ./voicefile_name_changer /target
/// 1.wav                ---> first.wav
/// 3.wav                ---> three.wav
/// 2.wav                ---> second.wav
/// 本当に変更しますか?(y/n)>y
/// ```
fn main() {
    // loop {
    let args = Args::parse();
    // 引数にフォルダのパスがあった場合はそれを、ない場合はカレントディレクトリのパスを読む
    let work_path = match match args.folder_path {
        // 引数にパスがあった場合
        Some(folder_path) => Ok(PathBuf::from(folder_path)),
        // 引数にパスがない場合
        None => {
            // 実行ファイルのパスを返す
            match (match env::current_exe() {
                Ok(path) => path,
                Err(_) => {
                    eprintln!("{}", ErrorCodeList::FailedGetPath);
                    return;
                }
            })
            .parent()
            {
                Some(p) => Ok(p.to_path_buf()),
                None => Err(ErrorCodeList::FailedGetPath),
            }
        }
    } {
        Ok(path) => path,
        Err(code) => {
            eprintln!("ERROR: {}", code);
            return;
        }
    };

    let Some(files_list) = fs::read_dir(&work_path).ok() else {
        eprintln!("ERROR: Failed to get files list");
        return;
    };

    match rename(files_list, &work_path) {
        Ok(count) => println!("{count}個のファイルを変換しました。"),
        Err(code) => {
            eprintln!("ERROR: {}", code);
        }
    };
}
