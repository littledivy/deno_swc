import {
  swc_parse
} from "./swc_plugin/index.ts";

export function parse(str: string) {
  return swc_parse(str);
}
