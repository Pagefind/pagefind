import test from 'ava';

import { PagefindInstance } from "../lib/coupled_search";

(globalThis as any).pagefind_version = "0.0.0";
(globalThis as any).wasm_bindgen = {};

test.serial('a failed index chunk fetch is retried by a later search', async t => {
    const instance = new PagefindInstance({ basePath: "/pagefind/" });
    instance.raw_ptr = 1;

    const loaded_chunks: string[] = [];
    instance.backend = {
        load_index_chunk: (_ptr: number, chunk: Uint8Array) => {
            loaded_chunks.push(new TextDecoder().decode(chunk));
            return 1;
        },
    };

    let requests = 0;
    globalThis.fetch = async () => {
        requests += 1;
        if (requests === 1) {
            return new Response("<h1>404 Not Found</h1>", { status: 404 });
        }
        return new Response(new TextEncoder().encode("pagefind_dcdchunk"));
    };

    await instance.loadChunk("aaaa");
    t.deepEqual(loaded_chunks, []);

    await instance.loadChunk("aaaa");
    t.deepEqual(loaded_chunks, ["chunk"]);
});

test.serial('a failed fragment group fetch is retried by a later search', async t => {
    const instance = new PagefindInstance({ basePath: "/pagefind/" });

    // A fragment group is a single JSON object of page hash -> fragment
    const fetches: string[] = [];
    const group_body = JSON.stringify({
        aaa: { content: "First grouped fragment", url: "/grouped/", anchors: [], meta: {} },
        ccc: { content: "Second grouped fragment", url: "/grouped/", anchors: [], meta: {} },
    });
    globalThis.fetch = async (input) => {
        fetches.push(input.toString());
        if (fetches.length === 1) {
            return new Response("<h1>404 Not Found</h1>", { status: 404 });
        }
        return new Response(new TextEncoder().encode(`pagefind_dcd${group_body}`), { status: 200 });
    };

    await t.throwsAsync(() => instance.loadFragment("aaa", "en_group_bbb"));

    // A different page from the same group must refetch the group, not reuse the failure
    const fragment = await instance.loadFragment("ccc", "en_group_bbb");
    t.is(fragment.url, "/grouped/");
    t.is(fetches.length, 2);
    t.is(fetches[1], "/pagefind/fragment/en_group_bbb.pf_fragment");

    // Both pages are then served from the single cached group fetch
    await instance.loadFragment("aaa", "en_group_bbb");
    t.is(fetches.length, 2);
});
