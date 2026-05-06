import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./smoke-cap",
  timeout: 60_000,
  fullyParallel: false,
  retries: 0,
  use: {
    baseURL: "http://localhost:4179",
    headless: true,
    viewport: { width: 1600, height: 1000 }
  },
  webServer: {
    command: "npm run preview",
    port: 4179,
    reuseExistingServer: true,
    timeout: 15_000
  }
});
