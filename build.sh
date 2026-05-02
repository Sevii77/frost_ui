# ./tools/preprocessor_linux "./vectors" "./" "./meta.yaml" "./raw"
RUSTFLAGS="-l dylib=stdc++" cargo run --manifest-path=./tools/preprocessor/Cargo.toml --release -- "./vectors" "./" "./meta.yaml" "./raw"
# aetherment pack "./" > /dev/null
aetherment pack "./" > ./pack.log
echo finished