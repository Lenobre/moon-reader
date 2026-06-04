<script lang="ts">
  import { Link, Trash } from "@lucide/svelte";

  const {
    to,
    link,
  }: {
    to: string;
    link: string;
  } = $props();
</script>

<div
  class="text-sm text-white transition-colors duration-200 bg-[#171920] border-[#2f333f] border p-4 rounded-lg w-64 h-36 flex flex-col"
>
  <!-- Header -->
  <div class="flex gap-2">
    <!-- Logo -->
    <img
      src="https://imgs.search.brave.com/0gTyyKGte0WL-k58A__GupGBMOgXkkb8TSgsoB9BG14/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9zc2wu/cHN0YXRpYy5uZXQv/c3RhdGljL3dlYy8y/MDI2MDUyMS9zbGlk/ZS9XRUJUT09OUHJv/ZHVjdGlvbi5wbmc_/dz0xMDAmcT0y"
      alt="LOGO"
      class="w-14 object-contain"
    />

    <div class="flex w-full">
      <div class="flex flex-col gap-1">
        <!-- Title  -->
        <span class="ml-2 font-semibold text-lg">Webtoon</span>

        <!-- Links -->
        {#if link}
          <a
            href={link}
            target="_blank"
            rel="noopener noreferrer"
            title="External Link"
          >
            <Link class="text-blue-500 w-5" />
          </a>
        {/if}
      </div>

      <!-- Delete -->
      <button class="ml-auto h-max">
        <Trash class="text-red-500 hover:text-red-700 transition-colors w-7" />
      </button>
    </div>
  </div>

  <!-- Redirect -->
  <a
    href={to}
    class="bg-[#7439E8] p-2 rounded-sm font-semibold text-center mt-auto"
    >Read from this</a
  >
</div>
