import { extractDependencies } from "../swc_deno/extract_dependencies.ts";
import { assertEquals } from "./deps.ts";

Deno.test("getDependencies", () => {
  const result = extractDependencies(
    { src: `import * as B from "./b.ts"; console.log(B);`, dynamic: false },
  );
  assertEquals(result, [
    "./b.ts",
  ]);
});

Deno.test("getDependencies (dynamic imports)", () => {
  const result = extractDependencies(
    {
      src:
        `import * as B from "./b.ts"; await import("./a.ts"); console.log(B);`,
      dynamic: true,
    },
  );
  assertEquals(result, [
    "./b.ts",
    "./a.ts",
  ]);
});
