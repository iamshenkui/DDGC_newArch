import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * UIR-013: Abbey (Faith Altar / 信仰祭坛) building screen screenshot captures.
 *
 * Navigates to the Abbey building from town and captures the dedicated
 * building screen for visual review of faith-altar UI fidelity.
 */
test("UIR-013: capture abbey building screen from replay", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open the Abbey building (信仰祭坛)
  await page.locator('[data-building-id="abbey"]').click();
  await page.waitForTimeout(800);

  // Capture abbey building screen
  await page.locator(".app-frame").screenshot({
    path: "test-results/uir013-abbey-building-screen.png",
    fullPage: false
  });

  // Verify abbey-specific content
  await expect(
    page.locator(".building-detail-name"),
    "Abbey building name must be visible (DDGC display name 信仰祭坛)"
  ).toHaveText("信仰祭坛");

  await expect(
    page.locator(".building-action-card-header").filter({ hasText: "祈祷" }),
    "Prayer action must be visible"
  ).toBeVisible();

  await expect(
    page.getByRole("button", { name: "Return to Town" }),
    "Return to Town button must be visible"
  ).toBeVisible();
});

test("UIR-013: capture abbey building screen from live", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Live" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open the Abbey building (信仰祭坛)
  await page.locator('[data-building-id="abbey"]').click();
  await page.waitForTimeout(800);

  // Capture abbey building screen
  await page.locator(".app-frame").screenshot({
    path: "test-results/uir013-abbey-building-live.png",
    fullPage: false
  });

  // Verify abbey-specific content
  await expect(
    page.locator(".building-detail-name"),
    "Abbey building name must be visible on live"
  ).toHaveText("信仰祭坛");

  await expect(
    page.getByRole("button", { name: "Return to Town" }),
    "Return to Town button must be visible on live"
  ).toBeVisible();
});
