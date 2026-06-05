import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("legacy tower building detail capture — replay", async ({ page }) => {
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);

  await page.locator('[data-building-id="legacytower"]').click();
  await page.waitForTimeout(800);

  // Verify dedicated screen mounts before capture
  await expect(
    page.locator('[data-source-hierarchy="LegacyTowerWindow/LeftPanel"]')
  ).toBeVisible();
  await expect(
    page.locator('.building-detail-name')
  ).toHaveText('维度灯塔');

  await page.screenshot({ path: "/tmp/legacytower-cap/replay-legacytower-detail.png", fullPage: false });
});

test("legacy tower building detail capture — live", async ({ page }) => {
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Live" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);

  await page.locator('[data-building-id="legacytower"]').click();
  await page.waitForTimeout(800);

  await expect(
    page.locator('[data-source-hierarchy="LegacyTowerWindow/LeftPanel"]')
  ).toBeVisible();

  await page.screenshot({ path: "/tmp/legacytower-cap/live-legacytower-detail.png", fullPage: false });
});
