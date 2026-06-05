import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * HB-iamshenkui-GameMigration-17: Garden building screen screenshot capture.
 *
 * Navigates to the garden building (天国花园) from the town shell and captures
 * the dedicated GardenBuildingScreen at key visual states to verify fidelity.
 */
test("HB-17: capture garden building screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Click on the garden building node in the town estate
  await page.locator('[data-building-id="garden"]').click();
  await page.waitForTimeout(600);

  // Verify garden building screen is rendered
  await expect(page.getByText("天国花园")).toBeVisible();
  await expect(page.getByText("使用设施")).toBeVisible();
  await expect(page.getByText("选择人物")).toBeVisible();

  // Capture the garden building screen (Use Facilities tab)
  await page.screenshot({
    path: "test-results/hb17-garden-use-facilities.png",
    fullPage: false
  });

  // Switch to Upgrade Facilities tab and capture
  await page.getByRole("button", { name: "升级设施" }).click();
  await page.waitForTimeout(300);

  await expect(page.getByText("升级设施")).toBeVisible();

  await page.screenshot({
    path: "test-results/hb17-garden-upgrade-facilities.png",
    fullPage: false
  });

  // Return to town
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Verify back in town
  await expect(page.getByText("城镇中枢")).toBeVisible();
});
