import { parseTypescript, print } from "https://x.nest.land/swc@0.3.0-rc.1/mod.ts";

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
  // @ts-ignore
  print({ program: parseTypescript(code, { target: "es2020", syntax: "typescript"}).value,
    options: {
      minify: true,
      isModule: true
    }
  }).code
)
