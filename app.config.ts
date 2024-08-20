import { defineConfig } from "@solidjs/start/config";

export default defineConfig({
  ssr: false,
  vite: {
    envPrefix: ["VITE_", "TAURI_"],
  },
});
