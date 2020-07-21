import { transform, parseTypescript } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("transform (no error)", () => {
  // @ts-ignore
  const result = transform("const x: number = 2;")
  console.log(result)
  assertEquals(result.code.trim(), "'const x: number = 2;';");
});
