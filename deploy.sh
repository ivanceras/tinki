
set -v
wasm-pack build --target no-modules --release

dest_dir="../ivanceras.github.io/"

cp -r ./pkg "${dest_dir}"
cp index.html "${dest_dir}"
cp index.md "${dest_dir}"
cp minimal.css "${dest_dir}"
rm "${dest_dir}pkg/.gitignore"

