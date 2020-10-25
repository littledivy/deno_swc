import { transform } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("transform (no error)", () => {
  const result = transform("const x: number = 2;", {});
  assertEquals(result.code.trim(), "var x;");
});
