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
    /// フォルダを指定 第一引数にパスを指定
    /// ```
    /// ./voicefile_name_changer folder_path
    /// ```
    folder_path: Option<String>,

    /// 音声ファイルの拡張子を変更するか
    /// ```
    /// ./voicefile_name_changer -a mp3
    /// ```
    #[arg(short = 'a', long = "audio", help = "change audio file extension")]
    audio_extension: Option<String>,

    /// セリフ(テキスト)ファイルの拡張子を変更するか
    /// ```
    /// ./voicefile_name_changer -t rtf
    /// ```
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
            ErrorCodeList::FailedGetPath => write!(f, "Failed to get path"),
            ErrorCodeList::FailedGetTxtContent => write!(f, "Failed to get text content"),
            ErrorCodeList::FailedCreateFile => write!(f, "Failed to create file"),
            ErrorCodeList::FailedConvert => write!(f, "Failed to convert"),
            ErrorCodeList::ChangeCancel => write!(f, "cancel"),
            ErrorCodeList::NotFoundChangeableFiles => write!(f, "not found changeable files"),
            ErrorCodeList::FailedGetFileName => write!(f, "failed get file name."),
            ErrorCodeList::FailedGetFileEx => write!(f, "failed get file extension."),
            // ErrorCodeList::NotFound => write!(f, "not found."),
            ErrorCodeList::FailedRename => write!(f, "failed rename file"),
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
            println!("{} have", &file.txt_path.to_string_lossy(),);
        }
        let text_content = get_txt_content(&file.txt_path)?;
        let new_audio_path = PathBuf::from(&new_dir_path).join(format!(
            "{}.{}",
            text_content,
            AUDIO_EXTENSION.get_or_init(|| "wav".to_string())
        ));
        tmp.push(ChangedSetAudioTxt {
            base: SetAudioTxt {
                audio_path: file.audio_path,
                txt_path: file.txt_path,
            },
            new: new_audio_path,
        });
    }
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
fn show_confirm_list(list: &Vec<ChangedSetAudioTxt>) -> Result<bool, ErrorCodeList> {
    if cfg!(test) || cfg!(debug_assertions) {
        return Ok(true);
    }
    for item in list {
        println!(
            "{:width$} ---> {}",
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
    if show_confirm_list(&added_new_name_list)? {
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
/// 1.txt   1.wav   2.txt   2.wav   3.txt   3.wav   voicefile_name_changer
/// $ cat 1.txt 2.txt 3.txt
/// first
/// second
/// three
///
/// // rename with this tool
/// // run
/// $ ./voicefile_name_changer
/// 1.wav                ---> first.wav
/// 3.wav                ---> three.wav
/// 2.wav                ---> second.wav
/// 本当に変更しますか?(y/n)>y
/// ```
///
/// ### when set the argument as a path
///
/// ```
/// $ ./voicefile_name_changer /target
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
/// $ ./voicefile_name_changer -a mp3 -t rtf
/// 1.mp3                ---> one.mp3
/// 2.mp3                ---> two.mp3
/// 3.mp3                ---> three.mp3
/// 本当に変更しますか?(y/n)>n
/// ```
///
fn main() {
    let args = Args::parse();

    let _ = AUDIO_EXTENSION.set(args.audio_extension.unwrap_or("wav".to_string()));
    let _ = TXT_EXTENSION.set(args.txt_extension.unwrap_or("txt".to_string()));

    if cfg!(debug_assertions) {
        println!(
            "The current audio extension is {:?}\nThe current txt extension is {:?}",
            AUDIO_EXTENSION.get(),
            TXT_EXTENSION.get(),
        );
    }

    // 引数にフォルダのパスがあった場合はそれを、ない場合はカレントディレクトリのパスを読む
    let mut work_path = match match args.folder_path {
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

    let result = process_directory(&mut work_path);

    println!("{}", result.unwrap_or_else(|e| e.to_string()));
}

#[cfg(test)]
mod test_s {
    use super::*;
    use std::{io::Write, vec};

    enum TestError {
        FailedGetFileList,
        FailedGetFile,
        // FailedCopy,
        FailedInit,
        FsErrorFailedCreate,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TestError::FailedGetFileList => write!(f, "failed get dile list"),
                TestError::FailedGetFile => write!(f, "failed get file"),
                // TestError::FailedCopy => write!(f, "failed copy"),
                TestError::FsErrorFailedCreate => write!(f, "failed create"),
                TestError::FailedInit => write!(f, "failed init"),
            }
        }
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
        let dir = PathBuf::from("test_files/test");
        let list = get_file_list(&dir).unwrap();
        let list_2 = remove_ignore_file(list).unwrap();
        assert_eq!(
            list_2,
            vec![
                PathBuf::from("test_files/test/foo.txt"),
                PathBuf::from("test_files/test/1.wav"),
                PathBuf::from("test_files/test/1.txt")
            ]
        );
    }

    #[test]
    fn new_list() {
        let list = vec![
            PathBuf::from("test_files/test/foo.txt"),
            PathBuf::from("test_files/test/1.wav"),
            PathBuf::from("test_files/test/1.txt"),
        ];
        let removed_list = remove_ignore_file(list).unwrap();

        let list_2 = create_same_name_list(removed_list, PathBuf::from("test_files/test/"));
        let new_list =
            create_new_file_list(list_2.unwrap(), PathBuf::from("test_files/test/renamed"));
        // assert_eq!(list_2, vec![SetAudioTxt { audio_path: PathBuf::from("test_files/test/1.wav"), txt_path: PathBuf::from("1.txt") }]);
        // eprintln!("{:?}", new_list);
        assert_eq!(
            new_list.unwrap(),
            vec![ChangedSetAudioTxt {
                base: SetAudioTxt {
                    audio_path: PathBuf::from("test_files/test/1.wav"),
                    txt_path: PathBuf::from("test_files/test/1.txt")
                },
                new: PathBuf::from("test_files/test/one")
            }]
        )
    }

    fn init_test_folder(dir: &Path) -> Result<(), (TestError, String)> {
        let file_list = dir
            .read_dir()
            .map_err(|_| (TestError::FailedGetFileList, "".to_string()))?;
        for file_path in file_list {
            let path = file_path
                .map_err(|_| (TestError::FailedGetFile, "".to_string()))?
                .path();
            if path.is_dir() {
                fs::remove_dir_all(&path)
                    .map_err(|_| (TestError::FailedInit, "failed delete folder".to_string()))?;
                continue;
            }
            fs::remove_file(&path)
                .map_err(|_| (TestError::FailedInit, path.to_string_lossy().to_string()))?;
        }
        Ok(())
    }

    /// テスト用のファイルを生成する関数
    ///
    /// # Arguments
    ///
    /// * `dir` - テストファイルを作成するディレクトリのパス
    ///
    /// # Returns
    /// Result<(), TestError> - 成功した場合Ok(()), 失敗した場合Err(TestError)
    fn create_test_files(dir: &Path) -> Result<(), TestError> {
        let file_names = [
            (PathBuf::from("1.wav"), ""),
            (PathBuf::from("2.wav"), ""),
            (PathBuf::from("3.wav"), ""),
            (PathBuf::from("bar.wav"), ""),
            (PathBuf::from("1.txt"), "one"),
            (PathBuf::from("2.txt"), "two"),
            (PathBuf::from("3.txt"), "three"),
            (PathBuf::from("foo.txt"), "foo"),
        ];
        for file in file_names {
            let file_path = dir.join(file.0);
            let mut f = fs::File::create(&file_path).map_err(|_| TestError::FsErrorFailedCreate)?;
            f.write_all(file.1.as_bytes())
                .map_err(|_| TestError::FsErrorFailedCreate)?;
        }
        Ok(())
    }

    // fn copy_dir(from: &PathBuf, to: &Path) -> Result<(), TestError> {
    //     let from_dir_files = fs::read_dir(from).map_err(|_| TestError::FailedGetFileList)?;
    //     for file in from_dir_files {
    //         let from_file_path = file.map_err(|_| TestError::FailedGetFile)?.path();
    //         let to_path = to.join(from_file_path.file_name().unwrap());
    //         fs::copy(from_file_path, to_path).map_err(|_| TestError::FailedCopy)?;
    //     }
    //     Ok(())
    // }

    /// テスト環境の準備をする関数。テストフォルダを作成し、テストファイルを作成する。
    ///
    /// # Arguments
    ///
    /// * `dir` - テスト環境を作成するディレクトリのパス
    fn ready(dir: &Path) -> Result<(), TestError> {
        if !&dir.exists() {
            fs::create_dir(dir).map_err(|_| TestError::FsErrorFailedCreate)?;
        }

        if let Err(e) = init_test_folder(dir) {
            println!("error: {}\npath: {}", e.0, e.1);
            return Err(e.0);
        };
        create_test_files(dir)?;

        Ok(())
    }

    #[test]
    fn gloal() {
        let mut test_folder_path = env::current_dir()
            .expect("error")
            .join("test_files")
            .join("test");

        match ready(&test_folder_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: {}", e);
                return;
            }
        }

        let true_file_list = vec!["two.wav", "three.wav", "one.wav"];
        if let Err(e) = process_directory(&mut test_folder_path) {
            eprintln!("error_: {}", e);
        };

        let dir_list = test_folder_path.join("renamed").read_dir().unwrap();
        let mut list: Vec<String> = Vec::new();
        for file in dir_list {
            list.push(
                file.unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
        assert_eq!(list, true_file_list);
    }
}
