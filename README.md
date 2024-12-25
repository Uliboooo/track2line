# voicefile_name_changer

## 用途

voisona Talk等で、出力された音声ファイルの名前を同名のテキストファイルを参照して変換するツールです。そのため、

- 音声ファイルとテキストファイルの名前が一致していること
- テキストファイルにはセリフが入っていること

上記の2点を確認の上使用してください。

## 使い方

音声ファイルの含まれたフォルダのパスを入力してください。実行すると、「renamed_files」というフォルダが作成され、変更済みの音声ファイルが移動されます。

## インストール方法

インストールというかパスをエイリアスに登録すると、フルパスで実行ファイルを指定しなくとも使用できるようになります。(Mac)

1. 適当なフォルダに実行ファイルを置く
2. その実行ファイルのパスをaliasに設定

```zsh: install
sudo cp ./voicefile_name_changer /usr/local/bin/;
echo "alias vfnc='/usr/local/bin/voicefile_name_changer'" >> ~/.zshrc;
source ~/.zshrc;
```
cp後のパスはダウンロードした実行ファイルのパスです。

### 使い方

```zsh: use
vfnc /target_folder_path
```

### 実行ファイルをターゲットディレクトリに配置するとき

```
// show file list
$ ls
1.txt   1.wav   2.txt   2.wav   3.txt   3.wav   voicefile_name_changer

$ cat 1.txt 2.txt 3.txt 
first
second
three

$ ./voicefile_name_changer
1.wav                ---> first.wav
3.wav                ---> three.wav
2.wav                ---> second.wav
本当に変更しますか?(y/n)>y
```

### パスを引数に設定するとき

```
$ ./voicefile_name_changer /target_folder_path
1.wav                ---> first.wav
3.wav                ---> three.wav
2.wav                ---> second.wav
本当に変更しますか?(y/n)>y
```

### wav, txt拡張子以外のファイルを扱う

実行ファイルをターゲットディレクトリに配置するときは利用できません。

```
$ ls
1.rtf   1.mp3   2.rtf   2.mp3   3.rtf   3.mp3   voicefile_name_changer

$ ./voicefile_name_changer -a mp3 -t rtf /target_folder_path
1.mp3                ---> one.mp3
2.mp3                ---> two.mp3
3.mp3                ---> three.mp3
```

## 免責事項

このソフトウェアを使用したことによって生じたすべての障害・損害・不具合等に関しては、私と私の関係者および私の所属するいかなる団体・組織とも、一切の責任を負いません。各自の責任においてご使用ください。
