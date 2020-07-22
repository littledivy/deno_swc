import { bundle } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("bundle (no error)", () => {
  const result = bundle({
    working_dir: Deno.cwd(),
    entry: ["mod.ts"],
  });
  console.log(result);
});
