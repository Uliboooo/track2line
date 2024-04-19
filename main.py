import os
import glob

def read_lines_textfile(file_name):
    data_file = open(f"{file_name}.txt", 'r')
    return data_file.read()[0:20]

while True:
    print("フォルダのパスを入力してください。")
    folder_path = input()
    # print("インデックスを付けますか?([a]昇順(0~9)/ [d]降順(9~0)/ [n]なし)")
    # index_enable = input()
    # file_number = sum(os.path.isfile(os.path.join(folder_path, name)) for name in os.listdir(folder_path)) / 2
    wav_files = glob.glob(f"{folder_path}/*.wav")
    count = 0
    renamed_files_path = os.path.join(folder_path, "renamed_files", "")
    os.makedirs(renamed_files_path, exist_ok=True)

    for file in wav_files: #fileは音声ファイルのパス
        file_name = file.replace(".wav", "")
        lines = read_lines_textfile(file_name)
        lines_last_character = lines[-1] #末字
        if lines_last_character == "/":
            lines = lines.replace("/", "")
        if lines_last_character == "\\":
            lines = lines.replace("\\", "")
        # if index_enable == "a": #昇順
        #     lines = f"{count}{lines}"
        # if index_enable == "d": #降順
        #     lines = f"{file_number}{lines}"
        #     file_number -= 1
        voice_file_path = os.path.join(renamed_files_path, f"{lines}.wav")
        os.rename(file, voice_file_path)
        count += 1

    print(f"{count}個のファイルの名称を変更しました。")
    print("もう一度使用しますか?(y/n)")
    if input() != "y":
        break
