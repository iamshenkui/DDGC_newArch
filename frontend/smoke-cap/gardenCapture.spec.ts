import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * UIR-015: Garden (天国花园) building screen capture for visual review.
 *
 * Navigates to the garden building from town and captures the
 * "使用空" (empty use) state with empty hero rest slots.
 */
test("UIR-015: capture garden building screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Click on the garden building (天国花园)
  await page.locator('[data-building-id="garden"]').click();
  await page.waitForTimeout(800);

  // Verify garden screen content
  await expect(page.locator(".building-detail-name")).toHaveText("天国花园");
  await expect(page.locator(".building-action-section-title").filter({ hasText: "休养位" })).toBeVisible();

  // Verify empty slots (使用空 state)
  const emptySlots = page.locator('[data-slot-state="empty"]');
  await expect(emptySlots).toHaveCount(3);

  // Verify empty slot labels
  await expect(page.getByText("花园休养位 1")).toBeVisible();
  await expect(page.getByText("花园休养位 2")).toBeVisible();
  await expect(page.getByText("花园休养位 3")).toBeVisible();
  await expect(page.getByText("空闲").first()).toBeVisible();

  // Verify "放置英雄" button on empty slots
  await expect(page.locator('[data-action="place-hero"]')).toHaveCount(3);

  // Capture garden screen
  await page.locator(".app-frame").screenshot({
    path: "test-results/uir015-garden-screen.png"
  });

  // Return to town
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Verify back in town
  await expect(page.getByText("城镇中枢")).toBeVisible();
});
