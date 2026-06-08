import { test } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("UIR-008: detailed capture for visual review", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(800);

  await page.getByRole("button", { name: "Proceed to Provisioning" }).click();
  await page.waitForSelector('[data-testid="provisioning-screen"]', { timeout: 5_000 });
  await page.waitForTimeout(800);

  // Provisioning capture
  await page.screenshot({
    path: "test-results/uir008-detail-provisioning.png",
    fullPage: true
  });

  // Capture in a tighter region for inspection
  await page.locator(".expedition-viewport").screenshot({
    path: "test-results/uir008-detail-provisioning-viewport.png"
  });

  // Verify roster scroll is reachable
  const rosterCount = await page.locator(".provisioning-roster-hero").count();
  console.log("Roster heroes count:", rosterCount);

  // Click roster to add a hero
  if (rosterCount > 0) {
    await page.locator(".provisioning-roster-hero:not([disabled])").first().click();
    await page.waitForTimeout(300);
    await page.locator(".expedition-viewport").screenshot({
      path: "test-results/uir008-detail-after-add.png"
    });
  }

  // Confirm and Launch
  await page.locator('[data-testid="footer-btn-launch"]').click();
  await page.waitForTimeout(800);

  await page.locator(".expedition-viewport").screenshot({
    path: "test-results/uir008-detail-expedition.png"
  });
});
