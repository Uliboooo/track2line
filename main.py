import os
import glob

def read_lines_textfile(file_name):
    data_file = open(f"{file_name}.txt", 'r', encoding='utf-8')
    return data_file.read()[0:20]

while True:
    print("フォルダのパスを入力してください。")
    folder_path = input()
    wav_files = glob.glob(f"{folder_path}/*.wav")
    count = 0
    renamed_files_path = os.path.join(folder_path, "renamed_files", "")
    os.makedirs(renamed_files_path, exist_ok=True)

    for file in wav_files: #fileは音声ファイルのパス
        file_name = file.replace(".wav", "")
        lines = read_lines_textfile(file_name)
        lines = lines.replace("/", "")
        lines = lines.replace("\\", "")
        voice_file_path = os.path.join(renamed_files_path, f"{lines}.wav")
        os.rename(file, voice_file_path)
        count += 1

    print(f"{count}個のファイルの名称を変更しました。")
    print("もう一度使用しますか?(y/n)")
    if input() != "y":
        break
