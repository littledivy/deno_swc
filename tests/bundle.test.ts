import { bundle } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("bundle (no error)", () => {
  const result = bundle({
    entry: [Deno.cwd() + "/tests/test_files/bundle_me.ts"],
  });
  assertEquals(result[Object.keys(result)[0]].code.trim(), "export var z = 3;");
});
