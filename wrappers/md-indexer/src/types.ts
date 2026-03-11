export interface MdIndexerOptions {
    /** Glob pattern, e.g. "src/content/**\/*.{md,mdx}" */
    glob?: string;
    /** Directory path; expands to vault/**\/*.md and sets contentDir */
    vault?: string;
    /** Alias for vault; a directory of markdown files */
    mdFiles?: string;
    /** Prepended to computed slugs, e.g. "/docs" */
    urlBase?: string;
    /** Root directory for relative URL computation */
    contentDir?: string;
    /** Output path for the pagefind bundle; defaults to "./public/_pagefind" */
    outputPath?: string;
    /** Opt-in MDX support (needs remark-mdx installed) */
    mdx?: boolean;
    /** Default "en" */
    language?: string;
    /** Config passed to Pagefind's createIndex */
    pagefindConfig?: PagefindServiceConfig;
}

export interface MdIndexerResult {
    pageCount: number;
    outputPath: string;
    errors: string[];
}

/** Mirrors pagefind's PagefindServiceConfig to avoid requiring the package at type-check time */
export interface PagefindServiceConfig {
    rootSelector?: string;
    excludeSelectors?: string[];
    forceLanguage?: string;
    verbose?: boolean;
    logfile?: string;
    keepIndexUrl?: boolean;
    writePlayground?: boolean;
    includeCharacters?: string;
}

export interface ParsedMarkdown {
    title: string;
    description: string;
    slug: string | undefined;
    html: string;
}
