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
              span: { ctxt: 0, end: 8, start: 7 },
              type: "Identifier",
              typeAnnotation: {
                span: { ctxt: 0, end: 16, start: 8 },
                type: "TsTypeAnnotation",
                typeAnnotation: {
                  kind: "number",
                  span: { ctxt: 0, end: 16, start: 10 },
                  type: "TsKeywordType",
                },
              },
              value: "x",
            },
            init: {
              raw: "2",
              span: { ctxt: 0, end: 20, start: 19 },
              type: "NumericLiteral",
              value: 2,
            },
            span: { ctxt: 0, end: 20, start: 7 },
            type: "VariableDeclarator",
          },
        ],
        declare: false,
        kind: "const",
        span: { ctxt: 0, end: 21, start: 1 },
        type: "VariableDeclaration",
      },
    ],
    interpreter: null,
    span: { ctxt: 0, end: 21, start: 1 },
  });
});
