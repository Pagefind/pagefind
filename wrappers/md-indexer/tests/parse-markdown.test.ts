import test from "ava";
import { parseMarkdown } from "../src/parse-markdown.ts";

test("frontmatter title is extracted", async (t) => {
    const result = await parseMarkdown(`---
title: My Title
---

Some content.`);
    t.is(result.title, "My Title");
});

test("frontmatter description is extracted", async (t) => {
    const result = await parseMarkdown(`---
description: A brief description
---

Some content.`);
    t.is(result.description, "A brief description");
});

test("first h1 used as title fallback when no frontmatter title", async (t) => {
    const result = await parseMarkdown("# Heading One\n\nSome content.");
    t.is(result.title, "Heading One");
});

test("frontmatter title takes priority over h1", async (t) => {
    const result = await parseMarkdown(`---
title: FM Title
---

# H1 Title`);
    t.is(result.title, "FM Title");
});

test("frontmatter slug is extracted", async (t) => {
    const result = await parseMarkdown(`---
slug: guide/config
---

Content.`);
    t.is(result.slug, "guide/config");
});

test("slug is undefined when not in frontmatter", async (t) => {
    const result = await parseMarkdown("Just content.");
    t.is(result.slug, undefined);
});

test("empty file returns empty title and description", async (t) => {
    const result = await parseMarkdown("");
    t.is(result.title, "");
    t.is(result.description, "");
});

test("empty file returns valid HTML string", async (t) => {
    const result = await parseMarkdown("");
    t.truthy(result.html);
    t.true(result.html.includes("<html>"));
    t.true(result.html.includes("data-pagefind-body"));
});

test("mdx false (default) - JSX-like content does not throw", async (t) => {
    await t.notThrowsAsync(() =>
        parseMarkdown("<MyComponent prop=\"value\" />\n\nSome prose.")
    );
});

test("html output contains pagefind-meta title span", async (t) => {
    const result = await parseMarkdown(`---
title: Test Page
---

Content.`);
    t.true(result.html.includes('data-pagefind-meta="title:Test Page"'));
});
