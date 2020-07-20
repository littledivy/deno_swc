#!/bin/sh
deno fmt swc_deno/ mod.ts deps.ts types/
cargo fmt --all
