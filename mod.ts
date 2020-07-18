import {
  compile as rs_compile
} from "./swc_plugin/index.ts";

export function compile(str: string) {
  return rs_compile(str);
}