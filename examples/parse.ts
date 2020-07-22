import { parseTypescript } from "https://x.nest.land/swc@0.3.0-rc.1/mod.ts";

console.log(parseTypescript(`
  import * as a from "./a.ts";
`));
