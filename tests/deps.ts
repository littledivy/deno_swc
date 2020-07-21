export {
  assert,
  assertEquals,
} from "https://deno.land/std@0.61.0/testing/asserts.ts";

export const getLatestReleaseTag = () => {
  return fetch(
    "https://api.github.com/repos/nestdotland/deno_swc/releases/latest",
  )
    .then((result) => result.json())
    .then(({ tag_name }) => tag_name);
};
