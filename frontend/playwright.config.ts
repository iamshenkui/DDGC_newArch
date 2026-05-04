import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./smoke",
  timeout: 90_000,
  fullyParallel: false,
  retries: 0,
  use: {
    baseURL: "http://localhost:4179",
    headless: true,
    viewport: { width: 1440, height: 900 }
  },
  webServer: {
    command: "npm run preview",
    port: 4179,
    reuseExistingServer: !process.env.CI,
    timeout: 15_000
  }
});
