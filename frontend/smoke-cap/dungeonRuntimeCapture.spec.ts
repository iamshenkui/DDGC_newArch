import { test } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("dungeon-runtime pages: capture all eight reference screens", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay and reach town
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // ── Navigate to provisioning ──
  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Proceed to Provisioning" }).click();
  await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(300);

  // Confirm and accept dungeon hint
  await page.locator('[data-testid="footer-btn-launch"]').click();
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Enter Dungeon" }).click();
  await page.waitForTimeout(300);

  // Launch expedition -> dungeon-assist
  await page.getByRole("button", { name: "Launch Expedition" }).click();
  await page.waitForSelector('[data-testid="dungeon-assist-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 1: 副本场景-人物辅助
  await page.screenshot({ path: "test-results/dungeon-runtime-01-assist.png", fullPage: false });

  // Use assist action to enable continue
  await page.locator('[data-testid="assist-action-heal-wound"]').click();
  await page.waitForTimeout(300);

  await page.locator('[data-testid="continue-dungeon-btn"]').click();
  await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 2: 副本场景-地图
  await page.screenshot({ path: "test-results/dungeon-runtime-02-map.png", fullPage: false });

  // Open dungeon items from map
  // Since there is no explicit "open items" button in the UI yet, we dispatch via bridge intent
  // through a small eval to trigger the intent dispatch on the global bridge.
  await page.evaluate(() => {
    (window as unknown as { __ddgcBridge?: { dispatchIntent: (intent: unknown) => Promise<unknown> } }).__ddgcBridge?.dispatchIntent({ type: "open-dungeon-items" });
  });
  await page.waitForSelector('[data-testid="dungeon-items-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 3: 副本场景-物品
  await page.screenshot({ path: "test-results/dungeon-runtime-03-items.png", fullPage: false });

  // Close items and return to map
  await page.locator('[data-testid="dungeon-items-close-btn"]').click();
  await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  // Enter combat room
  await page.locator('[data-room-id="room-combat-1"]').click();
  await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 4: 副本场景-人物攻击
  await page.screenshot({ path: "test-results/dungeon-runtime-04-attack.png", fullPage: false });

  // Confirm attack -> character-hit phase
  await page.locator(".action-primary.combat-attack-btn").click();
  await page.waitForTimeout(500);

  // Screenshot 5: 副本场景-人物受击
  await page.screenshot({ path: "test-results/dungeon-runtime-05-hit.png", fullPage: false });

  // Acknowledge hit -> result
  await page.locator('[data-testid="combat-continue-btn"]').click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 6: 副本结算
  await page.screenshot({ path: "test-results/dungeon-runtime-06-settlement.png", fullPage: false });

  // Continue from result -> return -> town
  await page.getByRole("button", { name: "Proceed to Return" }).click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Resume Town Activities" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });

  // Re-enter expedition flow to reach dungeon-interaction
  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Proceed to Provisioning" }).click();
  await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(300);

  await page.locator('[data-testid="footer-btn-launch"]').click();
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Enter Dungeon" }).click();
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Launch Expedition" }).click();
  await page.waitForSelector('[data-testid="dungeon-assist-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(300);

  await page.locator('[data-testid="assist-action-heal-wound"]').click();
  await page.waitForTimeout(300);

  await page.locator('[data-testid="continue-dungeon-btn"]').click();
  await page.waitForSelector(".dungeon-map-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  // Set dungeon complete via bridge eval to reach dungeon-interaction
  await page.evaluate(() => {
    (window as unknown as { __ddgcBridge?: { dispatchIntent: (intent: unknown) => Promise<unknown> } }).__ddgcBridge?.dispatchIntent({ type: "complete-dungeon" });
  });
  await page.waitForSelector(".dungeon-interaction-viewport", { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 7: 副本场景-交互
  await page.screenshot({ path: "test-results/dungeon-runtime-07-interaction.png", fullPage: false });

  // For combat/battle screenshot, we already captured attack and hit. To capture a pure "battle" view,
  // re-enter a combat room from map. But since we already have attack and hit, we consider the
  // combat screenshots as covering 副本场景-战斗 and 副本场景-人物攻击/受击.
  // To make an explicit "battle" screenshot, retreat from interaction and re-enter combat.
  await page.evaluate(() => {
    (window as unknown as { __ddgcBridge?: { dispatchIntent: (intent: unknown) => Promise<unknown> } }).__ddgcBridge?.dispatchIntent({ type: "retreat-dungeon" });
  });
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(300);

  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });

  // Explicit combat-only screenshot by booting into combat fixture directly
  await page.evaluate(() => {
    (window as unknown as { __ddgcBridge?: { dispatchIntent: (intent: unknown) => Promise<unknown> } }).__ddgcBridge?.dispatchIntent({ type: "boot", mode: "replay" });
  });
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Go through the flow again quickly to reach combat
  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(200);
  await page.getByRole("button", { name: "Proceed to Provisioning" }).click();
  await page.waitForTimeout(200);
  await page.locator('[data-testid="footer-btn-launch"]').click();
  await page.waitForTimeout(200);
  await page.getByRole("button", { name: "Enter Dungeon" }).click();
  await page.waitForTimeout(200);
  await page.getByRole("button", { name: "Launch Expedition" }).click();
  await page.waitForTimeout(200);
  await page.locator('[data-testid="assist-action-heal-wound"]').click();
  await page.waitForTimeout(200);
  await page.locator('[data-testid="continue-dungeon-btn"]').click();
  await page.waitForTimeout(200);
  await page.locator('[data-room-id="room-combat-1"]').click();
  await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
  await page.waitForTimeout(500);

  // Screenshot 8: 副本场景-战斗 (combat overview before any attack)
  await page.screenshot({ path: "test-results/dungeon-runtime-08-combat.png", fullPage: false });
});
