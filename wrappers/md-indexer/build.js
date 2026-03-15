import { build } from "esbuild";

const external = [
    "pagefind",
    "remark-mdx",
    "fast-glob",
    "gray-matter",
    "unified",
    "remark-parse",
    "remark-gfm",
    "mdast-util-to-string",
    "github-slugger",
];

// ESM: includes CLI (top-level await requires ESM)
await build({
    entryPoints: ["src/index.ts", "src/cli.ts"],
    bundle: true,
    platform: "node",
    format: "esm",
    external,
    outdir: "dist/esm",
    outExtension: { ".js": ".js" },
});

// CJS: library only (CLI is ESM-only due to top-level await)
await build({
    entryPoints: ["src/index.ts"],
    bundle: true,
    platform: "node",
    format: "cjs",
    external,
    outdir: "dist/cjs",
    outExtension: { ".js": ".cjs" },
});

console.log("Build complete: dist/esm + dist/cjs");
