cd "$(dirname "$0")"
# $USER may or may not be set
USERNAME="$(id -u -n)"
RUSTFLAGS="--remap-path-prefix /home/$USERNAME=" cargo build --release
mv ./target/release/preprocessor ../preprocessor_linux