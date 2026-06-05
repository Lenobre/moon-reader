<script>
  import AddSource from "$lib/components/AddSource.svelte";
  import Source from "$lib/components/Source.svelte";

  function selectSource() {}
</script>

<div
  class="flex flex-col items-center justify-center w-full h-screen text-white antialiased p-8"
>
  <!-- Header -->
  <div class="mb-10 text-center">
    <h1 class="text-3xl font-bold text-white mb-2">Your Reading Sources</h1>
    <p class="text-[#8B7EB5] text-sm">
      Manage and access your favorite manga & webtoon sources
    </p>
  </div>

  <!-- Sources Grid -->
  <div
    class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5"
  >
    <Source to="/" link="https://github.com/lecndy/moon-reader" />
    <Source to="/" link="https://github.com/lecndy/moon-reader" />
    <Source to="/" link="https://github.com/lecndy/moon-reader" />
    <AddSource />
  </div>
</div>
