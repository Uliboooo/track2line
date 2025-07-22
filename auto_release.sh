#!/bin/bash
# set -euxo pipefail

# いつかpackage nameを取得
project_name="t2l"

#build
cargo fmt

cargo build --release
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

rm -r release/
mkdir release/

cp ./target/release/track2line ./target/release/t2l
cp ./target/x86_64-unknown-linux-gnu/release/track2line ./target/x86_64-unknown-linux-gnu/release/t2l
cp ./target/x86_64-pc-windows-gnu/release/track2line.exe ./target/x86_64-pc-windows-gnu/release/t2l.exe 

zip -j ./release/${project_name}_arm_mac_$1.zip ./README.md ./target/release/t2l
zip -j ./release/${project_name}_linux_x86_64_$1.zip ./README.md ./target/x86_64-unknown-linux-gnu/release/t2l
zip -j ./release/${project_name}_win_x86_64_$1.zip ./README.md ./target/x86_64-pc-windows-gnu/release/t2l.exe

