import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./smoke-cap",
  timeout: 60_000,
  retries: 0,
  use: {
    baseURL: "http://localhost:4179",
    headless: true,
    viewport: { width: 1440, height: 900 }
  }
});
