import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  server: { watch: { usePolling: true } },
  plugins: [
    react(
      // for decorator
      { babel: { parserOpts: { plugins: ["decorators-legacy"] } } }
    ),
    vitepwa
  ],
  build: {
    rollupOptions: { output: { manualChunks: undefined } },
    chunkSizeWarningLimit: 1024 * 1,
  },
});
