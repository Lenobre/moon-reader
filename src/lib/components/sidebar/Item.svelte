<script lang="ts">
  import { page } from "$app/state";

  const { icon, title, to } = $props();

  const style =
    page.url.pathname === to
      ? "from-[#7639E4]/20 border-[#5e30b3]"
      : "border-[#0E121A] hover:from-[#7639E4]/20";
</script>

<li
  class="{style}
  hover:border-[#5e30b3] bg-gradient-to-r border-l-2 rounded-lg px-4 py-2 cursor-pointer transition-all duration-300"
>
  <a href={to} class="font-semibold flex items-center gap-2">
    {@render icon?.()}

    {@render title?.()}
  </a>
</li>
