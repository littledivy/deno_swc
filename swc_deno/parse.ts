import { ParseOptions } from "../types/options.ts";
import { Result } from "../types/result.ts";

type ParseResult = Result<{ ok: Record<string, string>; error: string }>;

export const parseTypescript = (swc_parse_ts: (opt: ParseOptions) => string) =>
  (opt: ParseOptions): ParseResult => {
    const result = JSON.parse(swc_parse_ts(opt));
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
