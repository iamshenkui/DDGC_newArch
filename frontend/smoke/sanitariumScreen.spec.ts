/**
 * Sanitarium building screen smoke tests — fidelity gates for
 * 公会界面-细胞修复站-使用空 (Guild Interface - Cell Repair Station - Empty Use).
 *
 * Validates:
 * 1. Town → open sanitarium building → correct screen renders
 * 2. Facility mode tabs (升级设施 / 使用设施) are visible and toggleable
 * 3. Treatment slots (心理疾病, 异星细胞工坊) render with empty state
 * 4. NPC portrait area + 对话 button present
 * 5. Resource strip at bottom
 * 6. Return to Town navigates back
 * 7. No console/page errors
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
  expect(pageErrors, `${phase}: page errors must be empty`).toEqual([]);
  expect(consoleErrors, `${phase}: console errors must be empty`).toEqual([]);
}

async function settle(page: Page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("sanitarium building screen: 细胞修复站-使用空", () => {
  test("town → sanitarium → treatment slots → return", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    // ── Phase 1: Boot replay → town ────────────────────────
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // ── Phase 2: Open sanitarium building ──────────────────
    await page.locator('[data-building-id="sanitarium"]').click();
    await settle(page, 800);

    // Building header
    await expect(
      page.locator(".building-detail-name"),
      "Sanitarium building name must be visible (细胞修复站)"
    ).toHaveText("细胞修复站");

    await expect(
      page.locator(".building-detail-eyebrow"),
      "Building eyebrow must be visible"
    ).toHaveText("Building");

    // NPC portrait area
    await expect(
      page.locator(".sanitarium-npc-area"),
      "NPC portrait area must render"
    ).toBeVisible();

    await expect(
      page.locator(".sanitarium-npc-portrait-frame"),
      "NPC portrait frame must render"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "对话" }),
      "Talk button must be visible"
    ).toBeVisible();

    // Facility mode tabs
    await expect(
      page.locator(".sanitarium-mode-tabs"),
      "Facility mode tabs must render"
    ).toBeVisible();

    await expect(
      page.locator(".sanitarium-mode-label").filter({ hasText: "升级设施" }),
      "Upgrade facility tab must be visible"
    ).toBeVisible();

    await expect(
      page.locator(".sanitarium-mode-label").filter({ hasText: "使用设施" }),
      "Use facility tab must be visible"
    ).toBeVisible();

    // Default mode should be "use"
    await expect(
      page.locator('.sanitarium-mode-tab input[type="checkbox"]').nth(1),
      "Use facility checkbox should be checked by default"
    ).toBeChecked();

    // Treatment slots
    await expect(
      page.locator(".sanitarium-treatment-slot"),
      "Treatment slots must render (2 slots from replay fixture)"
    ).toHaveCount(2);

    // Slot 1: 心理疾病
    await expect(
      page.locator(".sanitarium-slot-label").filter({ hasText: "心理疾病" }),
      "Mental disease treatment slot label must be visible"
    ).toBeVisible();

    // Slot 2: 异星细胞工坊
    await expect(
      page
        .locator(".sanitarium-slot-label")
        .filter({ hasText: "异星细胞工坊" }),
      "Alien cell workshop treatment slot label must be visible"
    ).toBeVisible();

    // Empty slot states
    await expect(
      page.locator(".sanitarium-slot-hero-name--empty").filter({ hasText: "空置" }),
      "Empty slot placeholder (空置) must be visible"
    ).toHaveCount(2);

    // "选择英雄" buttons for empty slots
    await expect(
      page.getByRole("button", { name: "选择英雄" }),
      "Select hero buttons must be present for empty slots"
    ).toHaveCount(2);

    // Resource strip
    await expect(
      page.locator(".sanitarium-resource-strip"),
      "Resource strip must render at bottom"
    ).toBeVisible();

    await expect(
      page.locator(".sanitarium-resource-slot--gold .sanitarium-resource-value"),
      "Gold amount must be visible in resource strip"
    ).toHaveText("5895");

    // ── Phase 3: Toggle to upgrade mode ────────────────────
    await page
      .locator(".sanitarium-mode-tab")
      .filter({ hasText: "升级设施" })
      .click();
    await settle(page, 200);

    await expect(
      page.locator('.sanitarium-mode-tab input[type="checkbox"]').first(),
      "Upgrade facility checkbox should be checked after click"
    ).toBeChecked();

    // Upgrade section should show
    await expect(
      page.locator(".sanitarium-upgrade-section"),
      "Upgrade section must render after tab switch"
    ).toBeVisible();

    // ── Phase 4: Return to town ────────────────────────────
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("城镇中枢"),
      "Must be back at town after returning from sanitarium"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Sanitarium screen flow");
  });

  test("sanitarium screenshot capture", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.locator('[data-building-id="sanitarium"]').click();
    await settle(page, 800);

    await expect(
      page.locator(".sanitarium-treatment-slot"),
      "Treatment slots must be visible for screenshot"
    ).toHaveCount(2);

    await page.screenshot({
      path: "/tmp/sanitarium-screen-current.png",
      fullPage: false,
    });
  });
});
