/**
 * Expedition planning (位面探索) screenshot capture test.
 *
 * Validates the plane exploration screen against the reference image:
 *   reference/ref_image/跨际元契约/3位面探索/位面探索.png
 *
 * Coverage:
 * - Plane selection strip with selectable and locked planes
 * - Selected plane details panel
 * - Interactive party assignment slots
 * - Available hero roster strip
 * - Bottom controls (返回城镇, 前往战前补给)
 * - Stable data-testid selectors for all interactive regions
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page: any, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("expedition planning screen: 位面探索 fidelity", () => {
  test("replay boot → town → expedition-planning layout and copy", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // Town embark button enters expedition planning
    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    // ── Layout anchors ─────────────────────────────────────
    await expect(
      page.locator('[data-testid="expedition-planning-screen"]'),
      "Expedition planning screen root must mount"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-selection-strip"]'),
      "Plane selection strip must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-details-panel"]'),
      "Plane details panel must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-panel"]'),
      "Party panel must be visible"
    ).toBeVisible();

    // ── Copy anchors (Chinese text matching reference) ─────
    await expect(
      page.getByText("位面探索"),
      "Title 位面探索 must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("苍灯远征"),
      "Eyebrow campaign name must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("可选位面"),
      "Plane strip title must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("位面详情"),
      "Plane details title must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("远征队伍"),
      "Party panel title must be visible"
    ).toBeVisible();

    // ── Plane cards ─────────────────────────────────────────
    await expect(
      page.locator('[data-testid="plane-card-qinglong"]'),
      "QingLong plane card must render"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-card-baihu"]'),
      "BaiHu plane card must render"
    ).toBeVisible();

    // Locked planes should be present but disabled
    await expect(
      page.locator('[data-testid="plane-card-zhuque"]'),
      "ZhuQue plane card must render"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-card-zhuque"]'),
      "ZhuQue plane card must be disabled"
    ).toBeDisabled();

    // ── Party slots ────────────────────────────────────────
    await expect(
      page.locator('[data-testid="party-slot-hero-hero-hunter-01"]'),
      "Shen must be assigned to a party slot"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-hero-hero-white-01"]'),
      "Bai Xiu must be assigned to a party slot"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-empty-2"]'),
      "Empty slot index 2 must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-empty-3"]'),
      "Empty slot index 3 must be visible"
    ).toBeVisible();

    // ── Roster anchors ─────────────────────────────────────
    await expect(
      page.locator('[data-testid="expedition-roster-strip"]'),
      "Available hero roster strip must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="roster-hero-hero-black-01"]'),
      "Hei Zhen must appear in available roster"
    ).toBeVisible();

    // ── Bottom controls ────────────────────────────────────
    await expect(
      page.locator('[data-testid="expedition-btn-return"]'),
      "Return to Town button must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="expedition-btn-proceed"]'),
      "Proceed to Provisioning button must be visible and enabled"
    ).toBeEnabled();

    // ── Screenshot evidence ────────────────────────────────
    await page.screenshot({
      path: "test-results/expedition-planning-screen.png",
      fullPage: false,
    });
  });

  test("plane selection updates details panel", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    // Default selection is qinglong
    await expect(
      page.locator('[data-testid="plane-card-qinglong"]'),
      "QingLong should be selected by default"
    ).toHaveClass(/plane-card--selected/);

    // Switch to baihu
    await page.locator('[data-testid="plane-card-baihu"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="plane-card-baihu"]'),
      "BaiHu should be selected after click"
    ).toHaveClass(/plane-card--selected/);

    await expect(
      page.getByText("白虎"),
      "BaiHu details must be visible"
    ).toBeVisible();
  });

  test("hero assignment toggle in expedition planning", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    // Hei Zhen is in the roster initially
    await expect(
      page.locator('[data-testid="roster-hero-hero-black-01"]'),
      "Hei Zhen must be in roster"
    ).toBeVisible();

    // Add Hei Zhen to party
    await page.locator('[data-testid="roster-hero-hero-black-01"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="party-slot-hero-hero-black-01"]'),
      "Hei Zhen must appear in party slot after selection"
    ).toBeVisible();

    // Remove Hei Zhen from party by clicking the slot
    await page.locator('[data-testid="party-slot-hero-hero-black-01"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="roster-hero-hero-black-01"]'),
      "Hei Zhen must return to roster after removal"
    ).toBeVisible();
  });

  test("expedition-planning → return-to-town navigation", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-testid="expedition-btn-return"]').click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.locator(".town-viewport"),
      "Must return to town from expedition planning"
    ).toBeVisible();

    await expect(
      page.getByText("城镇中枢"),
      "Town eyebrow must be visible after return"
    ).toBeVisible();
  });

  test("expedition-planning → provisioning navigation", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    await page.getByRole("button", { name: "前往战前补给" }).click();
    await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
    await settle(page);

    await expect(
      page.locator('[data-testid="provisioning-screen"]'),
      "Must reach provisioning from expedition planning"
    ).toBeVisible();

    await expect(
      page.getByText("战前补给"),
      "Provisioning title must be visible"
    ).toBeVisible();
  });
});
