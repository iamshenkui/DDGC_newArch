import { test } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * Blacksmith building screenshot capture for visual review.
 *
 * Captures the migrated page: 公会界面-锻造仓-使用空
 */
test("capture blacksmith forge empty usage screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open blacksmith building
  await page.locator('[data-building-id="blacksmith"]').click();
  await page.waitForTimeout(800);

  // Capture the full blacksmith screen
  await page.screenshot({
    path: "test-results/blacksmith-forge-empty-usage.png",
    fullPage: false
  });

  // Capture just the forge viewport
  await page.locator(".forge-viewport").screenshot({
    path: "test-results/blacksmith-forge-viewport.png"
  });
});
