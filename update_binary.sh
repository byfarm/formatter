sudo rm /usr/bin/ucfm
cargo build --release
sudo link ./target/release/unicode-formatter /usr/bin/ucfm
