import { describe, test, expect } from "bun:test";
import { unified } from "unified";
import remarkParse from "remark-parse";
import type { Root } from "mdast";
import { mdastToHtml } from "../src/mdast-to-html.ts";

function parse(md: string): Root {
    return unified().use(remarkParse).parse(md) as Root;
}

const NO_META = { title: "", description: "" } as const;

describe("mdast-to-html", () => {
    test("title meta span is emitted", () => {
        const html = mdastToHtml(parse("Hello"), { title: "My Page", description: "" });
        expect(html).toContain('data-pagefind-meta="title:My Page"');
        expect(html).toContain('style="display:none"');
    });

    test("description meta span is emitted", () => {
        const html = mdastToHtml(parse("Hello"), { title: "", description: "A description" });
        expect(html).toContain('data-pagefind-meta="description:A description"');
    });

    test("no title meta span when title is empty", () => {
        const html = mdastToHtml(parse("Hello"), NO_META);
        expect(html).not.toContain('data-pagefind-meta="title:');
    });

    test("headings get id slugs", () => {
        const html = mdastToHtml(parse("# Introduction\n\nSome text."), NO_META);
        expect(html).toContain('<h1 id="introduction">');
    });

    test("headings use github-slugger casing", () => {
        const html = mdastToHtml(parse("## My Section"), NO_META);
        expect(html).toContain('<h2 id="my-section">');
    });

    test("duplicate headings get numbered suffixes", () => {
        const html = mdastToHtml(parse("## Foo\n\n## Foo\n\n## Foo"), NO_META);
        expect(html).toContain('id="foo"');
        expect(html).toContain('id="foo-1"');
        expect(html).toContain('id="foo-2"');
    });

    test("code block becomes pre>code", () => {
        const html = mdastToHtml(parse("```js\nconsole.log('hi');\n```"), NO_META);
        expect(html).toContain('<pre><code class="language-js">');
    });

    test("bold becomes strong", () => {
        const html = mdastToHtml(parse("**bold text**"), NO_META);
        expect(html).toContain("<strong>bold text</strong>");
    });

    test("emphasis becomes em", () => {
        const html = mdastToHtml(parse("_em text_"), NO_META);
        expect(html).toContain("<em>em text</em>");
    });

    test("inline code becomes code", () => {
        const html = mdastToHtml(parse("Use `myFunc()` here."), NO_META);
        expect(html).toContain("<code>myFunc()</code>");
    });

    test("unknown node types are silently skipped (no crash)", () => {
        const fakeTree: Root = {
            type: "root",
            children: [
                { type: "mdxJsxFlowElement" as any, children: [] } as any,
            ],
        };
        expect(() => mdastToHtml(fakeTree, NO_META)).not.toThrow();
    });

    test("no headings produces only meta spans", () => {
        const html = mdastToHtml(parse("Just some prose."), { title: "T", description: "" });
        expect(html).not.toMatch(/<h[1-3]/);
        expect(html).toContain('data-pagefind-meta="title:T"');
    });

    test("title with double-quote is correctly escaped in meta span", () => {
        const html = mdastToHtml(parse("Hello"), { title: 'My "Guide"', description: "" });
        expect(html).toContain('data-pagefind-meta="title:My &quot;Guide&quot;"');
        expect(html).not.toContain("&amp;quot;");
    });

    test("title with ampersand is correctly escaped in meta span", () => {
        const html = mdastToHtml(parse("Hello"), { title: "Cats & Dogs", description: "" });
        expect(html).toContain('data-pagefind-meta="title:Cats &amp; Dogs"');
    });

    test("html wraps content in data-pagefind-body div", () => {
        const html = mdastToHtml(parse("Hello"), NO_META);
        expect(html).toContain("<div data-pagefind-body>");
        expect(html).toStartWith("<html><body>");
        expect(html).toEndWith("</body></html>");
    });
});
