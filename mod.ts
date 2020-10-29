import init, {
  parseSync,
  printSync,
  source,
  transformSync,
} from "./swc_wasm/wasm.js";

import { ParseOptions } from "./types/options.ts";

await init(source);

export function parse(source: string, opts: ParseOptions) {
  return parseSync(source, opts);
}

export function print(program: any, opts: any) {
  return printSync(program, opts);
}

export function transform(source: string, opts: any) {
  return transformSync(source, opts);
}
