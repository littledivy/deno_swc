import {
  swc_transform,
} from "../swc_plugin/index.ts";

export function transform(
  src: string,
) {
  const result = JSON.parse(swc_transform(opt));
  return result;
}
