import { test } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("UIR-008: capture provisioning and expedition launch screens", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.locator('[data-testid="boot-replay"]').click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await page.screenshot({
    path: "test-results/uir008-provisioning.png",
    fullPage: false
  });

  await page.getByRole("button", { name: "Confirm & Launch Expedition" }).click();
  await page.waitForTimeout(400);

  await page.screenshot({
    path: "test-results/uir008-expedition-launch.png",
    fullPage: false
  });
});
