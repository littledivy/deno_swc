import init, {
    source,
    parseSync,
} from "./swc_wasm/wasm.js";

await init(source);

export function parse(source: string, opts: any) { 
    parseSync(source, opts); 
};

console.log(parseSync(
    "import * as x from 'x'",
    {
        "syntax": "ecmascript",
        "jsx": true,
        "dynamicImport": false,
        "numericSeparator": false,
        "privateMethod": false,
        "functionBind": false,
        "exportDefaultFrom": false,
        "exportNamespaceFrom": false,
        "decorators": false,
        "decoratorsBeforeExport": false,
        "topLevelAwait": false,
        "importMeta": false
      }
))