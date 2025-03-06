#!/bin/bash
# set -euxo pipefail

# いつかpackage nameを取得
project_name="track2line"

# paths=(
#     "./target/release/$project_name"
#     "./target/x86_64-apple-darwin/release/${project_name}"
#     "./target/x86_64-unknown-linux-gnu/release/${project_name}"
#     "./target/x86_64-pc-windows-gnu/release/${project_name}.exe"
# )

#build
cargo fmt

# brew update
# brew upgrade

cargo build --release
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu

rm -r release/
mkdir release/

zip -j -9 ./release/${project_name}_mac_arm.zip ./README.md ./target/release/${project_name}
zip -j -9 ./release/${project_name}_mac_x86_64.zip ./README.md ./target/x86_64-apple-darwin/release/${project_name}
zip -j -9 ./release/${project_name}_linux_x86_64.zip ./README.md ./target/x86_64-unknown-linux-gnu/release/${project_name}
zip -j -9 ./release/${project_name}_win_x86_64.zip ./README.md ./target/x86_64-pc-windows-gnu/release/${project_name}.exe

# for file_path in "${paths[@]}"
# do
#     echo ${file_path}
#     zip x86_64_linux.zip 
# done