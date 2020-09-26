#!/bin/sh

# TODO: Just put the expected dependencies in a list and iterate over that

SVD2RUST_CMD=$(which svd2rust)
if [ x"${SVD2RUST_CMD}" = "x" ]; then
    echo "Dependency ‘svd2rust’ does not appear to be installed.  Try: cargo install svd2rust"
    exit 1
fi

FORM_CMD=$(which form)
if [ x"${FORM_CMD}" = "x" ]; then
    echo "Dependency ‘form’ does not appear to be installed.  Try: cargo install form"
    exit 1
fi

rm -f lib.rs
rm -rf ./src/*

svd2rust -i ATSAM3X8E.svd
if [ x"$?" != "x0" ]; then
    echo "svd2rust failed, aborting"
    exit 1
fi

form -i lib.rs -o ./src/
if [ x"$?" != "x0" ]; then
    echo "form failed, aborting"
    exit 1
fi

echo "Running cargo fmt"
cargo fmt
if [ x"$?" != "x0" ]; then
    echo "cargo fmt failed, aborting"
    exit 1
fi

# Cleanup
rm -f lib.rs

echo "Building crate"
cargo build
