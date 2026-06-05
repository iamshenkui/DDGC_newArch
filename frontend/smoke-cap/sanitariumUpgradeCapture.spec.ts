import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * Sanitarium upgrade screen capture and verification.
 *
 * Validates:
 * 1. Sanitarium building opens with upgrade tab active by default
 * 2. Upgrade trees render with checkbox progression
 * 3. Tab switching works between 升级设施 and 使用设施
 * 4. NPC portrait (梅玲) renders on left panel
 * 5. Screenshots capture the upgrade view for visual review
 */
test("sanitarium-upgrade: capture and verify upgrade screen", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open sanitarium building
  await page.locator('[data-building-id="sanitarium"]').click();
  await page.waitForTimeout(800);

  // Verify building name
  await expect(
    page.locator(".building-detail-name"),
    "Sanitarium building name must be visible"
  ).toHaveText("细胞修复站");

  // Verify upgrade tab is active by default
  await expect(
    page.locator('[data-testid="sanitarium-tab-upgrade"]'),
    "Upgrade tab must be visible"
  ).toBeVisible();

  await expect(
    page.locator('[data-testid="sanitarium-tab-upgrade"]'),
    "Upgrade tab must be active by default"
  ).toHaveClass(/sanitarium-tab-btn--active/);

  // Verify NPC portrait area
  await expect(
    page.locator(".sanitarium-npc-name"),
    "NPC name 梅玲 must be visible"
  ).toHaveText("梅玲");

  // Verify upgrade trees are rendered
  const treeCount = await page.locator(".sanitarium-upgrade-tree").count();
  expect(treeCount, "Must render at least one upgrade tree").toBeGreaterThan(0);

  // Verify each tree has slots
  const firstTreeSlots = await page
    .locator(".sanitarium-upgrade-tree")
    .first()
    .locator(".sanitarium-upgrade-slot")
    .count();
  expect(firstTreeSlots, "First tree must have upgrade slots").toBeGreaterThan(0);

  // Screenshot: upgrade view
  await page.screenshot({
    path: "test-results/sanitarium-upgrade-view.png",
    fullPage: true,
  });

  // Switch to "使用设施" tab
  await page.locator('[data-testid="sanitarium-tab-use"]').click();
  await page.waitForTimeout(400);

  // Verify use tab is now active
  await expect(
    page.locator('[data-testid="sanitarium-tab-use"]'),
    "Use tab must become active after click"
  ).toHaveClass(/sanitarium-tab-btn--active/);

  // Verify action cards are visible in use tab
  await expect(
    page.locator(".building-action-card").first(),
    "Action cards must be visible in use tab"
  ).toBeVisible();

  // Screenshot: use view
  await page.screenshot({
    path: "test-results/sanitarium-use-view.png",
    fullPage: true,
  });

  // Switch back to upgrade tab
  await page.locator('[data-testid="sanitarium-tab-upgrade"]').click();
  await page.waitForTimeout(400);

  await expect(
    page.locator('[data-testid="sanitarium-tab-upgrade"]'),
    "Upgrade tab must be active after switching back"
  ).toHaveClass(/sanitarium-tab-btn--active/);

  // Click "离开" (leave) button to return to town
  await page.locator(".sanitarium-leave-btn").click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Verify back in town
  await expect(
    page.getByText("城镇中枢"),
    "Must return to town after leaving sanitarium"
  ).toBeVisible();
});
