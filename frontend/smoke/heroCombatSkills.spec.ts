/**
 * Hero Combat Skills page smoke test
 *
 * Validates the 英雄面板-战斗技能 page against the reference image:
 * - Left sidebar with hero roster list
 * - Center portrait + hero info panel
 * - Vertical tabs on the right
 * - Combat skills panel with icon list + detail card
 * - Bottom resource bar
 * - Navigation arrows
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test.describe("hero combat skills page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await page.waitForTimeout(400);

    // Open hero detail
    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await page.waitForTimeout(300);
  });

  test("combat skills tab layout matches reference", async ({ page }) => {
    // Click combat skills tab (default, but be explicit)
    await page.locator('.hero-tab-btn[data-source-component="CombatSkillButton"]').click();
    await page.waitForTimeout(200);

    // ── Sidebar hero list ──
    await expect(
      page.locator(".hero-detail-sidebar"),
      "Hero roster sidebar must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hero-sidebar-item"),
      "Roster must have hero items"
    ).toHaveCount(3);
    await expect(
      page.locator(".hero-sidebar-item--active"),
      "Exactly one hero must be selected"
    ).toHaveCount(1);

    // ── Center column: portrait + info ──
    await expect(
      page.locator(".hero-portrait-frame--large"),
      "Large hero portrait frame must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hero-info-panel"),
      "Hero info panel must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hero-info-name"),
      "Hero name must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hero-talent-section"),
      "Talent section must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hero-exile-btn"),
      "Exile button must be visible"
    ).toBeVisible();

    // ── Navigation arrows ──
    await expect(
      page.locator(".hero-nav-arrow--prev"),
      "Prev hero arrow must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hero-nav-arrow--next"),
      "Next hero arrow must be visible"
    ).toBeVisible();

    // ── Vertical tabs ──
    await expect(
      page.locator(".hero-tab-bar--vertical"),
      "Vertical tab bar must be visible"
    ).toBeVisible();
    const tabLabels = ["人物信息", "人物状态", "托管技能", "战斗技能", "装备"];
    for (const label of tabLabels) {
      await expect(
        page.getByText(label).first(),
        `Tab label "${label}" must be visible`
      ).toBeVisible();
    }

    // ── Combat skills panel ──
    await expect(
      page.locator(".combat-skills-layout"),
      "Combat skills layout must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".skill-icon-list"),
      "Skill icon list must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".skill-detail-card"),
      "Skill detail card must be visible"
    ).toBeVisible();

    // Skill icons count should match fixture
    await expect(
      page.locator(".skill-icon-btn"),
      "Skill icon buttons must match fixture count"
    ).toHaveCount(6);

    // At least one locked skill
    await expect(
      page.locator(".skill-icon-btn--locked"),
      "Locked skill icons must be present"
    ).toHaveCount(2);

    // ── Resource bar ──
    await expect(
      page.locator(".hero-resource-bar"),
      "Resource bar must be visible"
    ).toBeVisible();
  });

  test("skill selection updates detail card", async ({ page }) => {
    await page.locator('.hero-tab-btn[data-source-component="CombatSkillButton"]').click();
    await page.waitForTimeout(200);

    // Click the second skill icon
    const skillIcons = page.locator(".skill-icon-btn:not(.skill-icon-btn--locked)");
    const count = await skillIcons.count();
    expect(count).toBeGreaterThanOrEqual(2);

    // Click second available skill
    await skillIcons.nth(1).click();
    await page.waitForTimeout(200);

    // Verify active state
    await expect(
      skillIcons.nth(1),
      "Clicked skill icon must have active state"
    ).toHaveClass(/skill-icon-btn--active/);

    // Verify detail card shows the skill name
    const skillName = await skillIcons.nth(1).getAttribute("data-skill-name");
    expect(skillName).toBeTruthy();
    await expect(
      page.locator(".skill-detail-name"),
      "Detail card must show selected skill name"
    ).toHaveText(skillName!);
  });

  test("hero navigation prev/next cycles roster", async ({ page }) => {
    const initialName = await page.locator(".hero-info-name").textContent();

    // Click next
    await page.locator(".hero-nav-arrow--next").click();
    await page.waitForTimeout(400);

    const nextName = await page.locator(".hero-info-name").textContent();
    expect(nextName).not.toEqual(initialName);

    // Click prev to go back
    await page.locator(".hero-nav-arrow--prev").click();
    await page.waitForTimeout(400);

    const backName = await page.locator(".hero-info-name").textContent();
    expect(backName).toEqual(initialName);
  });

  test("sidebar hero selection switches hero", async ({ page }) => {
    const initialName = await page.locator(".hero-info-name").textContent();

    // Click a non-active hero in sidebar
    const inactiveHero = page.locator(".hero-sidebar-item:not(.hero-sidebar-item--active)").first();
    await inactiveHero.click();
    await page.waitForTimeout(400);

    const newName = await page.locator(".hero-info-name").textContent();
    expect(newName).not.toEqual(initialName);
  });

  test("return to town works from hero detail", async ({ page }) => {
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await page.waitForTimeout(200);

    await expect(
      page.locator(".town-viewport"),
      "Must return to town viewport"
    ).toBeVisible();
  });
});
