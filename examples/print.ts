import { parse, print } from "../mod.ts";

const code = `
interface H {
  h: string;
}

const x: string = \`Hello, $\{"Hello"} Deno SWC!\`;

switch (x) {
  case "value":
    console.log(x);
    break;

  default:
    break;
}
`;

const ast = parse(code, {
  target: "es2019",
  syntax: "typescript",
  comments: false,
});
const regeneratedCode = print(ast, {
  minify: true,
  module: {
    type: "commonjs",
  },
}).code;

console.log(regeneratedCode);
