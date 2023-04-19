import { decompress } from "https://deno.land/x/lz4@v0.1.2/mod.ts";
import type {
  Config,
  ParseOptions,
  Program,
} from "https://esm.sh/@swc/core@1.2.212/types.d.ts";
import { instantiate } from "./lib/deno_swc.generated.js";

const { parseSync, printSync, transformSync, minifySync } = await instantiate(decompress);

export function parse(source: string, opts: ParseOptions): Program {
  return parseSync(source, opts);
}

export function print(program: Program, opts?: Config): { code: string } {
  return printSync(program, opts || {});
}

export function transform(source: string, opts: Config): { code: string } {
  return transformSync(source, opts);
}

export function minify(source: string, opts?: Config): { code: string } {
  return minifySync(source, opts);
}

export * from "https://esm.sh/@swc/core@1.2.212/types.d.ts";