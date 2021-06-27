import { parse, print } from "../mod.ts";

const code: string = `
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

console.log(
  print(parse(code, { target: "es2019", syntax: "typescript" }), {
    minify: true,
    module: {
      type: "commonjs"
    },
  }).code,
);
