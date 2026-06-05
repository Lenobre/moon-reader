<script lang="ts">
  import { Plus } from "@lucide/svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { BaseDirectory, copyFile, mkdir } from "@tauri-apps/plugin-fs";

  async function whenButtonClicked() {
    const files = await open({
      multiple: true,
      directory: false,
    });

    if (!files || files.length === 0)
      return console.info("No files selected for uploading");

    files.forEach(async (file) => {
      await copyFile(file, "plugins/teste", {
        toPathBaseDir: BaseDirectory.AppData,
      });
    });
  }
</script>

<button
  onclick={whenButtonClicked}
  class="group relative bg-[#0E121A] border-2 border-dashed border-[#222838] hover:border-[#5e30b3] rounded-xl w-72 h-44 flex flex-col items-center justify-center gap-3 transition-all duration-300 hover:shadow-[0_0_20px_-5px_#7439E8] cursor-pointer"
>
  <!-- Subtle gradient overlay on hover -->
  <div
    class="absolute inset-0 rounded-xl bg-gradient-to-br from-[#7639E4]/20 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"
  ></div>

  <!-- Icon Circle -->
  <div
    class="relative z-10 flex items-center justify-center w-12 h-12 rounded-full bg-[#222838] group-hover:bg-[#7439E8]/20 transition-colors duration-300"
  >
    <Plus
      class="w-6 h-6 text-[#8B7EB5] group-hover:text-[#7439E8] transition-colors duration-300"
    />
  </div>

  <!-- Text -->
  <span
    class="relative z-10 text-sm font-semibold text-[#8B7EB5] group-hover:text-white transition-colors duration-300"
  >
    Add new source
  </span>
</button>
