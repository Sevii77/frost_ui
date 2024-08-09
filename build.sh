# ./tools/preprocessor_linux "./vectors" "./" "./meta.yaml" "./raw"
RUSTFLAGS="--remap-path-prefix $CARGO_HOME=" cargo run --manifest-path=./tools/preprocessor/Cargo.toml --release -- "./vectors" "./" "./meta.yaml" "./raw"
# aetherment pack "./" > /dev/null
aetherment pack "./" > ./pack.log
echo finished