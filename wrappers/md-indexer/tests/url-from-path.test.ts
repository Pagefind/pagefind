import { describe, test, expect } from "bun:test";
import { urlFromPath } from "../src/url-from-path.ts";

describe("url-from-path", () => {
    test("index.md at root collapses to /", () => {
        expect(urlFromPath({ filePath: "index.md" })).toBe("/");
    });

    test("guide/index.md collapses to /guide/", () => {
        expect(urlFromPath({ filePath: "guide/index.md" })).toBe("/guide/");
    });

    test("regular file gets slug from path", () => {
        expect(urlFromPath({ filePath: "notes/about.md" })).toBe("/notes/about");
    });

    test("contentDir strips prefix from path", () => {
        expect(urlFromPath({
            filePath: "/vault/notes/Projects/foo.md",
            contentDir: "/vault/notes",
        })).toBe("/Projects/foo");
    });

    test("vault as contentDir strips vault prefix", () => {
        expect(urlFromPath({
            filePath: "/vault/guide/intro.md",
            contentDir: "/vault",
        })).toBe("/guide/intro");
    });

    test("urlBase is prepended to slug", () => {
        expect(urlFromPath({
            filePath: "guide/intro.md",
            urlBase: "/docs",
        })).toBe("/docs/guide/intro");
    });

    test("urlBase is prepended to index collapse", () => {
        expect(urlFromPath({
            filePath: "guide/index.md",
            urlBase: "/docs",
        })).toBe("/docs/guide/");
    });

    test("frontmatter slug (relative) overrides path", () => {
        expect(urlFromPath({
            filePath: "guide/configuration.md",
            frontmatterSlug: "guide/config",
        })).toBe("/guide/config");
    });

    test("frontmatter slug (absolute) overrides path", () => {
        expect(urlFromPath({
            filePath: "guide/configuration.md",
            frontmatterSlug: "/guide/config",
        })).toBe("/guide/config");
    });

    test("frontmatter slug with urlBase", () => {
        expect(urlFromPath({
            filePath: "guide/configuration.md",
            urlBase: "/docs",
            frontmatterSlug: "guide/config",
        })).toBe("/docs/guide/config");
    });

    test("no contentDir uses full filePath", () => {
        expect(urlFromPath({ filePath: "notes/foo.md" })).toBe("/notes/foo");
    });
});
