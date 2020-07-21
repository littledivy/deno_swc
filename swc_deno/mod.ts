import { initPlugin } from "../swc_plugin/index.ts";
import { parseTypescript } from "./parse.ts";
import { print } from "./print.ts";
import { extractDependencies } from "./extract_dependencies.ts";

export const DenoSWC = async ({
  pluginVersion,
}: {
  /**
   * Under normal circumstances, this value should be same as 
   * the version of DenoSWC.  
   * 
   * For example: "v0.0.1"
   */
  pluginVersion: string;
}) => {
  const { swc_extract_dependencies, swc_parse_ts, swc_print } =
    await initPlugin(
      {
        releaseTag: pluginVersion,
      },
    );
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
