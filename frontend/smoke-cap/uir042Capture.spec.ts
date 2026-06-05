import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * HB-iamshenkui-GameMigration-42: Trinket Inventory (饰品仓库) screen capture.
 *
 * Navigates from town to the inventory screen and captures screenshots
 * for visual review of the migrated page.
 */
test("HB-42: capture trinket inventory screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Click the 饰品仓库 button
  await page.getByRole("button", { name: "饰品仓库", exact: true }).click();
  await page.waitForTimeout(800);

  // Verify inventory screen content
  await expect(page.locator(".inventory-title")).toHaveText("饰品仓库");
  await expect(page.locator(".inventory-hero-card")).toHaveCount(3);
  await expect(page.locator(".inventory-trinket-card")).toHaveCount(4);

  // Capture inventory screen
  await page.locator(".app-frame").screenshot({
    path: "test-results/hb042-inventory-screen.png"
  });

  // Return to town
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Verify back in town
  await expect(page.getByText("城镇中枢")).toBeVisible();
});
