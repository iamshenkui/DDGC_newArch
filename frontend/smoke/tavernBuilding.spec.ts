/**
 * Tavern building screen smoke test — fidelity gates for the 迷情乐园 (Tavern) building.
 *
 * Run with: npx playwright test smoke/tavernBuilding.spec.ts
 *
 * Validates:
 * 1. Town shell renders with tavern building clickable
 * 2. Tavern building screen opens with correct title and layout
 * 3. Tab navigation (升级设施 / 使用设施) works
 * 4. Upgrade slots, action cards, and host portrait render
 * 5. Return navigation works
 * 6. No console/page errors
 */

import { test, expect, type Page } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

function setupErrorCollectors(page: Page): {
  consoleErrors: string[];
  pageErrors: string[];
} {
  const consoleErrors: string[] = [];
  const pageErrors: string[] = [];

  page.on("console", (msg) => {
    if (msg.type() === "error") {
      consoleErrors.push(`[console.error] ${msg.text()}`);
    }
  });
  page.on("pageerror", (err) => {
    pageErrors.push(`[page.error] ${err.message}`);
  });

  return { consoleErrors, pageErrors };
}

function expectNoErrors(
  pageErrors: string[],
  consoleErrors: string[],
  phase: string
): void {
  expect(
    pageErrors,
    `${phase}: page errors must be empty`
  ).toEqual([]);
  expect(
    consoleErrors,
    `${phase}: console errors must be empty`
  ).toEqual([]);
}

async function settle(page: Page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("tavern building screen smoke", () => {
  test("replay boot → open tavern → tab navigation → return", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    // ── Phase 1: Navigate and boot replay ──────────────────
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // ── Phase 2: Open tavern building ──────────────────────
    await page.locator('[data-building-id="tavern"]').click();
    await settle(page, 800);

    // Building header assertions
    await expect(
      page.locator(".building-detail-eyebrow"),
      "Tavern building eyebrow must be visible"
    ).toHaveText("Building");

    await expect(
      page.locator(".building-detail-name"),
      "Tavern building name must be 迷情乐园"
    ).toHaveText("迷情乐园");

    // Host portrait and action buttons
    await expect(
      page.locator(".tavern-host-portrait"),
      "Tavern host portrait must be visible"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "对话" }),
      "Talk button must be visible"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "离开" }),
      "Leave button must be visible"
    ).toBeVisible();

    // Tab bar assertions
    await expect(
      page.locator('.tavern-tab-btn[data-tab-id="upgrade"]'),
      "Upgrade tab must be visible"
    ).toBeVisible();

    await expect(
      page.locator('.tavern-tab-btn[data-tab-id="use"]'),
      "Use tab must be visible"
    ).toBeVisible();

    // Default active tab is upgrade
    await expect(
      page.locator('.tavern-tab-btn[data-tab-id="upgrade"]'),
      "Upgrade tab must be active by default"
    ).toHaveClass(/tavern-tab-btn--active/);

    // Upgrade panel content
    await expect(
      page.locator(".tavern-upgrade-panel"),
      "Upgrade panel must be visible"
    ).toBeVisible();

    await expect(
      page.locator(".tavern-upgrade-slot"),
      "Upgrade slots must render (4 slots)"
    ).toHaveCount(4);

    await expect(
      page.locator(".tavern-upgrade-level-label"),
      "Tavern level label must be visible"
    ).toHaveText("酒馆等级");

    // ── Phase 3: Switch to use tab ─────────────────────────
    await page.locator('.tavern-tab-btn[data-tab-id="use"]').click();
    await settle(page, 200);

    await expect(
      page.locator('.tavern-tab-btn[data-tab-id="use"]'),
      "Use tab must be active after click"
    ).toHaveClass(/tavern-tab-btn--active/);

    await expect(
      page.locator(".tavern-use-panel"),
      "Use panel must be visible after tab switch"
    ).toBeVisible();

    // ── Phase 4: Return to town ────────────────────────────
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.locator(".town-viewport"),
      "Must be back at town after returning from tavern"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Tavern building screen smoke");
  });

  test("live boot → open tavern → verify screen renders", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Live" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator('[data-building-id="tavern"]').click();
    await settle(page, 800);

    await expect(
      page.locator(".building-detail-name"),
      "Live tavern building name must be 迷情乐园"
    ).toHaveText("迷情乐园");

    await expect(
      page.locator(".tavern-tab-bar"),
      "Live tavern tab bar must render"
    ).toBeVisible();

    await expect(
      page.locator(".tavern-upgrade-panel"),
      "Live tavern upgrade panel must render"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Live tavern building screen");
  });
});
