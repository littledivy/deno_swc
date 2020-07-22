import {
  swc_bundle,
} from "../swc_plugin/index.ts";

export function bundle(
  cnf: object,
) {
  const result = JSON.parse(swc_bundle(cnf));
  return result;
}
