import { transform } from "../mod.ts";

const result = transform("const x: number = 2;", {});
console.log(result);
