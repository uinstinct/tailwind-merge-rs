const esbuild = require("esbuild");

esbuild
  .build({
    entryPoints: ["tailwind-merge.js"],
    bundle: true,
    outfile: "../src/tailwind-merge.js",
    format: "esm",
    minify: true,
  })
  .catch(() => process.exit(1));
