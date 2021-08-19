import {
  parseSync,
  printSync,
  // source,
  transformSync,
} from "./swc_wasm/wasm.js";

import {
  Config,
  ParseOptions,
  Program,
  TransformConfig,
} from "./types/options.ts";

//const start = performance.now();
// await init(source);
//const end = performance.now() - start;
//console.log(end)

export function parse(source: string, opts: ParseOptions): Program {
  return parseSync(source, opts);
}

// TODO(littledivy): Typings for `program`
// deno-lint-ignore no-explicit-any
export function print(program: any, opts?: Config): { code: string } {
  return printSync(program, opts || {});
}

export function transform(
  source: string,
  opts: TransformConfig,
): { code: string } {
  return transformSync(source, opts);
}
