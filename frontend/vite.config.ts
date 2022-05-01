import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(
      // for decorator
      { babel: { parserOpts: { plugins: ["decorators-legacy"] } } }
    ),
  ],
  build: {
    rollupOptions: { output: { manualChunks: undefined } },
    chunkSizeWarningLimit: 1024 * 1,
  },
});
