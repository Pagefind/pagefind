import path from "node:path";

export interface UrlFromPathOptions {
    filePath: string;
    contentDir?: string;
    urlBase?: string;
    frontmatterSlug?: string;
}

/**
 * Compute a URL for a markdown file.
 *
 * Priority:
 * 1. Frontmatter `slug` field
 * 2. File path relative to contentDir, stripped of extension
 * 3. `index.md` collapses to parent: `guide/index.md` -> `/guide/`
 */
export function urlFromPath(opts: UrlFromPathOptions): string {
    const { filePath, contentDir, urlBase = "", frontmatterSlug } = opts;

    if (frontmatterSlug) {
        const slug = frontmatterSlug.startsWith("/")
            ? frontmatterSlug
            : `/${frontmatterSlug}`;
        return urlBase + slug;
    }

    const base = contentDir
        ? path.relative(contentDir, filePath)
        : filePath;

    // Normalise to forward slashes on Windows
    const normalized = base.split(path.sep).join("/");

    // Strip extension
    const withoutExt = normalized.replace(/\.(md|mdx)$/i, "");

    // Detect if this is an index file collapse (e.g. guide/index -> /guide/)
    const isIndex = /(?:^|\/?)index$/.test(withoutExt);

    // Collapse index files to their parent path
    const collapsed = withoutExt
        .replace(/\/index$/, "/")
        .replace(/^index$/, "");

    // Ensure leading slash
    const slug = collapsed.startsWith("/") ? collapsed : `/${collapsed}`;

    // For index collapses keep the trailing slash; for regular files strip it
    if (isIndex) {
        // slug is either "/" or "/parent/"
        return urlBase + (slug || "/");
    }

    const finalSlug = slug.replace(/\/$/, "") || "/";
    return urlBase + finalSlug;
}
