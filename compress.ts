import { compress } from "https://deno.land/x/lz4@v0.1.2/mod.ts";

const name = "./lib/deno_swc_bg.wasm";
Deno.writeFileSync(name, compress(Deno.readFileSync(name)));
