import { resolve } from "node:path";

import solid from "vite-plugin-solid";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [solid()],
  resolve: {
    alias: {
      "@contracts/app-shell": resolve(__dirname, "./src/contracts/app-shell.ts"),
      "@contracts/ui-substrate": resolve(__dirname, "./src/contracts/ui-substrate.ts"),
      "@contracts/pixi-renderer": resolve(__dirname, "./src/contracts/pixi-renderer.ts"),
      "@contracts/spine-bridge": resolve(__dirname, "./src/contracts/spine-bridge.ts"),
      "@contracts/replay-inspector": resolve(__dirname, "./src/contracts/replay-inspector.ts")
    }
  },
  server: {
    port: 4179,
    host: "0.0.0.0"
  }
});