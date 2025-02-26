<script lang="ts">
  import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
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

<!-- <main class="container">
  <h1 class="text-3xl font-bold underline">Hello world!</h1>
  <nav>
    <button onclick={() => (activeTool = "jsonFormat")}>JSON Format</button>
    <button on:click={() => activeTool = 'jsonCompare'}>JSON Compare</button>
    <button on:click={() => activeTool = 'base64Encode'}>Base64 Encode</button>
    <button on:click={() => activeTool = 'base64Decode'}>Base64 Decode</button>
    <button on:click={() => activeTool = 'protobufDecode'}>Protobuf Decode</button> 
  </nav>

  <div style="height: 100vh;">
    <JSONFormat />
  </div>
</main> -->

<Navbar>
  <NavBrand href="/">
    <img src="/images/flowbite-svelte-icon-logo.svg" class="me-3 h-6 sm:h-9" alt="Flowbite Logo" />
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">Flowbite</span>
  </NavBrand>
  <NavHamburger  />
  <NavUl >
    <NavLi href="/">Home</NavLi>
    <NavLi href="/about">About</NavLi>
    <NavLi href="/docs/components/navbar">Navbar</NavLi>
    <NavLi href="/pricing">Pricing</NavLi>
    <NavLi href="/contact">Contact</NavLi>
  </NavUl>
</Navbar>

<style lang="postcss">
  @reference "tailwindcss/theme";
  :global(html) {
    background-color: theme(--color-gray-100);
  }
</style>
