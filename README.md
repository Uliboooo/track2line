# voicefile_name_changer

## 既知の問題

- wavファイル以外非対応

## 用途

voisona Talk等で、出力された音声ファイルの名前を同名のテキストファイルを参照して変換するツールです。そのため、

- 音声ファイルとテキストファイルの名前が一致していること
- テキストファイルにはセリフが入っていること

上記の2点を確認の上使用してください。

## 使い方

音声ファイルの含まれたフォルダのパスを入力してください。実行すると、「renamed_files」というフォルダが作成され、変更済みの音声ファイルが移動されます。

### 実行ファイルをターゲットディレクトリに配置するとき

```zsh
//target file list
user@pc target % ls
1.txt   1.wav   2.txt   2.wav   3.txt   3.wav   voicefile_name_changer
user@pc target % cat 1.txt 2.txt 3.txt 
first
second
three
//rename with this tool
//this tool run
user@pc ~ % ~/voicefile_name_changer
1.wav                ---> first.wav
3.wav                ---> three.wav
2.wav                ---> second.wav
本当に変更しますか?(y/n)>y
```

### パスを引数に設定するとき

```zsh
user@pc~ % ./voicefile_name_changer /target
1.wav                ---> first.wav
3.wav                ---> three.wav
2.wav                ---> second.wav
本当に変更しますか?(y/n)>y
```

## 免責事項

このソフトウェアを使用したことによって生じたすべての障害・損害・不具合等に関しては、私と私の関係者および私の所属するいかなる団体・組織とも、一切の責任を負いません。各自の責任においてご使用ください。
