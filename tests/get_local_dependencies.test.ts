import { getLocalDependencies } from "../swc_deno/get_local_dependencies.ts";
import { assertEquals } from "./deps.ts";

Deno.test("getDependencies (import from same directory)", () => {
  const result = getLocalDependencies("./tests/test_files/a.ts");
  assertEquals(result, [
    "./tests/test_files/a.ts",
    "./tests/test_files/b.ts",
  ]);
});

Deno.test("getDependencies (import from child directory)", () => {
  const result = getLocalDependencies("./tests/test_files/c.ts");
  assertEquals(result, [
    "./tests/test_files/c.ts",
    "./tests/test_files/d.ts",
  ]);
});

Deno.test("getDependencies (import from parent directory)", () => {
  const result = getLocalDependencies("./tests/test_files/e.ts");
  assertEquals(result, [
    "./tests/test_files/e.ts",
    "./tests/test_files/f.ts",
  ]);
});

Deno.test("getDependencies (circular imports)", () => {
  const result = getLocalDependencies("./tests/test_files/g.ts");
  assertEquals(result, [
    "./tests/test_files/g.ts",
    "./tests/test_files/h.ts",
  ]);
});

Deno.test("getDependencies (import from remote)", () => {
  throw new Error(`Unspecified yet`);
});

Deno.test("getDependencies (import map)", () => {
  throw new Error(`Unspecified yet`);
});

Deno.test("getDependencies (real world example: oak)", () => {
  throw new Error(`Unspecified yet`);
});
