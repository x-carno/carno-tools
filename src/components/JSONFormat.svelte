<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let inputJson = "";
    let outputJson = "";

    async function formatJson() {
        try {
            outputJson = await invoke("format_json", { json: inputJson });
        } catch (error) {
            outputJson = `Error: ${error}`;
        }
    }
</script>

<div class="container">
    <!-- <h2>JSON Format</h2> -->
    <textarea class="left" bind:value={inputJson} placeholder="Enter JSON"
    ></textarea>
    <button on:click={formatJson}>Format</button>
    <textarea
        class="right"
        bind:value={outputJson}
        readonly
        placeholder="Formatted JSON"
    ></textarea>
</div>

<style>
    .container {
        display: grid;
        grid-template-columns: 1fr auto 1fr;
        gap: 10px;
        height: 100%; /* 可根据需要调整高度 */
    }

    .left,
    .right {
        resize: none; /* 禁止调整大小 */
        width: 100%; /* 填满网格单元宽度 */
        height: 100%; /* 填满网格单元高度 */
        box-sizing: border-box; /* 确保边框和内边距不影响尺寸 */
    }

    button {
        padding: 10px 20px; /* 美化按钮 */
    }
</style>
