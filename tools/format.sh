#!/bin/sh
deno fmt --ignore=./swc_wasm/ --unstable
cd swc_wasm
cargo fmt --all
