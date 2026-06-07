import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * UIR-009: result and return screen screenshot captures for visual review.
 *
 * Navigates the full meta-loop and captures result and return screens
 * at key visual states to verify game-style hierarchy fidelity.
 */
test("UIR-009: capture result and return screens", async ({ page }) => {
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

  await page.getByRole("button", { name: "Proceed to Provisioning" }).click();
  await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Launch expedition
  await page.locator('[data-testid="footer-btn-launch"]').click();
  await page.waitForTimeout(400);

  // Launch to result (through dungeon-assist → dungeon-map → combat)
  await page.locator('[data-testid="expedition-btn-launch"]').click();
  await page.waitForSelector(".dungeon-assist-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await page.getByTestId("assist-action-heal-wound").click();
  await page.waitForTimeout(400);
  await page.getByRole("button", { name: "Continue Expedition" }).click();
  await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await page.locator('[data-room-id="room-combat-1"] .dungeon-room-btn').click();
  await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await page.getByRole("button", { name: "Confirm Attack" }).click();
  await page.waitForTimeout(400);
  await page.getByRole("button", { name: "Acknowledge" }).click();
  await page.waitForTimeout(400);

  // Capture result screen (success outcome)
  await page.locator(".expedition-viewport").screenshot({
    path: "test-results/uir009-result-screen.png"
  });

  // Verify result screen content
  await expect(page.getByText("Victory")).toBeVisible();
  await expect(page.getByText("Proceed to Return")).toBeVisible();
  await expect(page.getByText("Return to Town")).toBeVisible();

  // Proceed to return screen
  await page.getByRole("button", { name: "Proceed to Return" }).click();
  await page.waitForTimeout(600);

  // Capture return screen
  await page.locator(".expedition-viewport").screenshot({
    path: "test-results/uir009-return-screen.png"
  });

  // Verify return screen content
  await expect(page.getByText("Expedition Log Closed")).toBeVisible();
  await expect(page.getByText("Resume Town Activities")).toBeVisible();

  // Resume to town — close the meta-loop
  await page.getByRole("button", { name: "Resume Town Activities" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Verify back in town
  await expect(page.getByText("城镇中枢")).toBeVisible();

  // Full-page result capture as well
  await page.screenshot({
    path: "test-results/uir009-meta-loop-complete.png",
    fullPage: false
  });
});
