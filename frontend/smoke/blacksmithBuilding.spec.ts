import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * Blacksmith building screen smoke test.
 *
 * Validates the migrated page: 公会界面-锻造仓-使用空
 * - Building opens from town
 * - Tab switcher (升级设施 / 使用设施) is present
 * - Forge slot grid renders with correct empty/locked states
 * - NPC panel is visible
 * - Resource bar shows material counts
 * - Return navigation works
 */
test("blacksmith building: forge empty usage layout", async ({ page }) => {
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(400);

  // Open blacksmith building
  await page.locator('[data-building-id="blacksmith"]').click();
  await page.waitForTimeout(800);

  // ── Building header ──
  await expect(
    page.locator(".building-detail-name"),
    "Blacksmith building name must be visible (DDGC display name 锻造舱)"
  ).toHaveText("锻造舱");

  // ── NPC panel ──
  await expect(
    page.locator(".forge-npc-panel"),
    "NPC panel must be visible"
  ).toBeVisible();
  await expect(
    page.locator(".forge-npc-name"),
    "NPC name must be visible"
  ).toHaveText("铁匠");

  // ── Tab switcher ──
  await expect(
    page.locator('.forge-tab[data-tab="upgrade"]'),
    "Upgrade tab must be visible"
  ).toBeVisible();
  await expect(
    page.locator('.forge-tab[data-tab="use"]'),
    "Use tab must be visible"
  ).toBeVisible();

  // Default active tab is "use"
  await expect(
    page.locator('.forge-tab[data-tab="use"]'),
    "Use tab must be active by default"
  ).toHaveClass(/forge-tab--active/);

  // ── Forge slot grid (use tab) ──
  await expect(
    page.locator(".forge-slot-grid"),
    "Forge slot grid must be visible"
  ).toBeVisible();

  // 10 slots total (5 weapon + 5 armor)
  await expect(
    page.locator(".forge-slot"),
    "Must render 10 forge slots"
  ).toHaveCount(10);

  // 2 empty slots
  await expect(
    page.locator(".forge-slot--empty"),
    "Must have 2 empty slots"
  ).toHaveCount(2);

  // 8 locked slots
  await expect(
    page.locator(".forge-slot--locked"),
    "Must have 8 locked slots"
  ).toHaveCount(8);

  // ── Empty state hint ──
  await expect(
    page.locator(".forge-empty-hint"),
    "Empty state hint must be visible when all slots are empty"
  ).toBeVisible();

  // ── Detail area ──
  await expect(
    page.locator(".forge-detail-area"),
    "Forge detail area must be visible"
  ).toBeVisible();

  // ── Resource bar ──
  await expect(
    page.locator(".forge-resources-bar"),
    "Resource bar must be visible"
  ).toBeVisible();

  // 5 resources: iron, leather, crystal, essence, gold
  await expect(
    page.locator(".forge-resource-item"),
    "Must render 5 resource counters"
  ).toHaveCount(5);

  // Verify specific resource values
  await expect(
    page.locator('[data-resource-id="mat-iron"] .forge-resource-value'),
    "Iron count must be 10"
  ).toHaveText("10");
  await expect(
    page.locator('[data-resource-id="gold"] .forge-resource-value'),
    "Gold count must be 4295"
  ).toHaveText("4295");

  // ── Tab switching: upgrade tab ──
  await page.locator('.forge-tab[data-tab="upgrade"]').click();
  await page.waitForTimeout(300);

  await expect(
    page.locator('.forge-tab[data-tab="upgrade"]'),
    "Upgrade tab must become active after click"
  ).toHaveClass(/forge-tab--active/);

  await expect(
    page.locator(".building-action-section").first(),
    "Upgrade actions must be visible on upgrade tab"
  ).toBeVisible();

  // ── Return to town ──
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await expect(
    page.getByText("城镇中枢"),
    "Must be back at town after returning from blacksmith"
  ).toBeVisible();
});
