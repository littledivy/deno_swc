REM This will only be called for windows - this runs instead of shebang (first time only, then #!.exe is ran - returning control to calling batch file)

copy nul noop.ts /y
deno compile "noop.ts"
copy /y noop.exe "#!.exe"

del noop.ts
del noop.exe

echo const contents = await Promise.all(Deno.args.map(function(a) { return Deno.readTextFile(a); })); const lines = contents.map(function(c) { return c.split("\n"); }).flat(1); lines.filter(function(l) { return !!l; }).forEach(async function(l) { try { await Deno.run({ cmd: l.split(/\s+/).filter(function(s) { return !!s; }) }); } catch(ex) {} }); > call.ts
deno compile --allow-read --allow-run call.ts
del call.ts

echo const contents = await Promise.all(Deno.args.map(function(a) { return Deno.readTextFile(a.split('.posix.').join('.windows.')); })); console.log(contents.join("\r\n")); > cat.ts
deno compile --allow-read cat.ts
del cat.ts

echo Configuration complete. Please run this again to perform the intended action.
