<br />
<p align="center">
  <a href="https://github.com/littledivy/deno_swc">
    <img src="https://raw.githubusercontent.com/littledivy/deno_swc/master/assets/deno_swc.png" alt="deno_swc logo" width="310">
  </a>
  <h3 align="center">deno_swc</h3>

<p align="center">
    The SWC compiler for Deno.
 </p>
</p>

![ci](https://github.com/littledivy/deno_swc/workflows/ci/badge.svg)
![](https://img.shields.io/github/v/release/littledivy/deno_swc?style=plastic)

# Usage

`parse()`

```typescript
import { parse, print } from "https://deno.land/x/swc@0.2.0/mod.ts";

const code = `const x: string = "Hello, Deno SWC!"`;

const ast = parse(code, {
  target: "es2019",
  syntax: "typescript",
  comments: false,
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
```

`print()`

```typescript
const { code } = print(ast, {
  minify: true,
  module: {
    type: "commonjs",
  },
});

// const x = "Hello, Deno SWC!"
```

...and `transform()`

```typescript
const { code } = transform("const x: number = 2;", {
  jsc: {
    target: "es2016",
    parser: {
      syntax: "typescript",
    },
  },
});

// const x = 2;
```

## Copyright

deno_swc is licensed under the MIT license. Please see the [LICENSE](LICENSE)
file.
