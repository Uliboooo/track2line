#!/bin/bash
# set -euxo pipefail

# いつかpackage nameを取得
project_name="track2line"



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

cp ./target/release/${project_name} ./release/${project_name}_mac_arm_$1
cp ./target/x86_64-apple-darwin/release/${project_name} ./release/${project_name}_mac_x86_64_$1
cp ./target/x86_64-unknown-linux-gnu/release/${project_name} ./release/${project_name}_linux_x86_64_$1
cp ./target/x86_64-pc-windows-gnu/release/${project_name}.exe ./release/${project_name}_win_x86_64_$1.exe

zip -j -9 ./release/${project_name}_mac_arm_$1.zip ./README.md ./release/${project_name}_mac_arm_$1
zip -j -9 ./release/${project_name}_mac_x86_64_$1.zip ./README.md ./release/${project_name}_mac_x86_64_$1
zip -j -9 ./release/${project_name}_linux_x86_64_$1.zip ./README.md ./release/${project_name}_linux_x86_64_$1
zip -j -9 ./release/${project_name}_win_x86_64_$1.zip ./README.md ./release/${project_name}_win_x86_64_$1.exe
