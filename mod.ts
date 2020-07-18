import {
  swc_parse_ts,
} from "./swc_plugin/index.ts";
import { ParseOptions } from "./types/options.ts";

export function parse(opt: ParseOptions) {
  return JSON.parse(swc_parse_ts(opt));
}
