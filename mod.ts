import init, {
    source,
    parseSync,
} from "./swc_wasm/wasm.js";

await init(source);

export function parse(source: string, opts: any) { 
    return parseSync(source, opts); 
};
