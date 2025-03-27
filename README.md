# Track2line cli

## what's tool

this is tool that converts name of audio file provide by voisona talk, etc. by referring to a text file of the same name.

therefore, please check there points.

- the names of the audio file and the text file must match.
- the text file must contain the lines.(when lines is empty, insert "_" to name of the audio file.)

---

voisona talkなどで提供される音声ファイル名を、同名のテキストファイルを参照して変換するツールです。

そのため、以下の点をご確認ください。

- 音声ファイル名とテキストファイル名が一致していること。
- テキストファイルに行が含まれていること(行が空の場合は音声ファイル名に「_」を挿入)

## usage as a tool (**recommend**)

double click on `track2line.exe` or `track2line` file. then, input the path of target folder.

```bash
# open cli tool by double click on `track2line.exe` or `track2line` file.
target folder>assets_for_test/assets # 👈input the path of target folder
* Talk1_2.wav          ---> まずはジェネリック型。.wav
* Talk1_3.wav          ---> ジェネリック型、もしくはジェネリクスとは.wav
* include_whs_end .wav ---> include_whs_end.wav
* Talk1_1.wav          ---> 100秒で分かるジェネリック型とトレイト.wav
* Talk1_4.wav          ---> 以下のようにとても似ているが引数の型だけ.wav
* non_line.wav         ---> _.wav

continue?(y(yes or enter)/n)y        # if you ok, input y or enter, if you want to cancel, input n.
success. all file is renamed.
```

## usage as a cli

by using `-y`, you can avoid interactive inputs.

```bash
# track2line -h
Usage: track2line [OPTIONS] [FOLDER_PATH]

Arguments:
  [FOLDER_PATH]  a path of target folder

Options:
  -s, --set_mode                 set audio extension / set mode、設定などの永続化や表示、リセットを行う
                                 `track2line -s -a mp3(optional) -t rtf(optional)`
  -a, --audio <AUDIO_EXTENSION>  change audio file extension. if use set-mode(-s), change config. オーディオファイルの拡張子を変更可能。-sをつけて使うと永続化
  -r, --reset                    reset config. requires -s.　-sと同時に使用することで設定をデフォルトのwavとtxtにリセット
  -l, --list                     show config list. requires -s. -sと同時に使用することで現在の設定を表示
  -y, --yes                      don't request all interactive input. (多分)全部のインタラクティブな入力を回避できる
  -t, --text <TXT_EXTENSION>     change text file extension.  if use set-mode(-s), change config. テキストファイルの拡張子を変更可能。-sをつけて使うと永続化
  -h, --help                     Print help
```

### normal mode

```bash
track2line /folder/path
* Talk1_2.wav          ---> まずはジェネリック型。.wav
* Talk1_3.wav          ---> ジェネリック型、もしくはジェネリクスとは.wav
* include_whs_end .wav ---> include_whs_end.wav
* Talk1_1.wav          ---> 100秒で分かるジェネリック型とトレイト.wav
* Talk1_4.wav          ---> 以下のようにとても似ているが引数の型だけ.wav
* non_line.wav         ---> _.wav

continue?(y(yes or enter)/n)y        # if you ok, input y or enter, if you want to cancel, input n.
success. all file is renamed.
```

### set mode

This is the mode in which config(file) is changed.

## when use extension other than `wav` or `txt`

temporarily use. 一時的な拡張子の変更は引数の`-a`や`-t`オプションの引数で。

```bash
# when using mp3 and rtf
track2line -a mp3 -t rtf [FOLDER_PATH]
# or (You can also use either one)
track2line -a mp3
```

configuration persistence. 設定の永続化。`-s`オプションでset modeになり設定を永続化できます。

```bash
track2line -s -a mp3 -t rtf
# You can also use either one
track2line -s -a mp3
```

show config list. you can see config list by using both `-s` and `-l`.

```bash
track2line -sl
audio extension: wav
txt extension: txt
these config saved on /Users/user-name/Library/Application Support/track2line/config.toml
```

reset configuration. 設定のリセット。

```bash
track2line -sr
```

## install

add to the PATH of dir that include executable file (`track2line`).

```zsh
echo executable_file_path >> ~/.zshrc
```

## uninstall

remove from the PATH of dir that include executable file (`track2line`).
and remove config file 👇.

- windows
  - `%USERPROFILE%\AppData\Local\track2line\config.toml`
- macos
  - `~/Library/Application Support/track2line/config.toml`
- linux
  - `~/.config/track2line/config.toml`

## Disclaimer

I, my affiliates, and any other organizations to which I belong are not responsible for any damage, loss, or malfunction caused by the use of this software. Please use this software at your own risk.

---

このソフトウェアを使用したことによって生じたすべての障害・損害・不具合等に関しては、私と私の関係者および私の所属するいかなる団体・組織とも、一切の責任を負いません。各自の責任においてご使用ください。
