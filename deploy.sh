
set -v
cargo web deploy --release
cp -r ./target/deploy/* ../ivanceras.github.io/
