<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // let name = $state("");
  // let greetMsg = $state("");
  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }

  let activeTool = "jsonFormat";
  let drawerHidden = false;
</script>

<!-- 
<Button class="bg-blue-500!">Blue Button</Button>
<Button class="bg-green-500!">Green Button</Button> -->

<style lang="postcss">
  @reference "tailwindcss/theme";
  :global(html) {
    background-color: theme(--color-gray-100);
  }
</style>
