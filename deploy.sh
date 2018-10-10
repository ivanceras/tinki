
set -v
cargo web deploy --release
wasm-opt -Oz -o ./target/deploy/tinki.wasm ./target/deploy/tinki.wasm
cp -r ./target/deploy/* ../ivanceras.github.io/
