#!/bin/sh

deno run ./tools/print-version.ts

./tools/lint-local.sh
./tools/format-local.sh
./tools/dev-test.sh
./tools/e2e-test.sh

echo ""
cat .\\tools\\README.posix.md
echo ""

# TODO: remove lint-local.sh and format-local.sh after getting them to `cd swc-wasm`, run rust tools, `cd ..` without breaking CI 
cd swc_wasm
cargo +nightly clippy
cargo fmt --all
deno run -A ./build.js
cd ..

echo completed...
