
set -v
wasm-pack build --target no-modules --release

cp -r ./pkg ../ivanceras.github.io/
cp index.html ../ivanceras.github.io/
cp index.md ../ivanceras.github.io/
cp minimal.css ../ivanceras.github.io/

