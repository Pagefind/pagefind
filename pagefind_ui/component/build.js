import esbuild from "esbuild";
import path from "path";
import ImportGlobPlugin from "esbuild-plugin-import-glob";
import fs from "fs";

import { createRequire } from "module";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const { version } = require("./package.json");

const serve = async () => {
  const esbuildOptions = {
    outdir: path.join(__dirname, "_dev_files/pagefind"),
    entryPoints: [path.join(__dirname, "component-ui.ts")],
    plugins: [ImportGlobPlugin.default()],
    bundle: true,
  };

  fs.mkdirSync(path.join(__dirname, "_dev_files/pagefind"), {
    recursive: true,
  });

  const cssLink = path.join(__dirname, "_dev_files/pagefind/component-ui.css");
  try {
    fs.rmSync(cssLink);
  } catch (e) {}
  fs.symlinkSync(
    path.join(__dirname, "css/pagefind-component-ui.css"),
    cssLink,
  );

  const context = await esbuild.context(esbuildOptions);
  const server = await context.serve({
    servedir: path.join(__dirname, "_dev_files"),
  });
  console.log(`Serving Component UI on http://localhost:${server.port}`);
};

const build = async () => {
  const commonOpts = {
    write: true,
    plugins: [ImportGlobPlugin.default()],
    loader: {},
    define: {},
    bundle: true,
  };

  const esbuildVendorOptions = {
    ...commonOpts,
    outdir: path.join(__dirname, `../../pagefind/vendor/`),
    entryPoints: [path.join(__dirname, "component-ui.ts")],
    entryNames: `pagefind_component_ui.${version}`,
    minify: true,
  };
  const compiledVendor = await esbuild.build(esbuildVendorOptions);
  console.log("Vendor Build:", compiledVendor);

  // CJS "main" build (includes all components)
  const esbuildCjsOptions = {
    ...commonOpts,
    entryPoints: [path.join(__dirname, "component-ui.ts")],
    outdir: path.join(__dirname, `npm_dist/cjs/`),
    outExtension: { ".js": ".cjs" },
    format: "cjs",
    platform: "neutral",
  };
  const compiledCJS = await esbuild.build(esbuildCjsOptions);
  console.log("CJS Build:", compiledCJS);

  // ESM Module Build (includes all components)
  const esbuildModuleOptions = {
    ...commonOpts,
    entryPoints: [path.join(__dirname, "component-ui.ts")],
    outdir: path.join(__dirname, `npm_dist/mjs/`),
    outExtension: { ".js": ".mjs" },
    format: "esm",
    platform: "neutral",
  };
  const compiledMJS = await esbuild.build(esbuildModuleOptions);
  console.log("ESM Build:", compiledMJS);

  fs.copyFileSync(
    path.join(__dirname, "css/pagefind-component-ui.css"),
    path.join(
      __dirname,
      `../../pagefind/vendor/pagefind_component_ui.${version}.css`,
    ),
  );
};

if (process.env.PAGEFIND_DEV) {
  serve();
} else {
  build();
}
