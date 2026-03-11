import test from "ava";
import { unified } from "unified";
import remarkParse from "remark-parse";
import type { Root } from "mdast";
import { mdastToHtml } from "../src/mdast-to-html.ts";

function parse(md: string): Root {
    return unified().use(remarkParse).parse(md) as Root;
}

const NO_META = { title: "", description: "" } as const;

test("title meta span is emitted", (t) => {
    const tree = parse("Hello");
    const html = mdastToHtml(tree, { title: "My Page", description: "" });
    t.true(html.includes('data-pagefind-meta="title:My Page"'));
    t.true(html.includes('style="display:none"'));
});

test("description meta span is emitted", (t) => {
    const tree = parse("Hello");
    const html = mdastToHtml(tree, { title: "", description: "A description" });
    t.true(html.includes('data-pagefind-meta="description:A description"'));
});

test("no title meta span when title is empty", (t) => {
    const tree = parse("Hello");
    const html = mdastToHtml(tree, NO_META);
    t.false(html.includes("data-pagefind-meta=\"title:"));
});

test("headings get id slugs", (t) => {
    const tree = parse("# Introduction\n\nSome text.");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes('<h1 id="introduction">'));
});

test("headings use github-slugger casing", (t) => {
    const tree = parse("## My Section");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes('<h2 id="my-section">'));
});

test("duplicate headings get numbered suffixes", (t) => {
    const tree = parse("## Foo\n\n## Foo\n\n## Foo");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes('id="foo"'));
    t.true(html.includes('id="foo-1"'));
    t.true(html.includes('id="foo-2"'));
});

test("code block becomes pre>code", (t) => {
    const tree = parse("```js\nconsole.log('hi');\n```");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes('<pre><code class="language-js">'));
    t.true(html.includes("console.log(&#x27;hi&#x27;)") || html.includes("console.log('hi')"));
});

test("bold becomes strong", (t) => {
    const tree = parse("**bold text**");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes("<strong>bold text</strong>"));
});

test("emphasis becomes em", (t) => {
    const tree = parse("_em text_");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes("<em>em text</em>"));
});

test("inline code becomes code", (t) => {
    const tree = parse("Use `myFunc()` here.");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes("<code>myFunc()</code>"));
});

test("unknown node types are silently skipped (no crash)", (t) => {
    const fakeTree: Root = {
        type: "root",
        children: [
            { type: "mdxJsxFlowElement" as any, children: [] } as any,
        ],
    };
    t.notThrows(() => mdastToHtml(fakeTree, NO_META));
});

test("no headings produces only meta spans", (t) => {
    const tree = parse("Just some prose.");
    const html = mdastToHtml(tree, { title: "T", description: "" });
    t.false(html.includes("<h1") || html.includes("<h2") || html.includes("<h3"));
    t.true(html.includes('data-pagefind-meta="title:T"'));
});

test("html wraps content in data-pagefind-body div", (t) => {
    const tree = parse("Hello");
    const html = mdastToHtml(tree, NO_META);
    t.true(html.includes("<div data-pagefind-body>"));
    t.true(html.startsWith("<html><body>"));
    t.true(html.endsWith("</body></html>"));
});
