import { parse, print } from "https://raw.githubusercontent.com/tamusjroyce/deno_swc/master/mod.ts";

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

const ast = parse(code, { target: "es2019", syntax: "typescript" });
const regeneratedCode = print(ast, {
  minify: true,
  module: {
    type: "commonjs"
  },
}).code;

console.log(code);
console.log(''); console.log(''); console.log('');
console.log(ast);
console.log(''); console.log(''); console.log('');
console.log(regeneratedCode);

// interface H{h:string;}const x:string=`Hello, ${"Hello"} Deno SWC!`;switch(x){case "value":console.log(x);break;default:break}
