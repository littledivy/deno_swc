import { AnalyzeOptions } from "../types/options.ts";
import { Result } from "../types/result.ts";

export const extractDependencies = (
  swc_extract_dependencies: (opt: AnalyzeOptions) => string,
) =>
  (opt: AnalyzeOptions): Result<{ ok: string[]; error: string }> => {
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
