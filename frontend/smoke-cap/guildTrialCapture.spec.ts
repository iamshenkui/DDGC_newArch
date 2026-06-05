import { test } from "@playwright/test";

test("guild-trial-selection replay capture", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);

  // Open Guild building
  await page.locator('[data-building-id="guild"]').click();
  await page.waitForTimeout(800);

  // Click "使用设施" tab
  await page.locator('[data-tab-id="use"]').click();
  await page.waitForTimeout(600);

  await page.screenshot({
    path: "/tmp/guild-trial-selection-replay.png",
    fullPage: false
  });
});

test("guild-trial-selection live capture", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Live" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);

  // Open Guild building
  await page.locator('[data-building-id="guild"]').click();
  await page.waitForTimeout(800);

  // Click "使用设施" tab
  await page.locator('[data-tab-id="use"]').click();
  await page.waitForTimeout(600);

  await page.screenshot({
    path: "/tmp/guild-trial-selection-live.png",
    fullPage: false
  });
});
