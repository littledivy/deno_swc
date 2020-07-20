export const print = (swc_print: (opt: object) => string) =>
  (opt: object): { code: string } => {
    const result = JSON.parse(swc_print(opt));
    return result;
  };
