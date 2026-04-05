import test from "ava";

// This is the regex used in public_search_api.ts to derive basePath
// from import.meta.url. It must preserve the full URL including the
// protocol and domain so that cross-origin setups (e.g. auth proxies)
// work correctly. Origin stripping for same-origin cases is handled
// downstream in PagefindInstance/PagefindWrapper.
const basePathRegex = /^(.*\/)pagefind.js.*$/;

test("same-origin basePath preserves full URL", (t) => {
  const url = "https://example.com/pagefind/pagefind.js";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, "https://example.com/pagefind/");
});

test("cross-origin basePath preserves full URL", (t) => {
  const url = "https://project.gitlab.io/docs/pagefind/pagefind.js";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, "https://project.gitlab.io/docs/pagefind/");
});

test("basePath with port preserves full URL", (t) => {
  const url = "https://example.com:8080/pagefind/pagefind.js";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, "https://example.com:8080/pagefind/");
});

test("basePath with query string", (t) => {
  const url = "https://example.com/pagefind/pagefind.js?v=abc123";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, "https://example.com/pagefind/");
});

test("basePath with subpath", (t) => {
  const url = "https://example.com/site/docs/pagefind/pagefind.js";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, "https://example.com/site/docs/pagefind/");
});

test("basePath from relative URL", (t) => {
  const url = "/pagefind/pagefind.js";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, "/pagefind/");
});

test("basePath does not match non-pagefind scripts", (t) => {
  const url = "https://example.com/other/script.js";
  const basePath = url.match(basePathRegex)?.[1];
  t.is(basePath, undefined);
});
