# track2line

## 概要

voisona Talk等で、出力された音声ファイルの名前を同名のテキストファイルを参照して変換するツールです。そのため、

- 音声ファイルとテキストファイルの名前が一致していること
- テキストファイルにはセリフが入っていること

上記の2点を確認の上使用してください。

## インストール方法

実行ファイルを置く場所をパスに通すとどこからでも使えます。

一例としてmac + zshで

```zsh
echo export PATH=$PATH:ユーザディレクトリに適当なフォルダ(my_toolsなど)のフルパス >> ~/.zshrc
source ~/.zshrc
```

もしくは、実行ファイルのパスをエイリアスに登録すると、フルパスで実行ファイルを指定しなくとも使用できるようになります。(Mac)

1. 適当なフォルダに実行ファイルを置く
2. その実行ファイルのパスをaliasに設定

```zsh: install
echo "alias track2line='track2lineのフルパス'" >> ~/.zshrc;
source ~/.zshrc;
```

## 使い方

実行方法は

- 引数にパスを指定 👉 [引数にパスを指定](#引数にパスを指定)
- 実行後にパスを指定
  - 引数ではなく実行後にパスを入力 👉 [引数ではなく実行後にパスを入力](#引数ではなく実行後にパスを入力)
  - 実行ファイルのディレクトリを自動取得 👉 [実行ファイルのディレクトリを自動取得](#実行ファイルのディレクトリを自動取得)

の3種類.

以下コードの実行に用いたテスト用のディレクトリ構造

```bash
> tree tests
tests
├── 1.txt   # テキストファイル
├── 1.wav   # 音声ファイル
├── bad.txt # エラー判定用

> cat 1.txt            
one         # テキストファイルの内容
```

### 引数にパスを指定

```bash
> ./track2line ~/Desktop/tests
* 1.wav                ---> one.wav
ok?(y/n)>y
success.
```

#### 引数に指定する場合には対応する拡張子を変更できます

*この例ではそれぞれのファイルの拡張子をmp3とrtfに変更しています.

```bash
> ./track2line -a mp3 -t rtf ~/Desktop/tests
* 1.mp3                ---> one.mp3
ok?(y/n)>y
success.
```

### 引数ではなく実行後にパスを入力

実行ファイルをダブルクリックして入力を求めらた際にフォルダのパスを入力.

```bash
> ~/Develop/track2line/target/release/track2line ; exit;
input Rust's Project folder path.
if you use current directory, just leave it blank.
>>~/Desktop/tests          # 👈パスを入力.
* 1.wav                ---> one.wav
ok?(y/n)>y
success.
```

### 実行ファイルのディレクトリを自動取得

```bash
> ~/Desktop/tests/track2line ; exit; # 👈実行ファイルのパスを指定して実行してます.
input Rust's Project folder path.
if you use current directory, just leave it blank.
>>                                   # 👈パスを入力しなければ実行ファイルのパスが自動で入力されます.
* 1.wav                ---> one.wav
ok?(y/n)>y
success.
```

## 免責事項

このソフトウェアを使用したことによって生じたすべての障害・損害・不具合等に関しては、私と私の関係者および私の所属するいかなる団体・組織とも、一切の責任を負いません。各自の責任においてご使用ください。

---

## Overview

This tool converts the name of the output audio file in voisona Talk, etc. by referring to a text file of the same name. Therefore,

- The names of the audio file and the text file must match.
- The text file must contain the lines.

Please check the above two points before use.

## Installation

You can use it from anywhere by passing the location of the executable file through the path.

As an example, with mac + zsh

```zsh
echo export PATH=$PATH: Full path to an appropriate folder (e.g. my_tools) in your user directory >> ~/.zshrc
source ~/.zshrc
```

1. put the executable file in an appropriate folder
1. set the path of the executable to alias

```zsh: install
echo "alias track2line='track2lineのフルパス'" >> ~/.zshrc;
source ~/.zshrc;
```

## Demo

There are three methods of execution.

- Specify path as argument 👉 [go to](#specify path as argument)
- Specify path after execution
  - Enter path after execution instead of argument 👉 [go to](#Enter path after execution instead of argument)
  - Get directory of executable files automatically 👉 [go to](#Get directory of executable files automatically)

The following is the directory structure for the test used to run the code

```bash
> tree tests
tests
├── 1.txt   # text file
├── 1.wav   # audio file
├── bad.txt # for error

> cat 1.txt            
one         # content of text file
```

### specify path as argument

```bash
> ./track2line ~/Desktop/tests
* 1.wav                ---> one.wav
ok?(y/n)>y
success.
```

#### You can change the corresponding extension if you specify it as an argument

In this example, the extension of each file is changed to mp3 and rtf.

```bash
> ./track2line -a mp3 -t rtf ~/Desktop/tests
* 1.mp3                ---> one.mp3
ok?(y/n)>y
success.
```

### Enter path after execution instead of argument

Double-click on the executable file and enter the folder path when prompted.

```bash
> ~/Develop/track2line/target/release/track2line ; exit; 👈Executed by specifying the path to the executable file.
input Rust's Project folder path.
if you use current directory, just leave it blank.
>>~/Desktop/tests          # 👈 input path.
* 1.wav                ---> one.wav
ok?(y/n)>y
success.
```

### Get directory of executable files automatically

If you do not enter a path, the path to the executable file will be entered automatically.

```bash
> ~/Desktop/tests/track2line ; exit; # 👈Executed by specifying the path to the executable file.
input Rust's Project folder path.
if you use current directory, just leave it blank.
>>                                   # 👈If you do not enter a path, the path to the executable file will be entered automatically.
* 1.wav                ---> one.wav
ok?(y/n)>y
success.
```

## Disclaimer

I, my affiliates, and any other organizations to which I belong are not responsible for any damage, loss, or malfunction caused by the use of this software. Please use this software at your own risk.
