import {
  swc_parse
} from "./swc_plugin/index.ts";
import { ParseOptions } from "./types/options.ts";

export function parse(opt: ParseOptions) {
  return swc_parse(opt);
}
