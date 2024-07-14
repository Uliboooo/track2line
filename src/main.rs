use std::fs::{self, DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use get_input::get_input;

enum ErrorCodeList {
    FailedGetFilesList,
    FailedGetTxtContent,
    FailedCreateFile,
    NoneExtension,
    NonePath,
    FailedRename,
    FailedConvert,
    FailedShowList,
}

impl ErrorCodeList {
    fn to_string(&self) -> &str {
        match self {
            ErrorCodeList::FailedGetFilesList => "Failed to get files list",
            ErrorCodeList::FailedGetTxtContent => "Failed to get TXT content",
            ErrorCodeList::FailedCreateFile => "Failed Create File",
            ErrorCodeList::NoneExtension => "None Extension",
            ErrorCodeList::NonePath => "None Path",
            ErrorCodeList::FailedRename => "Failed Rename",
            ErrorCodeList::FailedConvert => "Failed Convert",
            ErrorCodeList::FailedShowList => "Failed Show List",
        }
    }
}

struct FilePathList {
    old_path: PathBuf,
    new_path: PathBuf,
}

fn get_files_list(path: &Path) -> Result<fs::ReadDir, ErrorCodeList>{
    match fs::read_dir(path) {
        Ok(list) => Ok(list),
        Err(_) => Err(ErrorCodeList :: FailedGetFilesList),
    }
}

fn get_txt_content(file_path: &Path) -> Result<String, ErrorCodeList> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content.chars().take(20).collect()),
        Err(_) => Err(ErrorCodeList::FailedGetTxtContent)
    }
}

fn create_folder(input_path: &Path) -> Result<(), ErrorCodeList> {
    let path = input_path;
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorCodeList::FailedCreateFile),
    }
}

fn get_audio_path(entry: Result<DirEntry, std::io::Error>) -> Result<PathBuf, ErrorCodeList>{
    match entry {
        Ok(entry) => {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "wav" {
                    Ok(path)
                } else {
                    Err(ErrorCodeList::NoneExtension)
                }
            } else {
                Err(ErrorCodeList::NoneExtension)
            }
        },
        Err(_) => {
            Err(ErrorCodeList::NonePath)
        },
    }
    
}

fn path_to_string(path: &Path) -> Result<String, ErrorCodeList> {
    if let Some(filename) = path.file_name() {
        if let Some(filename_str) = filename.to_str() {
            Ok(filename_str.to_string())
        } else {
            Err(ErrorCodeList::FailedConvert)
        }
    } else {
        Err(ErrorCodeList::FailedConvert)
    }
}

fn show_confirm_list(old_path: &Path, new_path: &Path) -> Result<(), ErrorCodeList>{
    println!(
        "{:width$} ---> {}",
        match path_to_string(old_path) {
            Ok(path_string) => path_string,
            Err(_) => return Err(ErrorCodeList::FailedShowList)
        },
        match path_to_string(new_path) {
            Ok(path_string) => path_string,
            Err(_) => return Err(ErrorCodeList::FailedShowList)
        },
        width=20,
    );
    Ok(())
}

fn rename(file_path_list: ReadDir, parent_path: &Path) -> Result<i32, ErrorCodeList>{
    let mut count = 0;
    let mut list = Vec::<FilePathList>::new();
    for entry in file_path_list {
        let audio_path = match get_audio_path(entry){ //オーディオファイルのパスを取得
            Ok(path) => path,
            Err(code) => match code {
                ErrorCodeList::NoneExtension => continue,
                _ => return Err(code)
            },
        };
        let new_audio_file_name = format!("{}.wav", //新しいオーディオファイルの名前をテキストファイルから取得
            match get_txt_content(&audio_path.with_extension("txt")) {
                Ok(content) => content,
                Err(_) => return Err(ErrorCodeList::FailedGetTxtContent)
            }.trim()
        );
        let new_audio_path = Path::new(parent_path).join("renamed_files").join(new_audio_file_name); //新しいパスを生成
        list.push(FilePathList{
            old_path: audio_path.clone(),
            new_path: new_audio_path.clone(),
        });
        match create_folder(&Path::new(parent_path).join("renamed_files")) { //配置用のフォルダ
            Ok(_) => {},
            Err(code) => return Err(code),
        }
        match show_confirm_list(&audio_path, &new_audio_path) {
            Ok(_) => {},
            Err(code) => return Err(code),
        };
    }
    println!("本当に変更しますか?(y/n)");
    if get_input() == "y".to_string() {
        for path_vec in list {
            match fs::rename(&path_vec.old_path, &path_vec.new_path) {
                Ok(_) => {},
                Err(_) => return Err(ErrorCodeList::FailedRename)
            }
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
        let files_list = match get_files_list(input_path) {
            Ok(list) => {
                // println!("リストの取得に成功");
                list
            },
            Err(code) => {
                println!("{}\n", code.to_string());
                continue;
            },
        };
        match rename(files_list, input_path) {
            Ok(count) => println!("{}個のファイルを変換しました。", count),
            Err(code) => {
                println!("{}\n", code.to_string());
                continue;
            },
        };
        println!("もう一度使用しますか?(y/n)");
        if get_input() == "n".to_string() {
            break;
        }
    }
}
