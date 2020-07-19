import { getLocalDependencies } from "../swc_deno/get_local_dependencies.ts";
import { assertEquals } from "./deps.ts";

Deno.test("getDependencies", () => {
  const result = getLocalDependencies(
    { src: `import * as B from "./b.ts"; console.log(B);`, dynamic: false },
  );
  assertEquals(result, [
    "./b.ts",
  ]);
});

Deno.test("getDependencies (dynamic imports)", () => {
  const result = getLocalDependencies(
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
