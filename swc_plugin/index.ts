import { Plug } from "../deps.ts";
import { ParseOptions, AnalyzeOptions } from "../types/options.ts";
import { version } from "../version.ts";

const PLUG_NAME = "deno_swc";

const DEV_ENV = Deno.env.get("DEV");

const PLUGIN_URL_BASE = DEV_ENV
  ? "./target/debug/"
  : `https://github.com/nestdotland/deno_swc/releases/download/${version}`;

const rid = await Plug.prepare({
  name: PLUG_NAME,
  url: PLUGIN_URL_BASE,
  policy: !DEV_ENV ? Plug.CachePolicy.STORE : Plug.CachePolicy.NONE,
});

const {
  parse,
  extract_dependencies,
  print,
  transform,
  bundle,
} = Plug.core.ops();

const textDecoder = new TextDecoder();
const textEncoder = new TextEncoder();

export function swc_print(opt: object) {
  const response = Plug.core.dispatch(
    print,
    textEncoder.encode(JSON.stringify(opt)),
  );
  return JSON.parse(textDecoder.decode(response));
}

export function swc_parse_ts(opt: { src: string; opt?: ParseOptions }) {
  const response = Plug.core.dispatch(
    parse,
    textEncoder.encode(JSON.stringify(opt)),
  );
  return JSON.parse(textDecoder.decode(response));
}

export function swc_extract_dependencies(opt: AnalyzeOptions) {
  const response = Plug.core.dispatch(
    extract_dependencies,
    textEncoder.encode(JSON.stringify(opt)),
  );
  return JSON.parse(textDecoder.decode(response));
}

export function swc_transform(src: string) {
  const response = Plug.core.dispatch(
    transform,
    textEncoder.encode(JSON.stringify(src)),
  );
  return JSON.parse(textDecoder.decode(response));
}

export function swc_bundle(cnf: object) {
  const response = Plug.core.dispatch(
    bundle,
    textEncoder.encode(JSON.stringify(cnf)),
  );
  return JSON.parse(textDecoder.decode(response));
}
