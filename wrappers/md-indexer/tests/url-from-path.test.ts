import test from "ava";
import { urlFromPath } from "../src/url-from-path.ts";

test("index.md at root collapses to /", (t) => {
    const url = urlFromPath({ filePath: "index.md" });
    t.is(url, "/");
});

test("guide/index.md collapses to /guide/", (t) => {
    const url = urlFromPath({ filePath: "guide/index.md" });
    t.is(url, "/guide/");
});

test("regular file gets slug from path", (t) => {
    const url = urlFromPath({ filePath: "notes/about.md" });
    t.is(url, "/notes/about");
});

test("contentDir strips prefix from path", (t) => {
    const url = urlFromPath({
        filePath: "/vault/notes/Projects/foo.md",
        contentDir: "/vault/notes",
    });
    t.is(url, "/Projects/foo");
});

test("vault as contentDir strips vault prefix", (t) => {
    const url = urlFromPath({
        filePath: "/vault/guide/intro.md",
        contentDir: "/vault",
    });
    t.is(url, "/guide/intro");
});

test("urlBase is prepended to slug", (t) => {
    const url = urlFromPath({
        filePath: "guide/intro.md",
        urlBase: "/docs",
    });
    t.is(url, "/docs/guide/intro");
});

test("urlBase is prepended to index collapse", (t) => {
    const url = urlFromPath({
        filePath: "guide/index.md",
        urlBase: "/docs",
    });
    t.is(url, "/docs/guide/");
});

test("frontmatter slug (relative) overrides path", (t) => {
    const url = urlFromPath({
        filePath: "guide/configuration.md",
        frontmatterSlug: "guide/config",
    });
    t.is(url, "/guide/config");
});

test("frontmatter slug (absolute) overrides path", (t) => {
    const url = urlFromPath({
        filePath: "guide/configuration.md",
        frontmatterSlug: "/guide/config",
    });
    t.is(url, "/guide/config");
});

test("frontmatter slug with urlBase", (t) => {
    const url = urlFromPath({
        filePath: "guide/configuration.md",
        urlBase: "/docs",
        frontmatterSlug: "guide/config",
    });
    t.is(url, "/docs/guide/config");
});

test("no contentDir uses full filePath", (t) => {
    const url = urlFromPath({ filePath: "notes/foo.md" });
    t.is(url, "/notes/foo");
});
