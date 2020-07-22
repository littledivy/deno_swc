#!/bin/sh
deno fmt swc_deno/ mod.ts deps.ts types/ tests/ tools/
cargo fmt --all
