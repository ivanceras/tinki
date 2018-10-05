
set -v
cargo web deploy
wasm-opt -Oz -o ./target/deploy/tinki.wasm ./target/deploy/tinki.wasm
cp ./target/deploy/* ../ivanceras.github.io/
