import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * UIR-011: Sanitarium (细胞修复站) select-2 screen screenshot capture.
 *
 * Navigates to the sanitarium building, opens the usage tab, clicks the
 * disease-treatment slot, and captures the "select 2" modal for visual
 * acceptance against the reference image.
 */
test("UIR-011: capture sanitarium select-2 screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay → town
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open sanitarium building
  await page.locator('[data-building-id="sanitarium"]').click();
  await page.waitForTimeout(600);

  // Verify sanitarium building detail renders
  await expect(
    page.locator(".building-detail-name"),
    "Sanitarium building name must be visible"
  ).toHaveText("细胞修复站");

  // Verify usage tab is present and click it
  await expect(
    page.locator('[data-testid="tab-usage"]'),
    "Usage tab must be visible"
  ).toBeVisible();

  // The usage tab should be active by default; verify the usage panel
  await expect(
    page.locator('[data-testid="sanitarium-usage-panel"]'),
    "Usage panel must render"
  ).toBeVisible();

  // Verify both slot cards are present
  await expect(
    page.locator('[data-testid="slot-quirk-treatment"]'),
    "Quirk treatment slot must render"
  ).toBeVisible();
  await expect(
    page.locator('[data-testid="slot-disease-treatment"]'),
    "Disease treatment slot must render"
  ).toBeVisible();

  // Capture the main sanitarium usage screen
  await page.screenshot({
    path: "test-results/uir011-sanitarium-usage.png",
    fullPage: false,
  });

  // Click the disease-treatment slot to open select-2 modal
  await page.locator('[data-testid="slot-disease-cell-0"]').click();
  await page.waitForTimeout(300);

  // Verify modal opens
  await expect(
    page.locator('[data-testid="sanitarium-modal"]'),
    "Select-2 modal must open"
  ).toBeVisible();

  // Verify modal title matches reference
  await expect(
    page.locator(".sanitarium-modal-title"),
    "Modal title must match reference"
  ).toHaveText("选择要治疗的神降");

  // Verify hero rows are present
  await expect(
    page.locator('[data-testid^="hero-row-"]'),
    "Hero selection rows must render"
  ).toHaveCount(3);

  // Select a hero
  await page.locator('[data-testid="hero-row-hero-hunter-01"]').click();
  await page.waitForTimeout(200);

  // Verify disease preview appears
  await expect(
    page.locator('[data-testid="disease-preview"]'),
    "Disease preview must appear after hero selection"
  ).toBeVisible();

  // Capture the select-2 modal
  await page.locator(".sanitarium-modal").screenshot({
    path: "test-results/uir011-sanitarium-select2-modal.png",
  });

  // Click confirm to proceed to treatment confirmation
  await page.locator('[data-testid="modal-confirm"]').click();
  await page.waitForTimeout(200);

  // Capture the confirm-treatment state
  await page.locator(".sanitarium-modal").screenshot({
    path: "test-results/uir011-sanitarium-confirm-treatment.png",
  });

  // Close modal and return to town
  await page.locator('[data-testid="modal-close"]').click();
  await page.waitForTimeout(200);

  await expect(
    page.locator('[data-testid="sanitarium-modal"]'),
    "Modal must close"
  ).not.toBeVisible();

  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  // Verify back in town
  await expect(page.getByText("城镇中枢")).toBeVisible();

  // Full-page capture of the closed loop
  await page.screenshot({
    path: "test-results/uir011-sanitarium-loop-complete.png",
    fullPage: false,
  });
});
