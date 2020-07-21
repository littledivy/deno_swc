import {
  swc_parse_ts,
} from "../swc_plugin/index.ts";
import { ParseOptions } from "../types/options.ts";
import { Result } from "../types/result.ts";

type ParseResult =
  | { type: "ok"; ast: Record<string, string> }
  | { type: "error"; error: string };

export function parseTypescript(
  opt: ParseOptions,
): Result<{ ok: Record<string, string>; error: string }> {
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
}
