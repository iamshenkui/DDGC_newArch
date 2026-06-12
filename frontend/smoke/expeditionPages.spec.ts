import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test.describe("expedition pages: 位面探索 + 位面探索-战前补给", () => {
  test("reach both pages from town and capture screenshots", async ({ page }) => {
    await page.setViewportSize({ width: 1600, height: 1000 });
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    // Boot replay and wait for town
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 10_000 });
    await page.waitForTimeout(400);

    // Town → 位面探索
    await page.locator(".estate-embark-button").click();
    await page.waitForSelector('[data-testid="expedition-planning-screen"]', {
      timeout: 5_000,
    });
    await page.waitForTimeout(400);

    await expect(
      page.getByRole("heading", { name: "位面探索" }),
      "位面探索 title must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="expedition-planning-details"]'),
      "Plane details panel must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="expedition-party-panel"]'),
      "Party panel must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="planning-btn-proceed"]'),
      "Proceed to provisioning button must be visible"
    ).toBeVisible();

    await page.screenshot({
      path: "test-results/expedition-planning-page.png",
      fullPage: false,
    });

    // 位面探索 → 位面探索-战前补给
    await page.locator('[data-testid="planning-btn-proceed"]').click();
    await page.waitForSelector('[data-testid="provisioning-screen"]', {
      timeout: 5_000,
    });
    await page.waitForTimeout(400);

    await expect(
      page.getByRole("heading", { name: "战前补给" }),
      "战前补给 title must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="provisioning-left-panel"]'),
      "Provisioning left panel (hero focus) must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="provisioning-right-panel"]'),
      "Provisioning right panel (supply grid) must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="supply-grid"]'),
      "Supply grid must be visible"
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="btn-start-adventure"]'),
      "Start adventure button must be visible"
    ).toBeVisible();

    await page.screenshot({
      path: "test-results/expedition-provisioning-page.png",
      fullPage: false,
    });

    // Return path: provisioning → town
    await page.locator('[data-testid="footer-btn-return"]').click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await expect(
      page.getByText("城镇中枢"),
      "Must return to town from provisioning"
    ).toBeVisible();
  });
});
