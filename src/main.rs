use std::fs::{self, ReadDir,};
use std::path::Path;
use get_input::get_input;

enum ErrorCodeList {
    FailedGetFilesList,
    FailedGetTxtContent,
    FailedCreateFile,
    NoneExtension,
    NonePath,
    FailedRename,
}

impl ErrorCodeList {
    fn to_string(&self) -> &str {
        match self {
            ErrorCodeList::FailedGetFilesList => "Failed to get files list",
            ErrorCodeList::FailedGetTxtContent => "Failed to get TXT content",
            ErrorCodeList::FailedCreateFile => "Failed Create File",
            ErrorCodeList::NoneExtension => "None Extension",
            ErrorCodeList::NonePath => "None Path",
            ErrorCodeList::FailedRename => "FailedRename",
        }
    }
}

fn get_files_list(path: &Path) -> Result<fs::ReadDir, ErrorCodeList>{
    match fs::read_dir(path) {
        Ok(list) => Ok(list),
        Err(_) => Err(ErrorCodeList :: FailedGetFilesList),
    }
}

fn get_txt_content(file_path: &Path) -> Result<String, ErrorCodeList> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok( content.get(0..20).unwrap_or(&content).to_string()),
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

fn rename(file_path_list: ReadDir, parent_path: &Path) -> Result<i32, ErrorCodeList>{
    let mut count = 0;
    for entry in file_path_list {
        let audio_path = match entry {
            Ok(entry) => {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "wav" {
                        path
                    } else {
                        continue;
                    }
                } else {
                    return Err(ErrorCodeList::NoneExtension)
                }
            },
            Err(_) => return Err(ErrorCodeList::NonePath),
        };
        let txt_path = audio_path.with_extension("txt");
        let txt_content = get_txt_content(&txt_path);
        let new_audio_file_name = format!("{}.wav",
            match txt_content {
                Ok(content) => content,
                Err(_) => return Err(ErrorCodeList::FailedGetTxtContent)
            }.trim()
        );
        match create_folder(&Path::new(parent_path).join("renamed_files")) {
            Ok(_) => {},
            Err(code) => return Err(code),
        }
        let new_audio_path = Path::new(parent_path).join("renamed_files").join(new_audio_file_name);
        match fs::rename(audio_path, new_audio_path) {
            Ok(_) => {},
            Err(_) => return Err(ErrorCodeList::FailedRename)
        }
        count += 1;
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
