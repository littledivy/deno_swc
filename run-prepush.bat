#!/Windows/system32/cmd
@echo off

deno run ./tools/print-version.ts

.\\call .\\tools\\lint-local.sh
.\\call .\\tools\\format-local.sh
.\\call .\\tools\\dev-test.sh
.\\call .\\tools\\e2e-test.sh

echo .
cat .\\tools\\README.windows.md
echo .

REM TODO: remove lint-local.sh and format-local.sh after getting them to `cd swc-wasm`, run rust tools, `cd ..` without breaking CI 
cd swc_wasm
cargo +nightly clippy
cargo fmt --all
deno run -A ./build.js
cd ..

echo completed...
pause
