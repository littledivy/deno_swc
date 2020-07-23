import { transform, parse } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("transform (no error)", () => {
  const result = transform("const x: number = 2;");
  assertEquals(result.code.trim(), "'const x: number = 2;';");
});
