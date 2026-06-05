import { test, expect } from "@playwright/test";

test("hb-19 replay guild upgrade capture", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);

  // Open the guild building (次元感知塔)
  await page.locator('[data-building-id="guild"]').click();
  await page.waitForSelector(".guild-building-screen", { timeout: 5_000 });
  await page.waitForTimeout(600);

  // Verify key elements before capture
  await expect(page.locator(".guild-building-name")).toHaveText("次元感知塔");
  await expect(page.locator('.guild-tab-btn--active[data-tab-id="upgrades"]')).toBeVisible();
  await expect(page.locator('[data-tree-id="guild_training"]')).toBeVisible();
  await expect(page.locator('[data-tree-id="guild_skills"]')).toBeVisible();
  await expect(page.locator('[data-tree-id="guild_capacity"]')).toBeVisible();
  await expect(page.locator(".guild-currency-strip")).toBeVisible();

  // Capture the guild upgrade screen
  await page.screenshot({ path: "/tmp/hb-19-cap/guild-upgrade-replay.png", fullPage: false });
});

test("hb-19 live guild upgrade capture", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Live" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);

  // Open the guild building (次元感知塔)
  await page.locator('[data-building-id="guild"]').click();
  await page.waitForSelector(".guild-building-screen", { timeout: 5_000 });
  await page.waitForTimeout(600);

  // Verify key elements before capture
  await expect(page.locator(".guild-building-name")).toHaveText("次元感知塔");
  await expect(page.locator('.guild-tab-btn--active[data-tab-id="upgrades"]')).toBeVisible();
  await expect(page.locator(".guild-currency-strip")).toBeVisible();

  // Capture the guild upgrade screen
  await page.screenshot({ path: "/tmp/hb-19-cap/guild-upgrade-live.png", fullPage: false });
});
