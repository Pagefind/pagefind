import test from "ava";
import path from "node:path";
import fs from "node:fs/promises";
import os from "node:os";
import { indexMarkdown } from "../src/index.ts";

const fixturesDir = new URL("../fixtures", import.meta.url).pathname;

async function makeTempDir(): Promise<string> {
    return fs.mkdtemp(path.join(os.tmpdir(), "md-indexer-test-"));
}

async function cleanupDir(dir: string): Promise<void> {
    await fs.rm(dir, { recursive: true, force: true });
}

test("indexes fixture vault and returns pageCount 3", async (t) => {
    const outputPath = await makeTempDir();
    try {
        const result = await indexMarkdown({ vault: fixturesDir, outputPath });
        t.is(result.pageCount, 3, `Expected 3 pages, got ${result.pageCount}. Errors: ${result.errors.join(", ")}`);
        t.deepEqual(result.errors, []);
    } finally {
        await cleanupDir(outputPath);
    }
});

test("output directory contains pagefind bundle", async (t) => {
    const outputPath = await makeTempDir();
    try {
        await indexMarkdown({ vault: fixturesDir, outputPath });
        const entries = await fs.readdir(outputPath);
        t.true(entries.some((e) => e.startsWith("pagefind")));
    } finally {
        await cleanupDir(outputPath);
    }
});

test("empty vault returns pageCount 0 with error message", async (t) => {
    const emptyVault = await makeTempDir();
    const outputPath = await makeTempDir();
    try {
        const result = await indexMarkdown({ vault: emptyVault, outputPath });
        t.is(result.pageCount, 0);
        t.true(result.errors.length > 0);
        t.true(result.errors.some((e) => e.toLowerCase().includes("no markdown")));
    } finally {
        await cleanupDir(emptyVault);
        await cleanupDir(outputPath);
    }
});

test("missing both glob and vault rejects", async (t) => {
    await t.throwsAsync(() => indexMarkdown({}));
});

test("mdFiles option works the same as vault", async (t) => {
    const outputPath = await makeTempDir();
    try {
        const result = await indexMarkdown({ mdFiles: fixturesDir, outputPath });
        t.is(result.pageCount, 3, `Expected 3 pages, got ${result.pageCount}. Errors: ${result.errors.join(", ")}`);
        t.deepEqual(result.errors, []);
    } finally {
        await cleanupDir(outputPath);
    }
});

test("hidden files and directories are skipped", async (t) => {
    const dir = await makeTempDir();
    const outputPath = await makeTempDir();
    try {
        // Visible file
        await fs.writeFile(path.join(dir, "visible.md"), "# Visible\n\nContent.");
        // Hidden file at root
        await fs.writeFile(path.join(dir, ".hidden.md"), "# Hidden\n\nShould be skipped.");
        // Hidden directory (e.g. .obsidian, .trash)
        await fs.mkdir(path.join(dir, ".obsidian"));
        await fs.writeFile(path.join(dir, ".obsidian", "config.md"), "# Config\n\nShould be skipped.");

        const result = await indexMarkdown({ vault: dir, outputPath });
        t.is(result.pageCount, 1, `Expected 1 page (hidden files skipped), got ${result.pageCount}. Errors: ${result.errors.join(", ")}`);
    } finally {
        await cleanupDir(dir);
        await cleanupDir(outputPath);
    }
});

test("urlBase is applied to output pages", async (t) => {
    const outputPath = await makeTempDir();
    try {
        // Should not throw and still return pages
        const result = await indexMarkdown({
            vault: fixturesDir,
            urlBase: "/docs",
            outputPath,
        });
        t.is(result.pageCount, 3);
        t.deepEqual(result.errors, []);
    } finally {
        await cleanupDir(outputPath);
    }
});
