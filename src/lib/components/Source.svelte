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
  class="group relative bg-[#0E121A] border border-[#222838] hover:border-[#5e30b3] rounded-xl p-5 w-72 h-44 flex flex-col transition-all duration-300 hover:shadow-[0_0_20px_-5px_#7439E8]"
>
  <!-- Subtle gradient overlay on hover -->
  <div
    class="absolute inset-0 rounded-xl bg-gradient-to-br from-[#7639E4]/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"
  ></div>

  <!-- Header -->
  <div class="flex gap-3 relative z-10">
    <!-- Logo -->
    <img
      src="https://imgs.search.brave.com/0gTyyKGte0WL-k58A__GupGBMOgXkkb8TSgsoB9BG14/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9zc2wu/cHN0YXRpYy5uZXQv/c3RhdGljL3dlYy8y/MDI2MDUyMS9zbGlkZS9XRUJUT09OUHJv/ZHVjdGlvbi5wbmc_/dz0xMDAmcT0y"
      alt="LOGO"
      class="w-12 h-12 object-contain rounded-lg"
    />

    <div class="flex w-full min-w-0">
      <div class="flex flex-col gap-0.5 min-w-0">
        <!-- Title -->
        <span class="font-semibold text-base text-white truncate">Webtoon</span>

        <!-- Links -->
        {#if link}
          <a
            href={link}
            target="_blank"
            rel="noopener noreferrer"
            title="External Link"
            class="text-[#8B7EB5] hover:text-[#7439E8] transition-colors w-max"
          >
            <span class="text-xs flex items-center gap-1">
              <Link class="w-3.5" />
              Visit source
            </span>
          </a>
        {/if}
      </div>

      <!-- Delete -->
      <button class="ml-auto h-max mt-0.5">
        <Trash
          class="text-[#4A4D5A] hover:text-red-400 transition-all duration-200 w-5 h-5 hover:scale-110"
        />
      </button>
    </div>
  </div>

  <!-- Redirect -->
  <a
    href={to}
    class="relative z-10 bg-[#7439E8] hover:bg-[#8B5CF6] text-white text-sm font-semibold text-center py-2.5 px-4 rounded-lg mt-auto transition-all duration-200 hover:shadow-[0_0_16px_-3px_#7439E8] active:scale-[0.98]"
  >
    Read from this source
  </a>
</div>
