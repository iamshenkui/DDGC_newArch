import { test } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("HB-9: hero equipment panel capture for visual review", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open hero detail
  await page.getByRole("button", { name: "英雄", exact: true }).click();
  await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
  await page.waitForTimeout(600);

  // Full hero detail layout capture
  await page.screenshot({
    path: "test-results/hb9-hero-equipment-full.png",
    fullPage: true
  });

  // Parchment panel capture
  await page.locator(".hero-detail-parchment").screenshot({
    path: "test-results/hb9-hero-equipment-parchment.png"
  });

  // Equipment panel capture
  await page.locator(".hero-panel--equipment").screenshot({
    path: "test-results/hb9-hero-equipment-panel.png"
  });

  // Verify roster sidebar
  const rosterCount = await page.locator(".hero-roster-thumb").count();
  console.log("Roster heroes count:", rosterCount);
});
