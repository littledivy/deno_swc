import { analyze } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("getDependencies", () => {
  const result = analyze(
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
  const result = analyze(
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
  const result = analyze(
    {
      src: `this sentence is no valid javascript`,
      dynamic: false,
    },
  );
  assertEquals(result, { type: "error", error: "parse_error" });
});
