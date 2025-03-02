<script lang="ts">
    export let node;
    export let indent = 0;

    let isExpanded = true;

    // 切换折叠状态
    function toggleExpand() {
        isExpanded = !isExpanded;
    }

    // 获取节点类型
    function getType<T>(value: T): String {
        if (Array.isArray(value)) return "array";
        if (typeof value === "object" && value !== null) return "object";
        return typeof value;
    }

    // 获取元素数量
    function getSize<T>(value: T): number {
        if (Array.isArray(value)) return value.length;
        if (typeof value === "object" && value !== null)
            return Object.keys(value).length;
        return 0;
    }

    const type = getType(node);
    const size = getSize(node);
</script>

<div style="margin-left: {indent * 20}px;">
    {#if type === "object" || type === "array"}
        <button on:click={toggleExpand} style="cursor: pointer;">
            {isExpanded ? "▼" : "▶"}
            {type === "object" ? "{" : "["}
        </button>
        {#if isExpanded}
            {#if type === "object"}
                {#each Object.entries(node) as [key, value]}
                    <div>
                        <span>"{key}":</span>
                        <svelte:self node={value} indent={indent + 1} />
                    </div>
                {/each}
                <span style="margin-left: {-20}px;">{"}"}</span>
            {:else if type === "array"}
                {#each node as item}
                    <div>
                        <svelte:self node={item} indent={indent + 1} />
                    </div>
                {/each}
                <span style="margin-left: {-20}px;">{"]"}</span>
            {/if}
        {:else}
            <span> ({size} {size === 1 ? "element" : "elements"})</span>
        {/if}
    {:else}
        <span>{JSON.stringify(node)}</span>
    {/if}
</div>
