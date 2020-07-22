import {
  swc_bundle,
} from "../swc_plugin/index.ts";

export function bundle(
  src: string,
) {
  const result = JSON.parse(swc_bundle(src));
  return result;
}
