<br />
<p align="center">
  <a href="https://github.com/divy-beta/deno_swc">
    <img src="https://raw.githubusercontent.com/nestdotland/deno_swc/master/assets/deno_swc.png" alt="deno_swc logo" width="310">
  </a>
  <h3 align="center">deno_swc</h3>

  <p align="center">
    The SWC compiler for Deno.
 </p>
</p>

![e2e-test](https://github.com/nestdotland/deno_swc/workflows/e2e-test/badge.svg)
![dev-ci](https://github.com/nestdotland/deno_swc/workflows/dev-ci/badge.svg)
![](https://img.shields.io/github/v/release/nestdotland/deno_swc?style=plastic)

## Usage

```typescript
import { parse, print } from "https://x.nest.land/swc@0.0.6/mod.ts";

const ast = parse(`const x: string = "Hello, Deno SWC!"`,{
  syntax: "typescript"
});

// {
//   type: "Module",
//   span: { start: 0, end: 36, ctxt: 0 },
//   body: [
//     {
//       type: "VariableDeclaration",
//       span: [Object],
//       kind: "const",
//       declare: false,
//       declarations: [Array]
//     }
//   ],
//   interpreter: null
// }

let code = print(ast, {
  minify: true,
  isModule: true,
}).code;

// const x: string = "Hello, Deno SWC!";
```

## Copyright

deno_swc is licensed under the MIT license. Please see the [LICENSE](LICENSE) file.
