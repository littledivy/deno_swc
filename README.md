<br />
<p align="center">
  <a href="https://github.com/divy-beta/deno_swc">
    <img src="https://raw.githubusercontent.com/nestdotland/deno_swc/master/assets/deno_swc.png" alt="deno_swc logo" width="310">
  </a>
  <h3 align="center">deno_swc</h3>

  <p align="center">
    High quality bindings to the swc for Deno.
 </p>
</p>

> 🌀 The library is not yet stable. Expect breaking changes.

![e2e-test](https://github.com/nestdotland/deno_swc/workflows/e2e-test/badge.svg)
![dev-ci](https://github.com/nestdotland/deno_swc/workflows/dev-ci/badge.svg)
![release](https://github.com/nestdotland/deno_swc/workflows/release/badge.svg)
![](https://img.shields.io/github/v/release/nestdotland/deno_swc?style=plastic)

## Usage

```typescript
import { parseTypescript } from "https://x.nest.land/swc@0.3.0-rc.1/mod.ts";

parseTypescript(`const x: string = "Hello, Deno SWC!"`);

// Output AST
{
  type: "ok",
  value: {
    type: "Module",
    span: { start: 0, end: 36, ctxt: 0 },
    body: [
      {
        type: "VariableDeclaration",
        span: [Object],
        kind: "const",
        declare: false,
        declarations: [Array]
      }
    ],
    interpreter: null
  }
}
```

## Folder structure
- `swc_core` - core `swc` methods.
- `swc_deno` - applying types and options to plugin ops.
- `swc_plugin` - loading plugin ops and dispatch them.
- `src` - `swc_core` bindings to Deno.

## Copyright

deno_swc is licensed under the MIT license. Please see the [LICENSE](LICENSE) file.
