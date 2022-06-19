import { parse } from "../mod.ts";

const start = performance.now();
console.log(parse(
  `
  import * as a from "./a.ts";
`,
  {
    syntax: "ecmascript",
  },
));
const end = performance.now() - start;
console.log(`parse time: ${end}ms`);
