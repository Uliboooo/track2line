# Track2line cli

core functionality is composed of https://github.com/Uliboooo/track2line_lib.

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

## usage

```bash
# track2line --help
Usage: track2line_cli [OPTIONS] <FOLDER_PATH>

Arguments:
  <FOLDER_PATH>  target folder path

Options:
  -a, --audio <AUDIO_EXTENSION>  change audio file extension
  -t, --text <TXT_EXTENSION>     change text file extension
  -s, --set_audio                set audio extension
  -h, --help                     Print help
```

### when use `mp3` as audio file and `rtf` as lines file.

```bash
track2line -a mp3 -t rtf <FOLDER_PATH>
```

### how to persist settings

```bash
# set mp3 as audio extension
track2line --set_audio -a mp3
# set rtf as text extension
track2line --set_text -t rtf
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

```zsh
sed -i '/executable_file_path/d' ~/.zshrc
```

## Disclaimer

I, my affiliates, and any other organizations to which I belong are not responsible for any damage, loss, or malfunction caused by the use of this software. Please use this software at your own risk.

---

このソフトウェアを使用したことによって生じたすべての障害・損害・不具合等に関しては、私と私の関係者および私の所属するいかなる団体・組織とも、一切の責任を負いません。各自の責任においてご使用ください。
