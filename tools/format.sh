#!/bin/sh
deno fmt --ignore=./builds/,./swc_wasm/ --unstable
cd swc_wasm
cargo fmt --all
