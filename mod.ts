import init, {
  parseSync,
  printSync,
  source,
  transformSync,
} from "./swc_wasm/wasm.js";

import { ParseOptions, Config, TransformConfig, Program } from "./types/options.ts";

await init(source);

export function parse(source: string, opts: ParseOptions): Program {
  return parseSync(source, opts);
}

// TODO(littledivy): Typings for `program`
export function print(program: any, opts: Config): { code: string } {
  return printSync(program, opts);
}

export function transform(source: string, opts: TransformConfig): { code: string } {
  return transformSync(source, opts);
}
