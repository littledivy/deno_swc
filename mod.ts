import { instantiate } from "./lib/deno_swc.generated.js";
import { decompress } from "https://deno.land/x/lz4@v0.1.2/mod.ts";
import {
  Config,
  ParseOptions,
  Program,
  TransformConfig,
} from "./types/options.ts";

const { parseSync, printSync, transformSync } = await instantiate(decompress);

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
