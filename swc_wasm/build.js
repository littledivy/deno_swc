import { encode } from "https://deno.land/std@0.103.0/encoding/base64.ts";
import Terser from "https://esm.sh/terser@4.8.0";
import * as lz4 from "https://deno.land/x/lz4@v0.1.2/mod.ts";

const name = "deno_swc";

const encoder = new TextEncoder();

async function requires(...executables) {
  const where = Deno.build.os === "windows" ? "where" : "which";

  for (const executable of executables) {
    const process = Deno.run({
      cmd: [where, executable],
      stderr: "null",
      stdin: "null",
      stdout: "null",
    });

    if (!(await process.status()).success) {
      err(`Could not find required build tool ${executable}`);
    }
  }
}

async function run(msg, cmd) {
  log(msg);

  const process = Deno.run({ cmd });

  if (!(await process.status()).success) {
    err(`${msg} failed`);
  }
}

function log(text) {
  console.log(`[log] ${text}`);
}

function err(text) {
  console.log(`[err] ${text}`);
  return Deno.exit(1);
}

await requires("rustup", "rustc", "cargo", "wasm-pack");

if (!(await Deno.stat("Cargo.toml")).isFile) {
  err(`the build script should be executed in the "${name}" root`);
}

await run("building wasm", ["cargo", "build", "--release", "--target", "wasm32-unknown-unknown"]);

await run(
  "building using wasm-pack",
  /// ["wasm-pack", "build", "--target", "deno", "--weak-refs", "--release"],
  ["wasm-bindgen", "target/wasm32-unknown-unknown/release/deno_swc.wasm" , "--target", "deno", "--weak-refs", "--out-dir", "pkg/"],
);

const wasm = await Deno.readFile(`pkg/${name}_bg.wasm`);

const compressed = lz4.compress(wasm);
console.log(
  `compressed wasm using lz4      (reduction: ${wasm.length -
    compressed.length} bytes, size: ${compressed.length} bytes)`,
);

const encoded = encode(compressed);

log(
  `encoded wasm using base64, size increase: ${encoded.length -
    wasm.length} bytes`,
);

log("inlining wasm in js");
const source = `import * as lz4 from "https://deno.land/x/lz4@v0.1.2/mod.ts";export const source=lz4.decompress(Uint8Array.from(atob("${encoded}"),c=>c.charCodeAt(0)));`;

let init = await Deno.readTextFile(`pkg/${name}.js`);
let lines = init.split('\n');
// We want to replace this code.
for (let i = 1; i < 4; i++) lines.splice(-i);
init = lines.join('\n');
init += `\nconst wasmModule = new WebAssembly.Module(source);\nconst wasmInstance = new WebAssembly.Instance(wasmModule, imports);\nconst wasm = wasmInstance.exports;\n`;
console.log(init)

log("minifying js");
const output = Terser.minify(`${source}\n${init}`, {
  mangle: { module: true },
  output: {
    preamble: "//deno-fmt-ignore-file",
  },
});

if (output.error) {
  err(`encountered error when minifying: ${output.error}`);
}

const reduction = new Blob([(`${source}\n${init}`)]).size -
  new Blob([output.code]).size;
log(`minified js, size reduction: ${reduction} bytes`);

log(`writing output to file ("wasm.js")`);
await Deno.writeFile("wasm.js", encoder.encode(output.code));

const outputFile = await Deno.stat("wasm.js");
log(
  `output file ("wasm.js"), final size is: ${outputFile.size} bytes`,
);
