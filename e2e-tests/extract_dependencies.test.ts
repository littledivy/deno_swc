import { assertEquals } from "../tests/deps.ts";
import { extractDependencies } from "./deps.ts";

Deno.test("getDependencies", () => {
  const result = extractDependencies(
    { src: `import * as B from "./b.ts"; console.log(B);`, dynamic: false },
  );
  assertEquals(result, {
    type: "ok",
    value: [
      "./b.ts",
    ],
  });
});
