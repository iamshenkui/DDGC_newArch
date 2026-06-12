/**
 * Expedition planning (位面探索) screenshot capture test.
 *
 * Validates the plane exploration screen against the reference image:
 *   reference/ref_image/跨际元契约/3位面探索/位面探索.png
 *
 * Coverage:
 * - Plane selection strip with selectable and locked plane cards
 * - Selected plane details panel
 * - Party assignment panel with real hero vitals
 * - Action buttons (Return to Town, Proceed to Provisioning)
 * - Stable data-testid selectors for all interactive regions
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page: any, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("expedition planning screen: 位面探索 fidelity", () => {
  test("replay boot → town → expedition planning layout and copy", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // Click embark to enter expedition planning
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
      page.locator('[data-testid="expedition-planning-details"]'),
      "Plane details panel must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="expedition-party-panel"]'),
      "Party panel must be visible"
    ).toBeVisible();

    // ── Copy anchors (Chinese text matching reference) ─────
    await expect(
      page.getByText("位面探索"),
      "Eyebrow 位面探索 must be visible"
    ).toBeVisible();

    await expect(
      page.getByRole("heading", { name: "Plane Exploration" }),
      "Title Plane Exploration must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("The Azure Lantern"),
      "Campaign name must be visible"
    ).toBeVisible();

    // ── Plane card anchors ─────────────────────────────────
    for (const planeId of ["qinglong", "baihu", "zhuque", "xuanwu"]) {
      await expect(
        page.locator(`[data-testid="plane-card-${planeId}"]`),
        `Plane card ${planeId} must render`
      ).toBeVisible();
    }

    // Selected plane details should reflect default selection
    await expect(
      page.locator('[data-testid="expedition-planning-details"]'),
      "Selected plane details must contain QingLong description"
    ).toContainText("Azure Dragon plane");

    // ── Party panel anchors ────────────────────────────────
    await expect(
      page.locator('[data-testid="party-slot-hero-hero-hunter-01"]'),
      "Selected hero Shen must appear in party panel"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-hero-hero-white-01"]'),
      "Selected hero Bai Xiu must appear in party panel"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-empty-2"]'),
      "Empty slot 2 must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-empty-3"]'),
      "Empty slot 3 must be visible"
    ).toBeVisible();

    // ── Action button anchors ──────────────────────────────
    await expect(
      page.locator('[data-testid="expedition-planning-btn-return"]'),
      "Return to Town button must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="expedition-planning-btn-proceed"]'),
      "Proceed to Provisioning button must be visible and enabled"
    ).toBeEnabled();

    // ── Screenshot evidence ────────────────────────────────
    await page.screenshot({
      path: "test-results/expedition-planning-screen.png",
      fullPage: false,
    });
  });

  test("expedition planning → provisioning navigation", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-testid="expedition-planning-btn-proceed"]').click();
    await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
    await settle(page);

    await expect(
      page.locator('[data-testid="provisioning-screen"]'),
      "Must navigate to provisioning from expedition planning"
    ).toBeVisible();

    await expect(
      page.getByRole("heading", { name: "战前补给" }),
      "Provisioning title 战前补给 must be visible"
    ).toBeVisible();

    // Derived from expedition-planning choices
    await expect(
      page.getByText("青龙"),
      "Expedition label should carry over selected plane name"
    ).toBeVisible();
  });

  test("expedition planning plane selection updates details", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    // Select BaiHu plane
    await page.locator('[data-testid="plane-card-baihu"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="expedition-planning-details"]'),
      "Selected plane details must update to BaiHu"
    ).toContainText("White Tiger plane");

    // Locked plane should not update selection
    await page.locator('[data-testid="plane-card-zhuque"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="expedition-planning-details"]'),
      "Locked plane must not change selection"
    ).toContainText("White Tiger plane");
  });

  test("expedition planning → return-to-town navigation", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-testid="expedition-planning-btn-return"]').click();
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
});
