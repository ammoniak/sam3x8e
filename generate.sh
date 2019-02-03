rm lib.rs
rm -rf ./src/*
../svd2rust/target/debug/svd2rust --nightly -i ATSAM3X8E.svd
form -i lib.rs -o ./src/
cargo fmt
cargo build
