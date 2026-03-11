import { describe, test, expect } from "bun:test";
import { parseMarkdown } from "../src/parse-markdown.ts";

describe("parse-markdown", () => {
    test("frontmatter title is extracted", async () => {
        const result = await parseMarkdown(`---
title: My Title
---

Some content.`);
        expect(result.title).toBe("My Title");
    });

    test("frontmatter description is extracted", async () => {
        const result = await parseMarkdown(`---
description: A brief description
---

Some content.`);
        expect(result.description).toBe("A brief description");
    });

    test("first h1 used as title fallback when no frontmatter title", async () => {
        const result = await parseMarkdown("# Heading One\n\nSome content.");
        expect(result.title).toBe("Heading One");
    });

    test("frontmatter title takes priority over h1", async () => {
        const result = await parseMarkdown(`---
title: FM Title
---

# H1 Title`);
        expect(result.title).toBe("FM Title");
    });

    test("frontmatter slug is extracted", async () => {
        const result = await parseMarkdown(`---
slug: guide/config
---

Content.`);
        expect(result.slug).toBe("guide/config");
    });

    test("slug is undefined when not in frontmatter", async () => {
        const result = await parseMarkdown("Just content.");
        expect(result.slug).toBeUndefined();
    });

    test("empty file returns empty title and description", async () => {
        const result = await parseMarkdown("");
        expect(result.title).toBe("");
        expect(result.description).toBe("");
    });

    test("empty file returns valid HTML string", async () => {
        const result = await parseMarkdown("");
        expect(result.html).toBeTruthy();
        expect(result.html).toContain("<html>");
        expect(result.html).toContain("data-pagefind-body");
    });

    test("mdx false (default) - JSX-like content does not throw", async () => {
        await expect(
            parseMarkdown('<MyComponent prop="value" />\n\nSome prose.')
        ).resolves.toBeDefined();
    });

    test("html output contains pagefind-meta title span", async () => {
        const result = await parseMarkdown(`---
title: Test Page
---

Content.`);
        expect(result.html).toContain('data-pagefind-meta="title:Test Page"');
    });
});
