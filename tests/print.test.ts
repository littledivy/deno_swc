import { print } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("print (no error)", () => {
  const result = print({
    "type": "Module",
    "span": { "start": 21, "end": 33, "ctxt": 0 },
    "body": [{
      "type": "ClassDeclaration",
      "identifier": {
        "type": "Identifier",
        "span": { "start": 27, "end": 28, "ctxt": 0 },
        "value": "X",
        "optional": false,
      },
      "declare": false,
      "span": { "start": 21, "end": 32, "ctxt": 0 },
      "decorators": [],
      "body": [],
      "superClass": null,
      "isAbstract": false,
      "typeParams": null,
      "superTypeParams": null,
      "implements": [],
    }, {
      "type": "EmptyStatement",
      "span": { "start": 32, "end": 33, "ctxt": 0 },
    }],
    "interpreter": null,
  }, {});
  assertEquals(result.code.trim(), "class X {\n}\n;");
});
