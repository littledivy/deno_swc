import { DenoSWC } from "./swc_deno/mod.ts";

export const {
  parseTypescript,
  print,
  extractDependencies,
} = await DenoSWC({ importMetaUrl: import.meta.url });
