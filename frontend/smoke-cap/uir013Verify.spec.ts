import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * UIR-013: Abbey (Faith Altar / 信仰祭坛) building screen verification.
 *
 * Validates the dedicated AbbeyBuildingScreen renders correctly with:
 * - Building header with correct name and description
 * - Prayer and meditation action sections
 * - Proper button states (available / unavailable)
 * - Return navigation works
 * - No console or page errors
 */
test("UIR-013: abbey building screen fidelity and navigation", async ({ page }) => {
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
  await page.waitForTimeout(400);

  // Open the Abbey building (信仰祭坛)
  await page.locator('[data-building-id="abbey"]').click();
  await page.waitForTimeout(800);

  // Verify building header
  await expect(
    page.locator(".building-detail-eyebrow"),
    "Building eyebrow must be visible"
  ).toHaveText("Building");

  await expect(
    page.locator(".building-detail-name"),
    "Abbey building name must be 信仰祭坛"
  ).toHaveText("信仰祭坛");

  // Verify the description matches the fixture
  await expect(
    page.locator(".building-detail-desc"),
    "Abbey description must match fixture"
  ).toContainText("祈祷");

  // Verify action sections
  await expect(
    page.locator(".building-action-section-title").filter({ hasText: "Prayer" }),
    "Prayer section title must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".building-action-card-header").filter({ hasText: "祈祷" }),
    "Pray action card must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".building-action-card-header").filter({ hasText: "冥想" }),
    "Meditate action card must be visible"
  ).toBeVisible();

  // Verify action button states from fixture:
  // pray is available, meditate is unavailable
  const prayButton = page
    .locator(".building-action-card")
    .filter({ hasText: "祈祷" })
    .locator("button.building-action-btn--primary");
  await expect(prayButton, "Pray button must be enabled (available)").toBeVisible();

  const meditateButton = page
    .locator(".building-action-card")
    .filter({ hasText: "冥想" })
    .locator("button.building-action-btn--disabled");
  await expect(meditateButton, "Meditate button must be disabled (unavailable)").toBeVisible();

  // Verify landscape layout class
  await expect(
    page.locator(".app-frame"),
    "Abbey screen must use .app-frame landscape layout"
  ).toBeVisible();

  // Verify return navigation
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await expect(
    page.getByText("城镇中枢"),
    "Must return to town after leaving abbey"
  ).toBeVisible();

  expect(consoleErrors, "no console.error during abbey flow").toEqual([]);
  expect(pageErrors, "no page errors during abbey flow").toEqual([]);
});
