<script lang="ts">
  import JSONFormat from "../components/JSONFormat.svelte";
  // import JSONCompare from '../components/JSONCompare.svelte';
  // import Base64Encode from '../components/Base64Encode.svelte';
  // import Base64Decode from '../components/Base64Decode.svelte';
  // import ProtobufDecode from '../components/ProtobufDecode.svelte';
  import { invoke } from "@tauri-apps/api/core";

  // let name = $state("");
  // let greetMsg = $state("");

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }

  let activeTool = "jsonFormat";
</script>

<main class="container">
  <nav>
    <button onclick={() => (activeTool = "jsonFormat")}>JSON Format</button>
    <!-- <button on:click={() => activeTool = 'jsonCompare'}>JSON Compare</button>
    <button on:click={() => activeTool = 'base64Encode'}>Base64 Encode</button>
    <button on:click={() => activeTool = 'base64Decode'}>Base64 Decode</button>
    <button on:click={() => activeTool = 'protobufDecode'}>Protobuf Decode</button> -->
  </nav>

  <div style="height: 100vh;">
    <JSONFormat />
  </div>
</main>

<style>
  nav {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }
  button {
    padding: 10px;
  }

  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  button {
    outline: none;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
