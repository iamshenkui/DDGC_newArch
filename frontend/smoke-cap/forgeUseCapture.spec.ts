import { test, expect } from "@playwright/test";

test("uir-037 forge use screen capture — replay mode", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");

  // Boot into replay mode
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(600);

  // Navigate to blacksmith building
  await page.locator('[data-building-id="blacksmith"]').click();
  await page.waitForTimeout(400);

  // Click the "使用设施" tab to open forge use screen
  await page.getByRole("button", { name: "使用设施" }).click();
  await page.waitForTimeout(400);

  // Verify key elements are present
  await expect(page.locator(".forge-use-tab--active")).toContainText("使用设施");
  await expect(page.locator(".forge-use-npc-desc-name")).toContainText("黑铁匠");
  await expect(page.locator(".forge-use-section-title")).toContainText("角色");

  // Verify hero rows are rendered
  const heroRows = page.locator(".forge-use-hero-row");
  await expect(heroRows).toHaveCount(3);

  // Screenshot for visual regression / fidelity check
  await page.screenshot({
    path: "/tmp/uir-037-cap/forge-use-replay.png",
    fullPage: false,
  });
});

test("uir-037 forge use screen — tab switch back to upgrade", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(600);

  // Open blacksmith
  await page.locator('[data-building-id="blacksmith"]').click();
  await page.waitForTimeout(400);

  // Switch to use tab
  await page.getByRole("button", { name: "使用设施" }).click();
  await page.waitForTimeout(400);

  // Verify we're on forge use screen
  await expect(page.locator(".forge-use-section-title")).toContainText("角色");

  // Switch back to upgrade tab
  await page.getByRole("button", { name: "升级设施" }).first().click();
  await page.waitForTimeout(400);

  // Verify we're back on building detail (upgrade view)
  await expect(page.locator(".building-action-section-title")).toContainText("Weapon Upgrades");
});

test("uir-037 forge use screen — return to town", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(600);

  // Open blacksmith and switch to use tab
  await page.locator('[data-building-id="blacksmith"]').click();
  await page.waitForTimeout(400);
  await page.getByRole("button", { name: "使用设施" }).click();
  await page.waitForTimeout(400);

  // Click leave button to return to town
  await page.getByRole("button", { name: "离开" }).click();
  await page.waitForTimeout(400);

  // Verify we're back in town
  await expect(page.locator(".town-viewport")).toBeVisible();
});
