Be sure to have your rust environment up and running!

Windows:

1. install rust
2. download wasm-pack-[version]-[OS]-[env].tar.gz from
   https://github.com/rustwasm/wasm-pack/releases,
3. open with 7-zip or your favorite archival app.
4. Save to %USERPROFILE%\\.cargo\\bin
5. install
   https://support.microsoft.com/en-us/topic/the-latest-supported-visual-c-downloads-2647da03-1eea-4433-9aff-95f26a218cc0
6. install https://visualstudio.microsoft.com/downloads/ -> '> tools for visual
   studio 2019' and open developer command prompt
7. navigate here and run cd swc_wasm && deno run -A ./build.js

When installing, customize and pick nightly. Otherwise you will get: 'Could not
find required build tool rustup' or other error
