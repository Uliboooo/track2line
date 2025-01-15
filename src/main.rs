use clap::Parser;
// use colored::*;
use get_input::get_input;
use std::env;
use std::fmt;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Parser)]
struct Args {
    /// Folder path
    folder_path: Option<String>,

    /// change audio extension *
    #[arg(short = 'a', long = "audio", help = "change audio file extension")]
    audio_extension: Option<String>,

    /// change text(lines) extension
    #[arg(short = 't', long = "text", help = "change text file extension")]
    txt_extension: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum ErrorCodeList {
    FailedGetPath,
    FailedGetTxtContent,
    FailedCreateFile,
    FailedConvert,
    ChangeCancel,
    NotFoundChangeableFiles,
    FailedGetFileName,
    FailedGetFileEx,
    FailedRename,
    // NotFound,
}
impl fmt::Display for ErrorCodeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCodeList::FailedGetPath => write!(f, "Error: Failed to get path"),
            ErrorCodeList::FailedGetTxtContent => write!(f, "Error: Failed to get text content"),
            ErrorCodeList::FailedCreateFile => write!(f, "Error: Failed to create file"),
            ErrorCodeList::FailedConvert => write!(f, "Error: Failed to convert"),
            ErrorCodeList::ChangeCancel => write!(f, "Error: cancel"),
            ErrorCodeList::NotFoundChangeableFiles => {
                write!(f, "Error: not found changeable files")
            }
            ErrorCodeList::FailedGetFileName => write!(f, "Error: failed get file name."),
            ErrorCodeList::FailedGetFileEx => write!(f, "Error: failed get file extension."),
            // ErrorCodeList::NotFound => write!(f, "not found."),
            ErrorCodeList::FailedRename => write!(f, "Error: failed rename file"),
        }
    }
}

/// 指定したディレクトリ内のファイル一覧を取得する関数
///
/// # Arguments
///
/// * `dir_path` - 対象ディレクトリのパス
///
/// # Returns
/// ファイルパスのベクター `Vec<PathBuf>`、またはエラー `ErrorCodeList`
fn get_file_list(dir_path: &PathBuf) -> Result<Vec<PathBuf>, ErrorCodeList> {
    let file_list = fs::read_dir(dir_path).map_err(|_| ErrorCodeList::FailedGetPath)?;
    let mut file_vec: Vec<PathBuf> = Vec::new();
    for entry in file_list {
        file_vec.push(match entry {
            Ok(entry) => entry.path(),
            Err(_) => return Err(ErrorCodeList::FailedGetPath),
        });
    }
    Ok(file_vec)
}

/// 指定した拡張子以外のファイルをリストから削除
///
/// # Arguments
///
/// * `list` - ファイルパスのリスト
///
/// # Returns
/// ファイルパスのリスト (指定した拡張子のファイルのみ)
fn remove_ignore_file(list: Vec<PathBuf>) -> Result<Vec<PathBuf>, ErrorCodeList> {
    let mut tmp_vec: Vec<PathBuf> = Vec::new();
    for path in list {
        let a = match (match path.extension() {
            Some(e) => e,
            None => continue,
        })
        .to_str()
        {
            Some(p) => p,
            None => continue,
        };
        if a == AUDIO_EXTENSION.get_or_init(|| "wav".to_string())
            || a == TXT_EXTENSION.get_or_init(|| "txt".to_string())
        {
            tmp_vec.push(path);
        }
    }
    Ok(tmp_vec)
}

#[derive(Debug, PartialEq, Eq)]
struct SetAudioTxt {
    audio_path: PathBuf,
    txt_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
struct ChangedSetAudioTxt {
    base: SetAudioTxt,
    new: PathBuf,
}

/// ファイルのリストから同名の音声とセリフファイルをリスト化(Vec<SetAudioTxt)
fn create_same_name_list(
    list: Vec<PathBuf>,
    dir_path: PathBuf,
) -> Result<Vec<SetAudioTxt>, ErrorCodeList> {
    let mut file_set_list: Vec<SetAudioTxt> = vec![];
    for path in list {
        let file_ex = if let Some(ex) = path.extension() {
            ex
        } else {
            if cfg!(test) {
                return Err(ErrorCodeList::FailedGetFileEx);
            }
            continue;
        };
        let file_name = if let Some(name) = path.file_stem() {
            match name.to_str() {
                Some(name) => name,
                None => {
                    if cfg!(test) {
                        return Err(ErrorCodeList::FailedGetFileName);
                    }
                    continue;
                }
            }
        } else {
            continue;
        };
        // 音声かテキストかを判別
        file_set_list.push(
            if &file_ex.to_string_lossy() == AUDIO_EXTENSION.get_or_init(|| "wav".to_string()) {
                let mut dir_path_2 = dir_path.clone();
                dir_path_2.push(PathBuf::from(format!(
                    "{}.{}",
                    file_name, // audio file name
                    TXT_EXTENSION.get_or_init(|| "txt".to_string())
                )));

                if dir_path_2.exists() {
                    // セリフファイルが存在
                    SetAudioTxt {
                        audio_path: path,
                        txt_path: dir_path_2,
                    }
                } else {
                    continue; // セリフファイルが存在しない場合
                }
            } else {
                // txtの場合
                continue;
            },
        )
    }
    if cfg!(test) {
        println!("{:?}", &file_set_list)
    }
    if file_set_list.is_empty() {
        Err(ErrorCodeList::NotFoundChangeableFiles)
    } else {
        Ok(file_set_list)
    }
}

/// 新規ファイル名を付与したリスト
fn create_new_file_list(
    list: Vec<SetAudioTxt>,
    new_dir_path: PathBuf,
) -> Result<Vec<ChangedSetAudioTxt>, ErrorCodeList> {
    let mut tmp: Vec<ChangedSetAudioTxt> = vec![];

    for file in list {
        // let old_audio_path = file.audio_path.clone();
        if cfg!(test) {
            println!("{} have", &file.txt_path.to_string_lossy());
        }
        let text_content = get_txt_content(&file.txt_path)?;
        let new_audio_path = PathBuf::from(&new_dir_path).join(format!(
            "{}.{}",
            text_content.trim(),
            AUDIO_EXTENSION.get_or_init(|| "wav".to_string()).trim(),
        ));

        tmp.push(ChangedSetAudioTxt {
            base: SetAudioTxt {
                audio_path: file.audio_path,
                txt_path: file.txt_path,
            },
            new: new_audio_path,
        });
    }
    // println!("tmp: {:?}", &tmp);
    Ok(tmp)
}

/// パスのテキストファイルの内容を取得
fn get_txt_content(file_path: &Path) -> Result<String, ErrorCodeList> {
    if cfg!(test) {
        eprintln!("get_txt_content: {:?}", file_path);
    }
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
fn create_folder(input_path: &PathBuf) -> Result<(), ErrorCodeList> {
    match fs::create_dir_all(input_path) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorCodeList::FailedCreateFile),
    }
}

/// pathをstringに
fn path_to_string(path: &Path) -> Result<String, ErrorCodeList> {
    path.file_name()
        .and_then(|filename| filename.to_str())
        .map(|s| s.to_string())
        .ok_or(ErrorCodeList::FailedConvert)
}

/// 確認用にリスト表示
fn confirm_changes(list: &Vec<ChangedSetAudioTxt>) -> Result<bool, ErrorCodeList> {
    if cfg!(test) {
        return Ok(true);
    }
    for item in list {
        println!(
            "* {:width$} ---> {}",
            path_to_string(&item.base.audio_path)?,
            path_to_string(&item.new)?,
            width = 20,
        );
    }
    Ok(get_input("ok?(y/n)") != "n")
}

/// ファイル名を変更
fn rename(list: Vec<ChangedSetAudioTxt>) -> Result<(), ErrorCodeList> {
    for file in list {
        fs::rename(file.base.audio_path, file.new).map_err(|_| ErrorCodeList::FailedRename)?;
    }
    Ok(())
}

/// 処理まとめ
fn process_directory(dir_path: &mut PathBuf) -> Result<String, ErrorCodeList> {
    let list = remove_ignore_file(get_file_list(dir_path)?)?;
    let new_folder_path = dir_path.clone().join("renamed");
    create_folder(&new_folder_path)?;
    let same_name_list = create_same_name_list(list, dir_path.to_path_buf())?;
    let added_new_name_list = create_new_file_list(same_name_list, new_folder_path)?;

    if confirm_changes(&added_new_name_list)? {
        rename(added_new_name_list)?;
        Ok("success.".to_string())
    } else {
        println!("canceled.");
        Err(ErrorCodeList::ChangeCancel)
    }
}

// 拡張子の設定
static AUDIO_EXTENSION: OnceLock<String> = OnceLock::new();
static TXT_EXTENSION: OnceLock<String> = OnceLock::new();

/// ## Examples
///
/// ### when place exe file in target directory.
///
/// ```
/// //target file list
/// $ ls
/// 1.txt   1.wav   2.txt   2.wav   3.txt   3.wav   track2line
/// $ cat 1.txt 2.txt 3.txt
/// first
/// second
/// three
///
/// // rename with this tool
/// // run
/// $ ./track2line
/// 1.wav                ---> first.wav
/// 3.wav                ---> three.wav
/// 2.wav                ---> second.wav
/// 本当に変更しますか?(y/n)>y
/// ```
///
/// ### when set the argument as a path
///
/// ```
/// $ ./track2line /target
/// 1.wav                ---> first.wav
/// 3.wav                ---> three.wav
/// 2.wav                ---> second.wav
/// 本当に変更しますか?(y/n)>y
/// ```
///
/// ### when audio file's arg is extensions other than wav
///
/// If you click on an executable file, you cannot specify an extension other than the default (wav, txt).
///
/// ```
/// // use mp3 and rtf
/// $ ./track2line -a mp3 -t rtf
/// 1.mp3                ---> one.mp3
/// 2.mp3                ---> two.mp3
/// 3.mp3                ---> three.mp3
/// 本当に変更しますか?(y/n)>n
/// ```
fn main() -> Result<(), ErrorCodeList> {
    // TODO: 👆CLI向けにちゃんとエラーを返す処理をあとで
    let args = Args::parse();

    let _ = AUDIO_EXTENSION.set(args.audio_extension.unwrap_or("wav".to_string()));
    let _ = TXT_EXTENSION.set(args.txt_extension.unwrap_or("txt".to_string()));

    let mut work_path = match args.folder_path {
        Some(p) => PathBuf::from(p),
        None => {
            let input = get_input("input Rust's Project folder path.\nif you use current directory, just leave it blank.\n>");
            if input.is_empty() {
                env::current_exe()
                    .map_err(|_| ErrorCodeList::FailedGetPath)?
                    .parent()
                    .ok_or(ErrorCodeList::FailedGetPath)?
                    .to_path_buf()
                // env::current_dir().map_err(|_| ErrorCodeList::FailedGetPath)?
            } else {
                PathBuf::from(input)
            }
        }
    };

    if cfg!(debug_assertions) {
        println!(
            "The current dir path: {:?}\nThe current audio extension is {:?}\nThe current txt extension is {:?}",
            work_path,
            AUDIO_EXTENSION.get(),
            TXT_EXTENSION.get(),
        );
    }

    // 引数にフォルダのパスがあった場合はそれを、ない場合はカレントディレクトリのパスを読む
    // let mut work_path = match match args.folder_path {
    //     // 引数にパスがあった場合
    //     Some(folder_path) => Ok(PathBuf::from(folder_path)),
    //     // 引数にパスがない場合
    //     None => {
    //         // 実行ファイルのパスを返す
    //         match (match env::current_exe() {
    //             Ok(path) => path,
    //             Err(_) => {
    //                 eprintln!("{}", ErrorCodeList::FailedGetPath);
    //                 return;
    //             }
    //         })
    //         .parent()
    //         {
    //             Some(p) => Ok(p.to_path_buf()),
    //             None => Err(ErrorCodeList::FailedGetPath),
    //         }
    //     }
    // } {
    //     Ok(path) => path,
    //     Err(code) => {
    //         eprintln!("ERROR: {}", code);
    //         return;
    //     }
    // };

    let result = process_directory(&mut work_path);

    println!("{}", result.unwrap_or_else(|e| e.to_string()));
    Ok(())
}

#[cfg(test)]
mod test_s {
    use super::*;
    use std::{io::Write, vec};

    /// test用のフォルダとファイルを作成
    #[test]
    fn init_test_files() {
        let _ = fs::remove_dir_all("test");
        let _ = fs::create_dir("test");
        let file_list = [
            PathBuf::from("test/1.txt"),
            PathBuf::from("test/1.wav"),
            PathBuf::from("test/bad.txt"),
        ];
        for p in file_list {
            // let a = dir.push(&p);
            let _ = fs::File::create(&p);
        }
        // let mut buf = fs::File::open("test/1.txt").unwrap();
        println!("{:?}", PathBuf::from("test/1.txt").exists());

        let mut buf = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("test/1.txt")
            .unwrap();
        buf.write_all(b"one").unwrap();
    }

    /// テスト用のフォルダを作成、初期化する関数
    ///
    /// # Arguments
    ///
    /// * `dir` - テストフォルダのパス
    ///
    /// # Returns
    /// Result<(), (TestError, String)> - 成功した場合Ok(()), 失敗した場合Err((TestError, String))
    /// テスト用のフォルダを作成、初期化する関数
    ///
    /// # Arguments
    ///
    /// * `dir` - テストフォルダのパス
    #[test]
    fn file_list_test() {
        init_test_files();
        let dir = PathBuf::from("test");
        let list = get_file_list(&dir).unwrap();
        let list_2 = remove_ignore_file(list).unwrap();
        assert_eq!(
            list_2,
            vec![
                PathBuf::from("test/bad.txt"),
                PathBuf::from("test/1.wav"),
                PathBuf::from("test/1.txt"),
            ]
        );
    }

    #[test]
    fn new_list() {
        init_test_files();

        let correct_list =
            remove_ignore_file(get_file_list(&PathBuf::from("test")).unwrap()).unwrap();

        let list_2 = create_same_name_list(correct_list, PathBuf::from("test"));
        let new_list = create_new_file_list(list_2.unwrap(), PathBuf::from("test/renamed"));

        assert_eq!(
            new_list.unwrap(),
            vec![ChangedSetAudioTxt {
                base: SetAudioTxt {
                    audio_path: PathBuf::from("test/1.wav"),
                    txt_path: PathBuf::from("test/1.txt")
                },
                new: PathBuf::from("test/renamed/one.wav")
            }]
        )
    }

    #[test]
    fn global() {
        init_test_files();

        if let Err(e) = process_directory(&mut PathBuf::from("test")) {
            eprintln!("error: {}", e);
        };

        let file_list = env::current_dir()
            .unwrap()
            .join("test")
            .join("renamed")
            .read_dir()
            .unwrap();
        // let mut list: Vec<String> = Vec::new();
        for file in file_list {
            assert!(file.unwrap().file_name() == "one.wav");
        }
    }
}
