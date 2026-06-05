import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * Dungeon Items screen screenshot capture for visual review.
 *
 * Navigates to the dungeon items screen and captures it
 * at key visual states to verify game-style hierarchy fidelity.
 */
test("capture dungeon items screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Navigate to provisioning
  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Launch expedition
  await page.getByRole("button", { name: "Confirm & Launch Expedition" }).click();
  await page.waitForTimeout(400);

  // Launch to dungeon items
  await page.getByRole("button", { name: "Launch Expedition" }).click();
  await page.waitForTimeout(600);

  // Verify dungeon items screen content
  await expect(page.getByText("副本场景-物品")).toBeVisible();
  await expect(page.getByText("Dungeon Items")).toBeVisible();
  await expect(page.locator(".item-card")).toHaveCount(4);
  await expect(page.locator(".party-status-card")).toHaveCount(2);

  // Capture dungeon items screen
  await page.locator(".expedition-viewport").screenshot({
    path: "test-results/dungeon-items-screen.png"
  });

  // Continue to result
  await page.getByRole("button", { name: "Continue Expedition" }).click();
  await page.waitForTimeout(600);

  // Verify result screen
  await expect(page.getByText("Victory")).toBeVisible();
});
