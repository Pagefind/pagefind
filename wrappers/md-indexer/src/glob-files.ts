import fg from "fast-glob";
import path from "node:path";

export interface GlobOptions {
    glob?: string;
    vault?: string;
    mdFiles?: string;
    mdx?: boolean;
}

/**
 * Resolve the list of markdown files to index.
 * `vault` and `mdFiles` are equivalent - both accept a directory path and
 * expand to `<dir>/**\/*.md` (or `**\/*.{md,mdx}` when `mdx: true`),
 * skipping hidden files and directories.
 * `glob` is used as-is when neither directory option is provided.
 */
export async function globFiles(opts: GlobOptions): Promise<string[]> {
    const dir = opts.vault ?? opts.mdFiles;

    if (!dir && !opts.glob) {
        throw new Error("Either `glob`, `vault`, or `md-files` must be provided.");
    }

    const ext = opts.mdx ? "{md,mdx}" : "md";
    const pattern = dir
        ? path.join(dir, `**/*.${ext}`).split(path.sep).join("/")
        : (opts.glob as string);

    const files = await fg(pattern, { absolute: true, onlyFiles: true, dot: false });
    return files;
}
