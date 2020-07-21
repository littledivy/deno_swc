export const getLatestReleaseTag = (): Promise<string> => {
  return fetch(
    "https://api.github.com/repos/nestdotland/deno_swc/releases/latest",
  )
    .then((result) => result.json())
    .then(({ tag_name }) => tag_name);
};

export const { extractDependencies, print, parseTypescript } = await import(
  `https://raw.githubusercontent.com/nestdotland/deno_swc/${await getLatestReleaseTag()}/mod.ts`
);
