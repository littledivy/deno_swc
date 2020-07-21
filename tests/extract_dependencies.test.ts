import { extractDependencies } from "../swc_deno/extract_dependencies.ts";
import { assertEquals } from "./deps.ts";

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

Deno.test("getDependencies (dynamic imports)", () => {
  const result = extractDependencies(
    {
      src: `
        import * as B from "./b.ts"; 
        await import("./a.ts"); 
        console.log(B);
      `,
      dynamic: true,
    },
  );
  assertEquals(result, {
    type: "ok",
    value: [
      "./b.ts",
      "./a.ts",
    ],
  });
});

Deno.test("getDependencies (parse error)", () => {
  const result = extractDependencies(
    {
      src: `this sentence is no valid javascript`,
      dynamic: false,
    },
  );
  assertEquals(result, { type: "error", error: "parse_error" });
});
