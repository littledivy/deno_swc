import init, {
  parseSync,
  printSync,
  source,
  transformSync,
} from "./swc_wasm/wasm.js";

await init(source);

export function parse(source: string, opts: any) {
  return parseSync(source, opts);
}

export function print(program: any, opts: any) {
  return printSync(program, opts);
}

export function transform(source: string, opts: any) {
  return transformSync(source, opts);
}
