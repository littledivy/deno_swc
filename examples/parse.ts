import { parse } from "../mod.ts";

console.log(parse(
  `
  import * as a from "./a.ts";
`,
  {
    syntax: "ecmascript",
  },
));
const end = performance.now() - start;
console.log(end);
