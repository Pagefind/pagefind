import matter from "gray-matter";
import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkGfm from "remark-gfm";
import { toString as mdastToString } from "mdast-util-to-string";
import { mdastToHtml } from "./mdast-to-html.ts";
import type { ParsedMarkdown } from "./types.ts";
import type { Root } from "mdast";
import type { Processor } from "unified";

const baseProcessor = unified().use(remarkParse).use(remarkGfm);
// eslint-disable-next-line @typescript-eslint/no-explicit-any
let mdxProcessor: Processor<any, any, any, any, any> | null = null;

async function getProcessor(mdx: boolean) {
    if (!mdx) return baseProcessor;
    if (!mdxProcessor) {
        // Dynamically import remark-mdx so it stays optional (peer dep)
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const { default: remarkMdx } = await import("remark-mdx" as any);
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        mdxProcessor = unified().use(remarkParse).use(remarkGfm).use(remarkMdx as any);
    }
    return mdxProcessor;
}

export async function parseMarkdown(
    fileContent: string,
    opts: { mdx?: boolean } = {}
): Promise<ParsedMarkdown> {
    const { data: frontmatter, content } = matter(fileContent);

    const processor = await getProcessor(opts.mdx ?? false);
    const tree = processor.parse(content) as Root;

    // Derive title: frontmatter.title > first h1 > empty string
    let title: string = frontmatter.title ?? "";
    if (!title) {
        const firstH1 = tree.children.find(
            (n) => n.type === "heading" && (n as { depth: number }).depth === 1
        );
        if (firstH1) {
            title = mdastToString(firstH1);
        }
    }

    const description: string = frontmatter.description ?? "";
    const slug: string | undefined = frontmatter.slug;

    const html = mdastToHtml(tree, { title, description });

    return { title, description, slug, html };
}
