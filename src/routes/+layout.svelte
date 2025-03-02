<script lang="ts">
    let { children } = $props();
    import "../app.css";

    import Sidebar from "./sidebar.svelte";
    import Navbar from "./navbar.svelte";

    let drawerHidden = $state(false);
</script>

<!-- {@render children()} -->

<header
  class="fixed top-0 z-40 mx-auto w-full flex-none border-b border-gray-200 bg-white dark:border-gray-600 dark:bg-gray-800"
>
  <Navbar bind:drawerHidden />
</header>
<div class="overflow-hidden lg:flex">
  <Sidebar bind:drawerHidden />
  <div class="relative h-full w-full overflow-y-auto lg:ml-64 pt-[70px]">
    {@render children()}
  </div>
</div> 