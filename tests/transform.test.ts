import { transform } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("transform (no error)", () => {
  const result = transform("const x: number = 2; console.log(x);", {
    // deno-lint-ignore ban-ts-comment
    // @ts-ignore
    "jsc": {
      "target": "es2016",
      "parser": {
        "syntax": "typescript",
      },
    },
  });
  assertEquals(result.code.trim(), "const x = 2;\nconsole.log(x);");
});
