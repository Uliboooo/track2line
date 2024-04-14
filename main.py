import os
import glob

while True:
    print("フォルダのパスを入力してください。")
    folder_path = input()

    wav_files = glob.glob(f"{folder_path}/*.wav")
    count = 0

    for file in wav_files:
        before_path = file
        data_file = open(f"{file.replace(".wav", "")}.txt", 'r')
        after_name = f"{data_file.read()}"
        data_file.close()
        after_name = f"{after_name[0:20]}"
        last_after_name = after_name[-1]
        if last_after_name == "/" or last_after_name == "\\":
            after_name = after_name.replace("/", "")
        after_name = f"{after_name}.wav"
        dir = f"{folder_path}/renamed_files/"
        os.makedirs(dir, exist_ok=True)
        after_path = f"{dir}{after_name}"
        os.rename(before_path, after_path)
        count += 1
    print(f"{count}個のファイルの名称を変更しました。")
    print("もう一度使用しますか?(y/n)")
    if input() != "y":
        break
