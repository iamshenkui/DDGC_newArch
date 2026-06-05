import { test } from "@playwright/test";

test("uir-005g replay town capture", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");
  await page.locator('[data-testid="boot-replay"]').click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);
  await page.screenshot({ path: "/tmp/uir-005g-cap/replay-town-current.png", fullPage: false });
});

test("uir-005g live town capture", async ({ page }) => {
  await page.goto("http://localhost:4179");
  await page.waitForLoadState("networkidle");
  await page.locator('[data-testid="boot-live"]').click();
  await page.waitForSelector(".town-viewport", { timeout: 8_000 });
  await page.waitForTimeout(800);
  await page.screenshot({ path: "/tmp/uir-005g-cap/live-town-current.png", fullPage: false });
});
