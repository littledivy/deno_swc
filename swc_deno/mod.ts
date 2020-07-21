import { initPlugin } from "../swc_plugin/index.ts";
import { parseTypescript } from "./parse.ts";
import { print } from "./print.ts";
import { extractDependencies } from "./extract_dependencies.ts";

export const DenoSWC = async ({
  importMetaUrl,
}: {
  importMetaUrl: string;
}) => {
  const { swc_extract_dependencies, swc_parse_ts, swc_print } =
    await initPlugin({ importMetaUrl });
  return {
    parseTypescript: parseTypescript(swc_parse_ts),
    print: print(swc_print),

    /**
    * Extract the dependencies of a Typescript/Javascript code. 
    * Note: dependencies of dependencies will not be resolved.
    */
    extractDependencies: extractDependencies(swc_extract_dependencies),
  };
};
