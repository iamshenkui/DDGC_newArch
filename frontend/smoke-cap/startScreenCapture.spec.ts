import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * Start Screen (开始界面) screenshot capture and verification.
 *
 * Reference: reference/ref_image/跨际元契约/1系统界面/开始界面.png
 * Validates the migrated start screen matches the reference layout:
 *   - Title "跨纪元契约" with sword-cross ornament
 *   - Parchment-style menu with four options
 *   - Version label "V 1.0"
 *   - Dev tools bar (Boot Replay, Boot Live)
 */
test("开始界面: capture and verify start screen fidelity", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Verify title is visible
  await expect(
    page.getByRole("heading", { name: "跨纪元契约" }),
    "Start screen title must be visible"
  ).toBeVisible();

  // Verify parchment menu buttons
  await expect(
    page.locator('[data-testid="start-new-game"]'),
    "New Game button (新游戏) must be present"
  ).toBeVisible();

  await expect(
    page.locator('[data-testid="start-continue"]'),
    "Continue button must be present"
  ).toBeVisible();

  await expect(
    page.locator('[data-testid="start-settings"]'),
    "Settings button (设置) must be present"
  ).toBeVisible();

  await expect(
    page.locator('[data-testid="start-exit"]'),
    "Exit button (退出游戏) must be present"
  ).toBeVisible();

  // Verify version label
  await expect(
    page.locator(".start-screen-version"),
    "Version label must show V 1.0"
  ).toHaveText("V 1.0");

  // Verify dev tools are present
  await expect(
    page.locator('[data-testid="boot-replay"]'),
    "Boot Replay dev button must be present"
  ).toBeVisible();

  await expect(
    page.locator('[data-testid="boot-live"]'),
    "Boot Live dev button must be present"
  ).toBeVisible();

  // Capture start screen for visual review
  await page.locator(".start-screen").screenshot({
    path: "test-results/start-screen-capture.png"
  });

  // Full page capture as well
  await page.screenshot({
    path: "test-results/start-screen-full.png",
    fullPage: false
  });
});
