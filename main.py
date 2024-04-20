import os
import re
import glob

def read_lines_textfile(file_name):
    data_file = open(f"{file_name}.txt", 'r', encoding='utf-8')
    data_file = data_file.read()
    data_file = re.sub(r'[\\/:*?"<>|]+','',data_file)
    data_file = data_file.replace("\n", "")
    data_file = data_file[0:20]
    # data_file = data_file.replace("/", "")
    # data_file = data_file.replace("\\", "")
    return data_file

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
        voice_file_path = os.path.join(renamed_files_path, f"{lines}.wav")
        os.rename(file, voice_file_path)
        count += 1

    print(f"{count}個のファイルの名称を変更しました。")
    print("もう一度使用しますか?(y/n)")
    if input() != "y":
        break
