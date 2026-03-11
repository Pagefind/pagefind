#!/usr/bin/env node
import { parseArgs } from "node:util";
import { indexMarkdown } from "./index.ts";

const { values } = parseArgs({
    options: {
        glob: { type: "string" },
        vault: { type: "string" },
        "md-files": { type: "string" },
        "url-base": { type: "string" },
        output: { type: "string" },
        mdx: { type: "boolean", default: false },
        language: { type: "string", default: "en" },
        verbose: { type: "boolean", default: false },
    },
    allowPositionals: false,
});

const urlBase = values["url-base"];
const outputPath = values["output"] ?? "./public/_pagefind";

if (!values.glob && !values.vault && !values["md-files"]) {
    console.error("Error: either --glob, --vault, or --md-files is required.");
    process.exit(1);
}

console.log("Indexing markdown files...");

const result = await indexMarkdown({
    glob: values.glob,
    vault: values.vault,
    mdFiles: values["md-files"],
    urlBase,
    outputPath,
    mdx: values.mdx,
    language: values.language,
    pagefindConfig: {
        verbose: values.verbose,
    },
});

if (result.errors.length > 0) {
    for (const err of result.errors) {
        console.error("  Error:", err);
    }
}

console.log(`Done. Indexed ${result.pageCount} page(s) -> ${result.outputPath}`);

if (result.errors.length > 0) {
    process.exit(1);
}
