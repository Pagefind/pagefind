<script lang="ts">
    let {
        runSearch,
        searchKeywords,
        queryTermIdfs,
    }: {
        runSearch: (term: string) => void;
        searchKeywords: string[];
        queryTermIdfs: PagefindQueryTermIdf[];
    } = $props();

    const handleInput = async (e: Event) => {
        if (e.target instanceof HTMLInputElement) {
            runSearch(e.target.value);
        }
    };

    let totalIdf = $derived(
        queryTermIdfs.reduce((sum, q) => sum + q.idf, 0),
    );
</script>

<label for="#search">Search</label>
<input
    type="search"
    id="search"
    placeholder="Search term..."
    oninput={handleInput}
/>

<p>
    {#if searchKeywords.length}Stemmed search keywords: <code class="kw"
            >{searchKeywords.join(", ")}</code
        >{/if}
    &nbsp;
</p>

{#if queryTermIdfs.length}
    <details class="idf-breakdown">
        <summary
            >Query IDF Breakdown (Total: <span class="hl"
                >{totalIdf.toFixed(4)}</span
            >)</summary
        >
        <div class="inner">
            <table>
                <thead>
                    <tr>
                        <th>Term</th>
                        <th>IDF</th>
                        <th>% of Total</th>
                    </tr>
                </thead>
                <tbody>
                    {#each queryTermIdfs as q}
                        <tr>
                            <td>{q.term}</td>
                            <td>{q.idf.toFixed(4)}</td>
                            <td class="hl"
                                >{((q.idf / totalIdf) * 100).toFixed(1)}%</td
                            >
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </details>
{/if}

<style>
    label {
        position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        border: 0;
    }

    input {
        box-sizing: border-box;
        color: var(--fg);
        border: solid 1px var(--fg);
        background-color: var(--bg);
        padding: 0 6px;
        height: 36px;
        width: 100%;
        font-size: 16px;
    }
    input::placeholder {
        color: var(--fg);
        opacity: 0.5;
    }

    p {
        margin: 8px 0 0 0;
    }

    .kw {
        color: var(--hl);
    }

    .idf-breakdown {
        margin-top: 8px;
        border-left: solid 2px var(--sub-fg);
        padding-left: 8px;
    }

    summary {
        font-size: var(--sfz);
        cursor: pointer;
        color: var(--sub-fg);
        list-style-type: none;
    }

    summary::after {
        content: " [+]";
    }

    .idf-breakdown[open] {
        border-color: var(--hl);
    }

    .idf-breakdown:has(summary:hover) {
        border-color: var(--hl);
    }

    .idf-breakdown[open] summary {
        color: var(--hl);
    }

    .idf-breakdown[open] summary::after {
        content: " [-]";
    }

    .inner {
        max-width: 100%;
        overflow-x: scroll;
    }

    table {
        border-collapse: collapse;
        margin-top: 8px;
    }

    tbody tr:nth-child(odd) {
        background-color: transparent;
    }

    tbody tr:nth-child(even) {
        background-color: var(--sub-bg);
    }

    td,
    th {
        padding-right: 8px;
        text-align: left;
    }

    .hl {
        color: var(--hl);
    }
</style>
