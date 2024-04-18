import os
import glob

def read_lines_textfile(file_name):
    data_file = open(f"{file_name}.txt", 'r')
    return data_file.read()[0:20]

while True:
    print("フォルダのパスを入力してください。")
    folder_path = input()
    wav_files = glob.glob(f"{folder_path}/*.wav")
    count = 0
    renamed_files_path = f"{folder_path}/renamed_files/"
    os.makedirs(renamed_files_path, exist_ok=True)

    for file in wav_files: #fileは音声ファイルのパス
        file_name = file.replace(".wav", "")
        lines = read_lines_textfile(file_name)
        if lines[-1] == "/":
            lines = lines.replace("/", "")
        voice_file_path = f"{renamed_files_path}/{lines}.wav"
        os.rename(file, voice_file_path)
        count += 1

    print(f"{count}個のファイルの名称を変更しました。")
    print("もう一度使用しますか?(y/n)")
    if input() != "y":
        break
