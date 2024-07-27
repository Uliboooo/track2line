use get_input::get_input;
use std::fmt;
use std::fs::{self, DirEntry, ReadDir};
use std::path::{Path, PathBuf};
enum ErrorCodeList {
    FailedGetTxtContent,
    FailedCreateFile,
    NoExtension,
    DirEntryError(std::io::Error),
    FailedRename,
    FailedConvert,
    FailedShowList,
}

impl fmt::Display for ErrorCodeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCodeList::FailedGetTxtContent => write!(f, "Failed to get text content"),
            ErrorCodeList::FailedCreateFile => write!(f, "Failed to create file"),
            ErrorCodeList::NoExtension => write!(f, "File has no extension"),
            ErrorCodeList::DirEntryError(error) => write!(f, "Failed to read dir entry: {error}"),
            ErrorCodeList::FailedRename => write!(f, "Failed to rename"),
            ErrorCodeList::FailedConvert => write!(f, "Failed to convert"),
            ErrorCodeList::FailedShowList => write!(f, "Failed to show list"),
        }
    }
}

struct FilePathList {
    old_path: PathBuf,
    new_path: PathBuf,
}

fn get_files_list(path: &Path) -> Option<fs::ReadDir> {
    fs::read_dir(path).ok()
}

fn get_txt_content(file_path: &Path) -> Result<String, ErrorCodeList> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content.chars().take(20).collect()),
        Err(_) => Err(ErrorCodeList::FailedGetTxtContent),
    }
}

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
        let new_audio_file_name = format!(
            "{}.wav", //新しいオーディオファイルの名前をテキストファイルから取得
            get_txt_content(&audio_path.with_extension("txt"))
                .map_err(|_| ErrorCodeList::FailedGetTxtContent)?
                .trim()
        );
        let new_audio_path = Path::new(parent_path)
            .join("renamed_files")
            .join(new_audio_file_name); //新しいパスを生成
        list.push(FilePathList {
            old_path: audio_path.clone(),
            new_path: new_audio_path.clone(),
        });
        create_folder(&Path::new(parent_path).join("renamed_files"))?;
        show_confirm_list(&audio_path, &new_audio_path)?;
    }
    println!("本当に変更しますか?(y/n)");
    if get_input() == "y".to_string() {
        for path_vec in list {
            fs::rename(&path_vec.old_path, &path_vec.new_path)
                .map_err(|_| ErrorCodeList::FailedRename)?;
            count += 1
        }
    }
    Ok(count)
}

fn main() {
    loop {
        println!("パスを入力して下さい。");
        let input_str = get_input();
        let input_path = Path::new(&input_str);
        let Some(files_list) = get_files_list(input_path) else {
            eprintln!("ERROR: Failed to get files list");
            continue;
        };
        match rename(files_list, input_path) {
            Ok(count) => println!("{count}個のファイルを変換しました。"),
            Err(code) => {
                eprintln!("ERROR: {code}");
                continue;
            }
        };
        println!("もう一度使用しますか?(y/n)");
        if get_input() == "n" {
            break;
        }
    }
}
