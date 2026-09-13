import test from 'ava';

import { PagefindInstance } from "../lib/coupled_search";

(globalThis as any).pagefind_version = "0.0.0";
(globalThis as any).wasm_bindgen = {};

test('a failed index chunk fetch is retried by a later search', async t => {
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
