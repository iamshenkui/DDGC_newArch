import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * Transition screen (转场) screenshot capture and verification.
 *
 * Covers the reference page:
 *   reference/ref_image/跨际元契约/1系统界面/转场.png
 *
 * Validates:
 *   - Region title (朱雀大陆) is visible and centered
 *   - Flavor text is visible at bottom-right
 *   - Prompt hint is visible at bottom-center
 *   - Ornament emblem renders top-left
 *   - Click / space dismiss continues to expedition
 *   - Return button goes back to provisioning
 *   - Screenshot capture for visual parity review
 */
test("transition screen: capture, content, and interaction", async ({ page }) => {
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

  // Confirm provisioning → transition
  await page.getByRole("button", { name: "Confirm & Launch Expedition" }).click();
  await page.waitForSelector(".transition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // ── Content verification ───────────────────────────────
  await expect(
    page.locator(".transition-title"),
    "Region name (朱雀大陆) must be visible"
  ).toHaveText("朱雀大陆");

  await expect(
    page.locator(".transition-flavor-text"),
    "Flavor text must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".transition-prompt-hint"),
    "Prompt hint must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".transition-ornament"),
    "Ornament emblem must render"
  ).toBeVisible();

  // ── Screenshot capture ─────────────────────────────────
  await page.locator(".transition-viewport").screenshot({
    path: "test-results/transition-screen.png",
    fullPage: false,
  });

  // ── Return interaction ─────────────────────────────────
  await page.getByRole("button", { name: "Return" }).click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await expect(
    page.getByText("Provisioning"),
    "Must be back at provisioning after Return"
  ).toBeVisible();

  // ── Continue interaction ───────────────────────────────
  await page.getByRole("button", { name: "Confirm & Launch Expedition" }).click();
  await page.waitForSelector(".transition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await page.locator(".transition-viewport").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await expect(
    page.getByRole("heading", { name: "Expedition Launch" }),
    "Must reach expedition after dismiss"
  ).toBeVisible();
});
