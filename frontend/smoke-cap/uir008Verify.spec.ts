import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

test("UIR-008: end-to-end provisioning + launch flow without errors", async ({ page }) => {
  const consoleErrors: string[] = [];
  const pageErrors: string[] = [];

  page.on("console", (msg) => {
    if (msg.type() === "error") {
      consoleErrors.push(msg.text());
    }
  });
  page.on("pageerror", (err) => {
    pageErrors.push(err.message);
  });

  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });

  await page.locator(".estate-embark-button").click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });

  // Verify provisioning surface basics
  await expect(page.locator(".party-formation")).toBeVisible();
  await expect(page.locator(".launch-primary")).toBeVisible();
  await expect(page.getByText("Return to Town")).toBeVisible();

  // Add a hero from the roster (if any unselected)
  const rosterHeroes = await page.locator(".provisioning-roster-hero:not([disabled])").count();
  if (rosterHeroes > 0) {
    await page.locator(".provisioning-roster-hero:not([disabled])").first().click();
  }

  // Confirm & Launch should reach the expedition screen
  await page.getByRole("button", { name: /Confirm & Launch Expedition/ }).click();
  await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
  // Confirm we have an expedition title
  await expect(page.locator(".expedition-title")).toBeVisible();

  // Try to use Return to Town from expedition surface
  await page.getByRole("button", { name: /Return to Town/ }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });

  expect(consoleErrors, "no console.error during flow").toEqual([]);
  expect(pageErrors, "no page errors during flow").toEqual([]);
});
