<script>
  import { Globe, House } from "@lucide/svelte";
  import Item from "./Item.svelte";
</script>

<nav class="bg-[#0E121A] w-64 border-[#222838] border-r">
  <ul class="py-4 px-2 flex flex-col gap-3">
    <Item to="/">
      {#snippet icon()}
        <House></House>
      {/snippet}

      {#snippet title()}
        Home
      {/snippet}
    </Item>

    <Item to="/sources">
      {#snippet icon()}
        <Globe></Globe>
      {/snippet}

      {#snippet title()}
        Fontes
      {/snippet}
    </Item>
  </ul>
</nav>
