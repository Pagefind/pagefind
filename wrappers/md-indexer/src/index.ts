import fs from "node:fs/promises";
import { createIndex } from "pagefind";
import { globFiles } from "./glob-files.ts";
import { parseMarkdown } from "./parse-markdown.ts";
import { urlFromPath } from "./url-from-path.ts";
import type { MdIndexerOptions, MdIndexerResult } from "./types.ts";

export type { MdIndexerOptions, MdIndexerResult };

export async function indexMarkdown(
    options: MdIndexerOptions
): Promise<MdIndexerResult> {
    const {
        glob,
        vault,
        mdFiles,
        urlBase = "",
        outputPath = "./public/_pagefind",
        mdx = false,
        language = "en",
        pagefindConfig = {},
    } = options;

    // vault/mdFiles set contentDir if not explicitly provided
    const contentDir = options.contentDir ?? vault ?? mdFiles;

    const errors: string[] = [];

    // Resolve files and create index in parallel
    const [files, { errors: initErrors, index }] = await Promise.all([
        globFiles({ glob, vault, mdFiles }),
        createIndex({ ...pagefindConfig, forceLanguage: pagefindConfig.forceLanguage ?? language }),
    ]);

    if (files.length === 0) {
        return { pageCount: 0, outputPath, errors: ["No markdown files found."] };
    }

    if (initErrors.length > 0) errors.push(...initErrors);
    if (!index) return { pageCount: 0, outputPath, errors };

    let pageCount = 0;

    // Index each file
    await Promise.all(
        files.map(async (filePath) => {
            try {
                const raw = await fs.readFile(filePath, "utf-8");
                const parsed = await parseMarkdown(raw, { mdx });
                const url = urlFromPath({
                    filePath,
                    contentDir,
                    urlBase,
                    frontmatterSlug: parsed.slug,
                });

                const { errors: fileErrors } = await index.addHTMLFile({
                    url,
                    content: parsed.html,
                });

                if (fileErrors.length > 0) {
                    errors.push(...fileErrors.map((e) => `${filePath}: ${e}`));
                } else {
                    pageCount++;
                }
            } catch (err) {
                errors.push(
                    `${filePath}: ${err instanceof Error ? err.message : String(err)}`
                );
            }
        })
    );

    const { errors: writeErrors, outputPath: writtenPath } =
        await index.writeFiles({ outputPath });

    if (writeErrors.length > 0) {
        errors.push(...writeErrors);
    }

    return {
        pageCount,
        outputPath: writtenPath ?? outputPath,
        errors,
    };
}
