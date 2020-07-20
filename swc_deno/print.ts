import {
  swc_print,
} from "../swc_plugin/index.ts";

export function print(
  opt: object,
): { code: string } {
  const result = JSON.parse(swc_print(opt));
  return result;
}
