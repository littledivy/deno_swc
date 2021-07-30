import { transform } from "../mod.ts";

const { code } = transform("const x: number = 2;", {
  // @ts-ignore: TransformConfig typings for swc_wasm and node_swc are different
  "jsc": {
    "target": "es2016",
    "parser": {
      "syntax": "typescript",
    },
  },
});

console.log(code);
