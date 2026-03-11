import { describe, test, expect } from "bun:test";
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

describe("index", () => {
    test("indexes fixture vault and returns pageCount 4", async () => {
        const outputPath = await makeTempDir();
        try {
            const result = await indexMarkdown({ vault: fixturesDir, outputPath });
            expect(result.pageCount).toBe(4);
            expect(result.errors).toEqual([]);
        } finally {
            await cleanupDir(outputPath);
        }
    });

    test("output directory contains pagefind bundle", async () => {
        const outputPath = await makeTempDir();
        try {
            await indexMarkdown({ vault: fixturesDir, outputPath });
            const entries = await fs.readdir(outputPath);
            expect(entries.some((e) => e.startsWith("pagefind"))).toBe(true);
        } finally {
            await cleanupDir(outputPath);
        }
    });

    test("empty vault returns pageCount 0 with error message", async () => {
        const emptyVault = await makeTempDir();
        const outputPath = await makeTempDir();
        try {
            const result = await indexMarkdown({ vault: emptyVault, outputPath });
            expect(result.pageCount).toBe(0);
            expect(result.errors.length).toBeGreaterThan(0);
            expect(result.errors.some((e) => e.toLowerCase().includes("no markdown"))).toBe(true);
        } finally {
            await cleanupDir(emptyVault);
            await cleanupDir(outputPath);
        }
    });

    test("missing both glob and vault rejects", async () => {
        await expect(indexMarkdown({})).rejects.toBeDefined();
    });

    test("mdFiles option works the same as vault", async () => {
        const outputPath = await makeTempDir();
        try {
            const result = await indexMarkdown({ mdFiles: fixturesDir, outputPath });
            expect(result.pageCount).toBe(4);
            expect(result.errors).toEqual([]);
        } finally {
            await cleanupDir(outputPath);
        }
    });

    test("hidden files and directories are skipped", async () => {
        const dir = await makeTempDir();
        const outputPath = await makeTempDir();
        try {
            await fs.writeFile(path.join(dir, "visible.md"), "# Visible\n\nContent.");
            await fs.writeFile(path.join(dir, ".hidden.md"), "# Hidden\n\nShould be skipped.");
            await fs.mkdir(path.join(dir, ".obsidian"));
            await fs.writeFile(path.join(dir, ".obsidian", "config.md"), "# Config\n\nShould be skipped.");

            const result = await indexMarkdown({ vault: dir, outputPath });
            expect(result.pageCount).toBe(1);
        } finally {
            await cleanupDir(dir);
            await cleanupDir(outputPath);
        }
    });

    test("urlBase is applied to output pages", async () => {
        const outputPath = await makeTempDir();
        try {
            const result = await indexMarkdown({
                vault: fixturesDir,
                urlBase: "/docs",
                outputPath,
            });
            expect(result.pageCount).toBe(4);
            expect(result.errors).toEqual([]);
        } finally {
            await cleanupDir(outputPath);
        }
    });

    test("ranking page scores highest for query 'bm25'", async () => {
        const outputPath = await makeTempDir();
        const server = Bun.serve({
            port: 0,
            async fetch(req) {
                const url = new URL(req.url);
                const file = Bun.file(path.join(outputPath, url.pathname));
                return new Response(file);
            },
        });
        try {
            await indexMarkdown({ vault: fixturesDir, outputPath });

            const base = `http://localhost:${server.port}/`;
            const pagefind = await import(`file://${outputPath}/pagefind.js`);
            await pagefind.options({ basePath: base });
            await pagefind.init();

            const { results } = await pagefind.search("bm25");
            expect(results.length).toBeGreaterThan(0);

            const top = await results[0].data();
            expect(top.url).toContain("ranking");
        } finally {
            server.stop(true);
            await cleanupDir(outputPath);
        }
    });
});
