import {
  swc_extract_dependencies,
} from "../swc_plugin/index.ts";

import { AnalyzeOptions } from "../types/options.ts";
import { Result } from "../types/result.ts";

/**
 * Extract the dependencies of a Typescript/Javascript code. 
 * Note: dependencies of dependencies will not be resolved.
 */
export const extractDependencies = (
  opt: AnalyzeOptions,
): Result<{ ok: string[]; error: string }> => {
  const result = JSON.parse(swc_extract_dependencies(opt));
  if (typeof result === "string") {
    return {
      type: "error",
      error: result,
    };
  } else {
    return {
      type: "ok",
      value: result,
    };
  }
};
