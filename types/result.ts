export type Result<
  T extends {
    ok: any;
    error: any;
  },
  OkType = T["ok"],
  ErrorType = T["error"],
> =
  | {
    type: "ok";
    value: OkType;
  }
  | {
    type: "error";
    error: ErrorType;
  };
