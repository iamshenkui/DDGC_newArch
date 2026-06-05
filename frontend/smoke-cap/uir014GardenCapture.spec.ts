import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * UIR-014: Garden (天国花园) building screen screenshot captures.
 *
 * Navigates to the garden building from town and captures the
 * "使用设施" (Use Facilities) tab to verify visual fidelity against
 * the reference image: 公会界面-天国花园-使用中.
 */
test("UIR-014: capture garden building screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay and wait for town
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Verify town is visible with garden building
  await expect(page.getByText("天国花园")).toBeVisible();

  // Click on the garden building
  await page.locator('[data-building-id="garden"]').click();
  await page.waitForTimeout(800);

  // Verify garden building screen elements
  await expect(
    page.locator(".building-detail-name"),
    "Garden building name must be visible"
  ).toHaveText("天国花园");

  // Verify tab bar with 升级设施 and 使用设施 tabs
  await expect(
    page.locator('.garden-tab-btn[data-tab="upgrade"]'),
    "Upgrade tab must be visible"
  ).toBeVisible();
  await expect(
    page.locator('.garden-tab-btn[data-tab="use"]'),
    "Use tab must be visible"
  ).toBeVisible();

  // Click the "使用设施" tab to ensure it's active
  await page.locator('.garden-tab-btn[data-tab="use"]').click();
  await page.waitForTimeout(300);

  // Verify facility sections are present (rendered as building-action-section with titles)
  await expect(
    page.locator('.building-action-section-title').filter({ hasText: "星空观测台" }),
    "Stargazing Observatory facility must be visible"
  ).toBeVisible();
  await expect(
    page.locator('.building-action-section-title').filter({ hasText: "回忆长廊" }),
    "Memory Corridor facility must be visible"
  ).toBeVisible();
  await expect(
    page.locator('.building-action-section-title').filter({ hasText: "美梦舱室" }),
    "Dream Chamber facility must be visible"
  ).toBeVisible();

  // Verify facility names and descriptions
  await expect(page.getByText("远离城市喧嚣，寻找心灵的平静。")).toBeVisible();
  await expect(page.getByText("回顾曾经的苦痛与记忆。")).toBeVisible();
  await expect(page.getByText("编织美梦。")).toBeVisible();

  // Verify hero slots are rendered (3 per facility = 9 total)
  await expect(
    page.locator(".garden-hero-slot"),
    "Garden must render hero slots for facilities"
  ).toHaveCount(9);

  // Verify empty slot placeholders
  await expect(
    page.locator('.garden-hero-slot--empty').first(),
    "Empty hero slots must be visible"
  ).toBeVisible();

  // Verify action buttons are present (3 primary action buttons for the 3 facilities)
  await expect(
    page.locator('.building-action-btn--primary'),
    "Facility action buttons must be visible"
  ).toHaveCount(3);

  // Capture garden building screen
  await page.locator(".app-frame").screenshot({
    path: "test-results/uir014-garden-building.png",
  });

  // Verify the "使用设施" tab is active (has active class)
  await expect(
    page.locator('.garden-tab-btn--active[data-tab="use"]'),
    "Use tab must be active"
  ).toBeVisible();

  // Return to town
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Verify back in town
  await expect(page.getByText("城镇中枢")).toBeVisible();
});

test("UIR-014: garden upgrade tab renders correctly", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  await page.locator('[data-building-id="garden"]').click();
  await page.waitForTimeout(800);

  // Click the upgrade tab
  await page.locator('.garden-tab-btn[data-tab="upgrade"]').click();
  await page.waitForTimeout(300);

  // Verify upgrade tab is active
  await expect(
    page.locator('.garden-tab-btn--active[data-tab="upgrade"]'),
    "Upgrade tab must be active after click"
  ).toBeVisible();

  // Capture upgrade tab
  await page.locator(".app-frame").screenshot({
    path: "test-results/uir014-garden-upgrade-tab.png",
  });

  // Return to town
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
});
