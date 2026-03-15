import GithubSlugger from "github-slugger";
import { toString as mdastToString } from "mdast-util-to-string";
import type { Root, RootContent, PhrasingContent } from "mdast";

/**
 * Walk an mdast and emit an HTML string suitable for Pagefind indexing.
 *
 * - All headings get GitHub-compatible `id` slugs
 * - `data-pagefind-meta` spans carry title/description (one per element)
 * - MDX JSX nodes are silently skipped when not recognised
 */
export function mdastToHtml(
    tree: Root,
    meta: { title: string; description: string }
): string {
    const slugger = new GithubSlugger();
    const parts: string[] = [];

    parts.push("<html><body>\n<div data-pagefind-body>\n");

    if (meta.title) {
        parts.push(
            `  <span data-pagefind-meta="title:${escAttr(meta.title)}" style="display:none"></span>\n`
        );
    }
    if (meta.description) {
        parts.push(
            `  <span data-pagefind-meta="description:${escAttr(meta.description)}" style="display:none"></span>\n`
        );
    }

    for (const node of tree.children) {
        parts.push(renderNode(node, slugger));
    }

    parts.push("</div>\n</body></html>");
    return parts.join("");
}

function renderNode(node: RootContent, slugger: GithubSlugger): string {
    switch (node.type) {
        case "heading": {
            const text = mdastToString(node);
            const id = slugger.slug(text);
            const tag = `h${node.depth}`;
            const inner = node.children.map(renderInline).join("");
            return `  <${tag} id="${escAttr(id)}">${inner}</${tag}>\n`;
        }
        case "paragraph": {
            const inner = node.children.map(renderInline).join("");
            return `  <p>${inner}</p>\n`;
        }
        case "code": {
            const lang = node.lang ? ` class="language-${escAttr(node.lang)}"` : "";
            return `  <pre><code${lang}>${escHtml(node.value)}</code></pre>\n`;
        }
        case "blockquote": {
            const inner = node.children
                .map((c) => renderNode(c as RootContent, slugger))
                .join("");
            return `  <blockquote>\n${inner}  </blockquote>\n`;
        }
        case "list": {
            const tag = node.ordered ? "ol" : "ul";
            const items = node.children
                .map((item) => {
                    const content = item.children
                        .map((c) => renderNode(c as RootContent, slugger))
                        .join("");
                    return `    <li>${content.trim()}</li>\n`;
                })
                .join("");
            return `  <${tag}>\n${items}  </${tag}>\n`;
        }
        case "thematicBreak":
            return "  <hr />\n";
        case "html":
            // Pass raw HTML through (e.g. embedded HTML in md)
            return node.value + "\n";
        case "table": {
            const rows = node.children.map((row, rowIdx) => {
                const cells = row.children.map((cell) => {
                    const tag = rowIdx === 0 ? "th" : "td";
                    const inner = cell.children.map(renderInline).join("");
                    return `<${tag}>${inner}</${tag}>`;
                });
                return `<tr>${cells.join("")}</tr>`;
            });
            return `  <table><tbody>${rows.join("")}</tbody></table>\n`;
        }
        default:
            // Silently skip unknown nodes (MDX jsx, mdxjsEsm, etc.)
            return "";
    }
}

function renderInline(node: PhrasingContent): string {
    switch (node.type) {
        case "text":
            return escHtml(node.value);
        case "strong":
            return `<strong>${node.children.map(renderInline).join("")}</strong>`;
        case "emphasis":
            return `<em>${node.children.map(renderInline).join("")}</em>`;
        case "inlineCode":
            return `<code>${escHtml(node.value)}</code>`;
        case "link": {
            const href = escAttr(node.url);
            const inner = node.children.map(renderInline).join("");
            return `<a href="${href}">${inner}</a>`;
        }
        case "image":
            return `<img src="${escAttr(node.url)}" alt="${escAttr(node.alt ?? "")}" />`;
        case "break":
            return "<br />";
        case "delete":
            return `<del>${node.children.map(renderInline).join("")}</del>`;
        case "html":
            return node.value;
        default:
            // Skip unrecognised inline nodes (MDX expression, etc.)
            return "";
    }
}

function escHtml(s: string): string {
    return s
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;");
}

function escAttr(s: string): string {
    return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}
