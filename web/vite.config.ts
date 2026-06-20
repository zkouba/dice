import { defineConfig } from "vite";

export default defineConfig({
  // Relative base so the built static bundle works when served from any
  // sub-path (e.g. a GitHub Pages project URL), not just the domain root.
  base: "./",
});
