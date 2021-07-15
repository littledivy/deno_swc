#!/bin/sh
deno lint --ignore=./builds/,./swc_wasm/ --unstable
cargo +nightly clippy
