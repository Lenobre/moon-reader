<script lang="ts">
  import Sidebar from "$lib/components/sidebar/Sidebar.svelte";
  import "./layout.css";

  const { children } = $props();
</script>

<div class="flex flex-row w-full h-screen bg-[#0B0E14] text-white antialiased">
  <Sidebar />
  {@render children()}
</div>
