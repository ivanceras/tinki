set -v

sh ./deploy.sh
cd server
cargo run --release

