/**
 * Sanitarium building screen capture and verification.
 *
 * Covers the migrated 公会界面-细胞修复站-选择1 page:
 *   - Building open from town shell
 *   - Tab navigation (使用设施 / 升级设施)
 *   - Treatment selector (异源细胞工坊 / 心魇斋)
 *   - Quirk treatment backdrop with slot grid
 *   - Disease treatment backdrop with slot grid
 *   - Cost display and activity button
 *   - Return to town flow
 *
 * Run with: npx playwright test smoke-cap/sanitariumCapture.spec.ts
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("sanitarium screen: open, verify structure, capture screenshots", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(600);

  // ── Open sanitarium from town ──
  await page.locator('[data-building-id="sanitarium"]').click();
  await page.waitForTimeout(800);

  // Verify building header
  await expect(
    page.locator(".building-detail-name"),
    "Sanitarium building name must be visible (细胞修复站)"
  ).toHaveText("细胞修复站");

  // ── Verify tab bar ──
  const useTab = page.locator('.sanitarium-tab[data-source-component="UseButton"]');
  const upgradeTab = page.locator('.sanitarium-tab[data-source-component="UpgradeButton"]');
  await expect(useTab, "Use tab (使用设施) must be visible").toBeVisible();
  await expect(upgradeTab, "Upgrade tab (升级设施) must be visible").toBeVisible();

  // Use tab should be active by default
  await expect(useTab, "Use tab should be active by default").toHaveClass(/sanitarium-tab--active/);

  // ── Verify treatment selector ──
  const diseaseCard = page.locator('.sanitarium-treatment-card[data-treatment="disease"]');
  const quirkCard = page.locator('.sanitarium-treatment-card[data-treatment="quirk"]');
  await expect(diseaseCard, "Disease treatment card must be visible").toBeVisible();
  await expect(quirkCard, "Quirk treatment card must be visible").toBeVisible();

  await expect(
    diseaseCard.locator(".sanitarium-treatment-label"),
    "Disease card label must be 异源细胞工坊"
  ).toHaveText("异源细胞工坊");
  await expect(
    quirkCard.locator(".sanitarium-treatment-label"),
    "Quirk card label must be 心魇斋"
  ).toHaveText("心魇斋");

  // ── Quirk backdrop (Selection 1) ──
  // Quirk should be selected by default
  await expect(quirkCard, "Quirk card should be active by default").toHaveClass(/sanitarium-treatment-card--active/);

  const quirkBackdrop = page.locator('.sanitarium-quirk-backdrop[data-source-component="SanitariumQuirkWindow"]');
  await expect(quirkBackdrop, "Quirk treatment backdrop must be visible").toBeVisible();

  // Verify slot grids
  const negativeSlots = page.locator('.sanitarium-quirk-slot[data-slot-type="negative"]');
  const positiveSlots = page.locator('.sanitarium-quirk-slot[data-slot-type="positive"]');
  await expect(negativeSlots, "Must have 5 negative quirk slots").toHaveCount(5);
  await expect(positiveSlots, "Must have 5 positive quirk slots").toHaveCount(5);

  // Verify cost and activity button
  await expect(
    page.locator('.sanitarium-quirk-backdrop .sanitarium-activity-btn'),
    "Quirk start-treatment button must be visible"
  ).toHaveText("开始治疗");

  // ── Screenshot: Quirk selection (选择1) ──
  await page.screenshot({
    path: "test-results/sanitarium-quirk-selection.png",
    fullPage: true
  });

  // ── Switch to Disease ──
  await diseaseCard.click();
  await page.waitForTimeout(300);

  const diseaseBackdrop = page.locator('.sanitarium-disease-backdrop[data-source-component="SanitariumDiseaseWindow"]');
  await expect(diseaseBackdrop, "Disease treatment backdrop must be visible after click").toBeVisible();

  const diseaseSlots = page.locator('.sanitarium-disease-slot[data-slot-type="disease"]');
  await expect(diseaseSlots, "Must have 3 disease slots").toHaveCount(3);

  await expect(
    page.locator('.sanitarium-disease-backdrop .sanitarium-activity-btn'),
    "Disease start-treatment button must be visible"
  ).toHaveText("开始治疗");

  // ── Screenshot: Disease selection ──
  await page.screenshot({
    path: "test-results/sanitarium-disease-selection.png",
    fullPage: true
  });

  // ── Switch to Upgrade tab ──
  await upgradeTab.click();
  await page.waitForTimeout(300);

  await expect(upgradeTab, "Upgrade tab should be active after click").toHaveClass(/sanitarium-tab--active/);

  const upgradeTrees = page.locator(".sanitarium-upgrade-tree");
  await expect(upgradeTrees, "Must have 3 upgrade trees").toHaveCount(3);

  // ── Screenshot: Upgrade tab ──
  await page.screenshot({
    path: "test-results/sanitarium-upgrade.png",
    fullPage: true
  });

  // ── Return to town ──
  await page.locator('.sanitarium-return-btn[data-source-component="CloseButton"]').click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await expect(
    page.locator(".town-viewport"),
    "Must return to town after clicking leave"
  ).toBeVisible();
});
