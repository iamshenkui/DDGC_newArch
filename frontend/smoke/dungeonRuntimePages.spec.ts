/**
 * Dungeon runtime page family screenshot parity tests.
 *
 * Covers all eight reference pages:
 *   副本场景-交互, 副本场景-人物受击, 副本场景-人物攻击, 副本场景-人物辅助,
 *   副本场景-地图, 副本场景-战斗, 副本场景-物品, 副本结算
 *
 * Each test boots replay mode, navigates to the target screen, asserts stable
 * click targets, and captures a screenshot for visual parity review.
 */

import { test, expect, type Page } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page: Page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

async function bootReplayAndReachDungeonInteraction(page: Page): Promise<void> {
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await settle(page);

  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await settle(page);

  await page.locator('[data-dungeon-id="dungeon-ruins-01"]').click();
  await settle(page);
  await page.locator('.dungeon-select-roster-hero').first().click();
  await settle(page);
  await page.locator('.dungeon-select-roster-hero').first().click();
  await settle(page);
  await page.getByRole("button", { name: "确认出征" }).click();
  await settle(page);

  await page.locator('[data-testid="footer-btn-launch"]').click();
  await settle(page);

  await page.getByRole("button", { name: "Enter Dungeon" }).click();
  await settle(page);

  await page.getByRole("button", { name: "Launch Expedition" }).click();
  await page.waitForSelector(".dungeon-assist-viewport", { timeout: 5_000 });
  await settle(page);

  await page.getByTestId("assist-action-heal-wound").click();
  await settle(page);
  await page.getByRole("button", { name: "Continue Expedition" }).click();
  await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
  await settle(page);

  // Enter a non-combat room first to reach dungeon-interaction
  await page.locator('[data-room-id="room-empty-1"] .dungeon-room-btn').click();
  await page.waitForSelector(".dungeon-interaction-viewport", { timeout: 5_000 });
  await settle(page);
}

async function expectNoConsoleErrors(page: Page, phase: string): Promise<void> {
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") {
      errors.push(msg.text());
    }
  });
  page.on("pageerror", (err) => {
    errors.push(err.message);
  });
  // Give a moment for any errors to accumulate after navigation
  await settle(page, 200);
  expect(errors, `${phase}: no console or page errors`).toEqual([]);
}

test.describe("dungeon runtime page family screenshots", () => {
  test("副本场景-交互 — dungeon interaction", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    await expect(page.locator(".dungeon-interaction-viewport")).toBeVisible();
    await expect(page.getByText("Dungeon Exploration")).toBeVisible();
    await expect(page.getByRole("button", { name: "Inventory" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Proceed" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Retreat" })).toBeVisible();

    await page.screenshot({ path: "test-results/dungeon-interaction.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-interaction");
  });

  test("副本场景-人物辅助 — dungeon assist", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    // Go back to map, then enter a combat room to trigger assist after combat?
    // Actually assist is the pre-map screen. Re-navigate via expedition launch.
    await page.getByRole("button", { name: "Retreat" }).click();
    await settle(page);
    // Retreat goes to result from interaction; return to town and relaunch to get assist
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await page.locator(".estate-embark-button").click();
    await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
    await settle(page);
    await page.locator('[data-dungeon-id="dungeon-ruins-01"]').click();
    await settle(page);
    await page.locator('.dungeon-select-roster-hero').first().click();
    await settle(page);
    await page.locator('.dungeon-select-roster-hero').first().click();
    await settle(page);
    await page.getByRole("button", { name: "确认出征" }).click();
    await settle(page);
    await page.locator('[data-testid="footer-btn-launch"]').click();
    await settle(page);
    await page.getByRole("button", { name: "Enter Dungeon" }).click();
    await settle(page);
    await page.getByRole("button", { name: "Launch Expedition" }).click();
    await page.waitForSelector(".dungeon-assist-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(page.locator(".dungeon-assist-viewport")).toBeVisible();
    await expect(page.getByRole("heading", { name: "人物辅助" })).toBeVisible();
    await expect(page.getByTestId("assist-action-heal-wound")).toBeVisible();
    await expect(page.getByTestId("hero-detail-panel")).toBeVisible();

    await page.screenshot({ path: "test-results/dungeon-assist.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-assist");
  });

  test("副本场景-地图 — dungeon map", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    // Return to map from interaction
    await page.getByRole("button", { name: "Return" }).first().click();
    await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(page.locator(".dungeon-map-viewport")).toBeVisible();
    await expect(page.getByText("Dungeon Exploration")).toBeVisible();
    await expect(page.locator("[data-room-id]")).toHaveCount(10);
    await expect(page.getByTestId("open-inventory-map-btn")).toBeVisible();

    await page.screenshot({ path: "test-results/dungeon-map.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-map");
  });

  test("副本场景-战斗 — combat", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    // Return to map and enter combat room
    await page.getByRole("button", { name: "Return" }).first().click();
    await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-room-id="room-combat-1"] .dungeon-room-btn').click();
    await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(page.locator(".combat-viewport")).toBeVisible();
    await expect(page.getByRole("heading", { name: "副本场景-人物攻击" })).toBeVisible();
    await expect(page.locator(".combat-enemy-stand")).toHaveCount(2);
    await expect(page.getByRole("button", { name: "Confirm Attack" })).toBeVisible();

    await page.screenshot({ path: "test-results/dungeon-combat.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-combat");
  });

  test("副本场景-人物受击 — character hit", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    await page.getByRole("button", { name: "Return" }).first().click();
    await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-room-id="room-combat-1"] .dungeon-room-btn').click();
    await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
    await settle(page);

    await page.getByRole("button", { name: "Confirm Attack" }).click();
    await settle(page);

    await expect(page.locator(".combat-viewport--character-hit")).toBeVisible();
    await expect(page.getByRole("button", { name: "Acknowledge" })).toBeVisible();
    await expect(page.getByTestId("combat-hit-damage")).toBeVisible();

    await page.screenshot({ path: "test-results/dungeon-character-hit.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-character-hit");
  });

  test("副本场景-物品 — dungeon items", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    await page.getByRole("button", { name: "Inventory" }).click();
    await page.waitForSelector(".dungeon-items-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(page.locator(".dungeon-items-viewport")).toBeVisible();
    await expect(page.getByText("Dungeon Inventory")).toBeVisible();
    await expect(page.getByTestId("items-inventory-panel")).toBeVisible();
    await expect(page.getByTestId("items-detail-panel")).toBeVisible();
    await expect(page.getByTestId("dungeon-item-supply-food")).toBeVisible();
    await expect(page.getByTestId("item-target-hero-hero-hunter-01")).toBeVisible();
    await expect(page.getByTestId("use-item-btn")).toBeVisible();
    await expect(page.getByTestId("close-items-btn")).toBeVisible();

    // Exercise selection and target
    await page.getByTestId("dungeon-item-supply-bandage").click();
    await settle(page);
    await page.getByTestId("item-target-hero-hero-white-01").click();
    await settle(page);

    await page.screenshot({ path: "test-results/dungeon-items.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-items");

    // Close returns to dungeon-interaction
    await page.getByTestId("close-items-btn").click();
    await page.waitForSelector(".dungeon-interaction-viewport", { timeout: 5_000 });
    await settle(page);
    await expect(page.locator(".dungeon-interaction-viewport")).toBeVisible();
  });

  test("副本结算 — dungeon result", async ({ page }) => {
    await bootReplayAndReachDungeonInteraction(page);

    await page.getByRole("button", { name: "Return" }).first().click();
    await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-room-id="room-combat-1"] .dungeon-room-btn').click();
    await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
    await settle(page);

    await page.getByRole("button", { name: "Confirm Attack" }).click();
    await settle(page);
    await page.getByRole("button", { name: "Acknowledge" }).click();
    await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(page.getByRole("heading", { name: "Expedition Complete" })).toBeVisible();
    await expect(page.getByText("Ancient Gold Coin")).toBeVisible();
    await expect(page.getByRole("button", { name: "Proceed to Return" })).toBeVisible();

    await page.screenshot({ path: "test-results/dungeon-result.png", fullPage: false });
    await expectNoConsoleErrors(page, "dungeon-result");
  });
});
