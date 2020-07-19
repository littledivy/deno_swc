import {
  swc_extract_dependencies,
} from "../swc_plugin/index.ts";

import { AnalyzeOptions } from "../types/options.ts";

/**
 * Extract the dependencies of a Typescript/Javascript code. 
 * Note: dependencies of dependencies will not be resolved.
 */
export const extractDependencies = (opt: AnalyzeOptions): string[] => {
  return JSON.parse(swc_extract_dependencies(opt));
};
