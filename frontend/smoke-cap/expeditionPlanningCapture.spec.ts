/**
 * Expedition planning (位面探索) screenshot capture test.
 *
 * Validates the plane exploration screen against the reference image:
 *   reference/ref_image/跨际元契约/3位面探索/位面探索.png
 *
 * Coverage:
 * - Top HUD with Chinese copy and real view-model data
 * - Plane selection strip with locked/unlocked states
 * - Two-panel layout (plane details + party assignment)
 * - Interactive party slots fed by real view-model data
 * - Available-heroes roster for add/remove interactions
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

    // Town → Expedition Planning
    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    // ── Layout anchors ─────────────────────────────────────
    await expect(
      page.locator('[data-testid="expedition-planning-screen"]'),
      "Expedition planning screen root must mount"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-details-panel"]'),
      "Plane details panel must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-assignment-panel"]'),
      "Party assignment panel must be visible"
    ).toBeVisible();

    // ── Copy anchors (Chinese text matching reference) ─────
    await expect(
      page.getByText("位面探索"),
      "Title 位面探索 must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("苍灯远征"),
      "Eyebrow (campaign name) must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("可选位面"),
      "Plane strip title 可选位面 must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("远征队伍"),
      "Party panel title 远征队伍 must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="planning-btn-return"]'),
      "Return to Town button (返回城镇) must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="planning-btn-proceed"]'),
      "Proceed to Provisioning button (战前补给) must be visible and enabled"
    ).toBeEnabled();

    // ── Plane cards ────────────────────────────────────────
    await expect(
      page.locator('[data-testid="plane-card-qinglong"]'),
      "QingLong plane card must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-card-baihu"]'),
      "BaiHu plane card must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="plane-card-zhuque"]'),
      "ZhuQue plane card must be visible and locked"
    ).toHaveAttribute("data-locked", "true");

    // ── Party slots ────────────────────────────────────────
    await expect(
      page.locator('[data-testid="party-slot-hero-hunter-01"]'),
      "Shen must be assigned to a party slot"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-hero-white-01"]'),
      "Bai Xiu must be assigned to a party slot"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="party-slot-empty-2"]'),
      "Empty party slot must be visible"
    ).toBeVisible();

    // ── Roster anchors ─────────────────────────────────────
    await expect(
      page.locator('[data-testid="planning-roster-hero-hero-black-01"]'),
      "Unselected hero Hei Zhen must appear in roster"
    ).toBeVisible();

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

    // Default selected plane is QingLong
    await expect(
      page.locator('[data-testid="plane-details-panel"]'),
      "QingLong details must be shown by default"
    ).toContainText("青龙");

    // Switch to BaiHu
    await page.locator('[data-testid="plane-card-baihu"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="plane-details-panel"]'),
      "BaiHu details must be shown after selection"
    ).toContainText("白虎");

    await expect(
      page.locator('[data-testid="plane-card-baihu"]'),
      "BaiHu card must be selected"
    ).toHaveClass(/plane-card--selected/);
  });

  test("party assignment add/remove interactions", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', { timeout: 5_000 });
    await settle(page);

    // Remove Shen from party
    await page.locator('[data-testid="party-slot-remove-hero-hunter-01"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="party-slot-hero-hunter-01"]'),
      "Shen must be removed from party slots"
    ).toHaveCount(0);

    await expect(
      page.locator('[data-testid="planning-roster-hero-hero-hunter-01"]'),
      "Shen must return to available roster"
    ).toBeVisible();

    // Add Hei Zhen from roster
    await page.locator('[data-testid="planning-roster-hero-hero-black-01"]').click();
    await settle(page, 300);

    await expect(
      page.locator('[data-testid="party-slot-hero-black-01"]'),
      "Hei Zhen must be assigned to a party slot"
    ).toBeVisible();
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

    await page.locator('[data-testid="planning-btn-return"]').click();
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
