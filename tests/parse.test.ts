import { parse } from "../mod.ts";
import { assertEquals } from "./deps.ts";

Deno.test("parse (no error)", () => {
  const result = parse("const x: number = 2;", {
    "syntax": "typescript",
  });
  assertEquals(result, {
    type: "Module",
    body: [
      {
        declarations: [
          {
            definite: false,
            id: {
              optional: false,
              span: { ctxt: 0, end: 63, start: 62 },
              type: "Identifier",
              typeAnnotation: {
                span: { ctxt: 0, end: 71, start: 63 },
                type: "TsTypeAnnotation",
                typeAnnotation: {
                  kind: "number",
                  span: { ctxt: 0, end: 71, start: 65 },
                  type: "TsKeywordType",
                },
              },
              value: "x",
            },
            init: {
              span: { ctxt: 0, end: 75, start: 74 },
              type: "NumericLiteral",
              value: 2,
            },
            span: { ctxt: 0, end: 75, start: 62 },
            type: "VariableDeclarator",
          },
        ],
        declare: false,
        kind: "const",
        span: { ctxt: 0, end: 76, start: 56 },
        type: "VariableDeclaration",
      },
    ],
    interpreter: null,
    span: { ctxt: 0, end: 76, start: 56 },
  });
});
