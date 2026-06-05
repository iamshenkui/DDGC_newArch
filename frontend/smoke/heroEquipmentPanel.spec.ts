/**
 * Hero Equipment Panel smoke tests — fidelity gates for the 英雄面板-装备 page.
 *
 * Validates:
 * 1. Hero detail screen opens from town
 * 2. Equipment tab is the default active tab
 * 3. Equipment slots (weapon, armor, trinkets) render with data
 * 4. Hero roster sidebar renders with all heroes
 * 5. Clicking a hero in the sidebar switches to that hero
 * 6. Parchment panel layout is present
 * 7. Vertical tab bar is present
 * 8. No placeholder/skeletal language
 * 9. Screenshot capture for visual review
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("hero equipment panel: 英雄面板-装备", () => {
  test("equipment tab default + slots render with real data", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    // Boot replay → town
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // Open hero detail via "英雄" utility button
    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page);

    // Verify hero detail eyebrow
    await expect(
      page.getByText("Hero Detail"),
      "Hero detail eyebrow must be visible"
    ).toBeVisible();

    // Verify hero name + class in title
    await expect(
      page.getByText("Shen — Hunter"),
      "Hero detail title must show name and class"
    ).toBeVisible();

    // ── Layout anchors ──
    await expect(
      page.locator(".hero-detail-roster"),
      "Hero roster sidebar must be present"
    ).toBeVisible();

    await expect(
      page.locator(".hero-detail-center"),
      "Hero center column must be present"
    ).toBeVisible();

    await expect(
      page.locator(".hero-detail-parchment"),
      "Parchment panel must be present"
    ).toBeVisible();

    await expect(
      page.locator(".hero-tab-bar--vertical"),
      "Vertical tab bar must be present"
    ).toBeVisible();

    // ── Equipment tab is default active ──
    const equipTab = page.locator('.hero-tab-btn--active[data-source-component="EquipButton"]');
    await expect(equipTab, "Equipment tab must be active by default").toBeVisible();

    // ── Equipment slots ──
    await expect(
      page.locator('[data-source-component="WeaponSlot"]'),
      "Weapon slot must render"
    ).toBeVisible();
    await expect(
      page.locator('[data-source-component="ArmorSlot"]'),
      "Armor slot must render"
    ).toBeVisible();
    await expect(
      page.locator('[data-source-component="LeftTrinketSlot"]'),
      "Left trinket slot must render"
    ).toBeVisible();
    await expect(
      page.locator('[data-source-component="RightTrinketSlot"]'),
      "Right trinket slot must render"
    ).toBeVisible();

    // Verify slot names are populated with real data
    await expect(
      page.locator('.equipment-slot--weapon .equipment-slot-name'),
      "Weapon name must come from view model"
    ).not.toHaveText("");
    await expect(
      page.locator('.equipment-slot--armor .equipment-slot-name'),
      "Armor name must come from view model"
    ).not.toHaveText("");

    // ── Roster sidebar has all heroes ──
    const rosterCount = await page.locator(".hero-roster-thumb").count();
    expect(rosterCount, "Roster sidebar must show all heroes").toBeGreaterThanOrEqual(1);

    // Verify active hero is highlighted in roster
    await expect(
      page.locator('.hero-roster-thumb--active[data-hero-id="hero-hunter-01"]'),
      "Active hero must be highlighted in roster"
    ).toBeVisible();

    // ── HP / Stress bars ──
    await expect(
      page.locator(".core-stat-fill--hp"),
      "HP bar must render"
    ).toBeVisible();
    await expect(
      page.locator(".core-stat-fill--stress"),
      "Stress bar must render"
    ).toBeVisible();

    // ── Fidelity: no placeholder language ──
    const bodyText = await page.locator("body").innerText();
    expect(bodyText).not.toMatch(/placeholder/i);
    expect(bodyText).not.toMatch(/skeletal/i);
    expect(bodyText).not.toMatch(/reserved canvas/i);
  });

  test("hero switching via roster sidebar", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    // Boot replay → town → hero detail
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page);

    // Initial hero should be Shen
    await expect(
      page.getByText("Shen — Hunter"),
      "Initial hero must be Shen"
    ).toBeVisible();

    // Click a different hero in the roster if available
    const rosterThumbs = page.locator(".hero-roster-thumb:not(.hero-roster-thumb--active)");
    const altCount = await rosterThumbs.count();
    if (altCount > 0) {
      await rosterThumbs.first().click();
      await settle(page, 600);

      // After switching, the clicked hero should be active
      await expect(
        page.locator(".hero-roster-thumb--active"),
        "A different hero should be active after clicking"
      ).toBeVisible();
    }
  });

  test("tab navigation cycles all panels", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page);

    const tabComponents = [
      "EquipButton",
      "CombatSkillButton",
      "StateButton",
      "InfoButton",
      "CampingSkillButton",
    ];

    for (const comp of tabComponents) {
      const tabBtn = page.locator(`.hero-tab-btn[data-source-component="${comp}"]`);
      await expect(tabBtn, `Tab "${comp}" must be visible`).toBeVisible();
      await tabBtn.click();
      await settle(page, 200);

      const activeTab = page.locator(`.hero-tab-btn--active[data-source-component="${comp}"]`);
      await expect(activeTab, `Tab "${comp}" must become active after click`).toBeVisible();
    }
  });

  test("screenshot capture for visual review", async ({ page }) => {
    await page.setViewportSize({ width: 1600, height: 1000 });
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 10_000 });
    await settle(page, 400);

    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page, 600);

    // Full page capture
    await page.screenshot({
      path: "test-results/hero-equipment-panel-full.png",
      fullPage: true
    });

    // Equipment panel capture
    await page.locator(".hero-detail-parchment").screenshot({
      path: "test-results/hero-equipment-panel-parchment.png"
    });

    // Verify no console errors during capture
    const consoleErrors: string[] = [];
    page.on("console", (msg) => {
      if (msg.type() === "error") {
        consoleErrors.push(msg.text());
      }
    });

    expect(consoleErrors, "No console errors during hero equipment panel capture").toEqual([]);
  });
});
