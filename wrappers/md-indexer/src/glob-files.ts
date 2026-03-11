import fg from "fast-glob";
import path from "node:path";

export interface GlobOptions {
    glob?: string;
    vault?: string;
    mdFiles?: string;
}

/**
 * Resolve the list of markdown files to index.
 * `vault` and `mdFiles` are equivalent - both accept a directory path and
 * expand to `<dir>/**\/*.md`, skipping hidden files and directories.
 * `glob` is used as-is when neither directory option is provided.
 */
export async function globFiles(opts: GlobOptions): Promise<string[]> {
    const dir = opts.vault ?? opts.mdFiles;

    if (!dir && !opts.glob) {
        throw new Error("Either `glob`, `vault`, or `md-files` must be provided.");
    }

    const pattern = dir
        ? path.join(dir, "**/*.md").split(path.sep).join("/")
        : (opts.glob as string);

    const files = await fg(pattern, { absolute: true, onlyFiles: true, dot: false });
    return files;
}
