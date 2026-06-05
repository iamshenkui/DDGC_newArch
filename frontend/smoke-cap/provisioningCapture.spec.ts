/**
 * Provisioning (战前补给) screenshot capture test.
 *
 * Validates the pre-battle supply screen against the reference image:
 *   reference/ref_image/跨际元契约/3位面探索/位面探索-战前补给.png
 *
 * Coverage:
 * - Two-panel layout (left hero, right supply grid)
 * - Chinese copy matching reference
 * - Supply item grid visibility
 * - Action buttons (返回城镇, 开始冒险)
 * - Party status indicators
 * - Stable data-testid selectors for all interactive regions
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page: any, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("provisioning screen: 战前补给 fidelity", () => {
  test("replay boot → town → provisioning layout and copy", async ({ page }) => {
    // Boot replay and navigate to provisioning
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // Click embark to enter provisioning
    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
    await settle(page);

    // ── Layout anchors ─────────────────────────────────────
    await expect(
      page.locator('[data-testid="provisioning-screen"]'),
      "Provisioning screen root must mount"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="provisioning-left-panel"]'),
      "Left panel (hero focus) must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="provisioning-right-panel"]'),
      "Right panel (supply grid) must be visible"
    ).toBeVisible();

    // ── Copy anchors (Chinese text matching reference) ─────
    await expect(
      page.getByText("战前补给"),
      "Title 战前补给 must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("远征准备"),
      "Eyebrow 远征准备 must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("补给物资"),
      "Supply section title 补给物资 must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("做好出发前的准备，合理分配补给"),
      "Subtitle must match reference copy"
    ).toBeVisible();

    // ── Supply grid anchors ────────────────────────────────
    const supplyGrid = page.locator('[data-testid="supply-grid"]');
    await expect(supplyGrid, "Supply grid must be visible").toBeVisible();

    const supplyItems = supplyGrid.locator(".provisioning-supply-item");
    await expect(supplyItems, "Supply grid must contain placeholder items").toHaveCount(7);

    // Specific supply items
    for (const itemId of [
      "supply-food",
      "supply-torch",
      "supply-bandage",
      "supply-antidote",
      "supply-shovel",
      "supply-key",
      "supply-holy",
    ]) {
      await expect(
        page.locator(`[data-testid="supply-item-${itemId}"]`),
        `Supply item ${itemId} must render`
      ).toBeVisible();
    }

    // ── Hero focus anchors ─────────────────────────────────
    await expect(
      page.locator('[data-testid="focused-hero-portrait"]'),
      "Focused hero portrait must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="focused-hero-name"]'),
      "Focused hero name must be visible"
    ).toHaveText("Shen");

    // ── Action button anchors ──────────────────────────────
    await expect(
      page.locator('[data-testid="btn-return-town"]'),
      "Return to Town button (返回城镇) must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="btn-start-adventure"]'),
      "Start Adventure button (开始冒险) must be visible"
    ).toBeVisible();

    // ── Party strip anchors ────────────────────────────────
    await expect(
      page.locator('[data-testid="party-strip"]'),
      "Party strip must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-hero-hero-hunter-01"]'),
      "Selected hero Shen must appear in party strip"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-hero-hero-white-01"]'),
      "Selected hero Bai Xiu must appear in party strip"
    ).toBeVisible();

    // ── Roster anchors ─────────────────────────────────────
    await expect(
      page.locator('[data-testid="roster-hero-hero-black-01"]'),
      "Unselected hero Hei Zhen must appear in roster"
    ).toBeVisible();

    // ── Resource strip anchors ─────────────────────────────
    await expect(
      page.locator('[data-testid="resource-strip"]'),
      "Resource strip must be visible"
    ).toBeVisible();

    // ── Footer controls ────────────────────────────────────
    await expect(
      page.locator('[data-testid="party-status-pill"]'),
      "Party status pill must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="footer-btn-return"]'),
      "Footer return button must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="footer-btn-launch"]'),
      "Footer launch button must be visible and enabled"
    ).toBeEnabled();

    // ── Screenshot evidence ────────────────────────────────
    await page.screenshot({
      path: "test-results/provisioning-screen.png",
      fullPage: false,
    });
  });

  test("provisioning → return-to-town navigation", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
    await settle(page);

    // Click return to town from the left panel
    await page.locator('[data-testid="btn-return-town"]').click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.locator(".town-viewport"),
      "Must return to town from provisioning"
    ).toBeVisible();

    await expect(
      page.getByText("城镇中枢"),
      "Town eyebrow must be visible after return"
    ).toBeVisible();
  });

  test("provisioning hero selection toggle", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
    await settle(page);

    // Hei Zhen is unselected initially
    await expect(
      page.locator('[data-testid="roster-hero-hero-black-01"]'),
      "Hei Zhen must be in roster"
    ).toBeVisible();

    // Click to add to party
    await page.locator('[data-testid="roster-hero-hero-black-01"]').click();
    await settle(page, 300);

    // Should now appear in party strip
    await expect(
      page.locator('[data-testid="party-hero-hero-black-01"]'),
      "Hei Zhen must appear in party strip after selection"
    ).toBeVisible();

    // Click party hero to remove
    await page.locator('[data-testid="party-hero-hero-black-01"]').click();
    await settle(page, 300);

    // Should be back in roster
    await expect(
      page.locator('[data-testid="roster-hero-hero-black-01"]'),
      "Hei Zhen must return to roster after deselection"
    ).toBeVisible();
  });
});
