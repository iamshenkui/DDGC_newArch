/**
 * Focused browser smoke for the first-combat demo entry added in DDGC H5 Demo M5.
 *
 * Proves the demo is reachable from the existing startup screen, routes through
 * the replay bridge, and renders the existing migrated CombatScreen with the
 * first-combat fixture content — not a separate /demo page.
 *
 * Run with: npm run build && npm run smoke-browser -- smoke/firstCombatDemo.spec.ts
 * Or focused equivalent (jsdom): npm run test -- src/screens/startup/StartupScreen.test.tsx src/screens/combat/CombatScreen.test.tsx
 */

import { test, expect, type Page } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

function setupErrorCollectors(page: Page): {
  consoleErrors: string[];
  pageErrors: string[];
} {
  const consoleErrors: string[] = [];
  const pageErrors: string[] = [];

  page.on("console", (msg) => {
    if (msg.type() === "error") {
      consoleErrors.push(`[console.error] ${msg.text()}`);
    }
  });
  page.on("pageerror", (err) => {
    pageErrors.push(`[page.error] ${err.message}`);
  });

  return { consoleErrors, pageErrors };
}

async function settle(page: Page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

test.describe("browser smoke: first-combat demo entry", () => {
  test("startup → First Combat Demo → existing CombatScreen renders non-blank", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await expect(
      page.getByRole("button", { name: "First Combat Demo" }),
      "First Combat Demo entry must be visible on startup"
    ).toBeVisible();

    await page.getByRole("button", { name: "First Combat Demo" }).click();
    await page.waitForSelector(".combat-viewport", { timeout: 8_000 });
    await settle(page);

    await expect(
      page.locator(".combat-viewport"),
      "Existing migrated CombatScreen must render"
    ).toBeVisible();

    await expect(
      page.getByRole("heading", { name: "初战：苍灯林地" }),
      "First-combat fixture title must be visible"
    ).toBeVisible();

    await expect(
      page.locator(".combat-enemy-stand"),
      "First-combat encounter must render three QingLong mantis flower enemies"
    ).toHaveCount(3);

    await expect(
      page.locator(".combat-hero-stand"),
      "First-combat party must render four heroes"
    ).toHaveCount(4);

    await expect(
      page.getByText("Magic Mantis Flower").first(),
      "Magic Mantis Flower enemy must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("Spiny Mantis Flower").first(),
      "Spiny Mantis Flower enemy must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("Walking Mantis Flower").first(),
      "Walking Mantis Flower enemy must be visible"
    ).toBeVisible();

    // A combat interaction remains usable: select the active hero's skill.
    await page.locator('[data-skill-id="hunter-mark"]').click();
    await settle(page, 200);

    await expect(
      page.locator(".combat-skill-slot--selected"),
      "Selected skill must be highlighted after click"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "Confirm Attack" }),
      "Confirm Attack must be enabled after selecting a skill"
    ).toBeEnabled();

    expect(pageErrors).toEqual([]);
    expect(consoleErrors).toEqual([]);
  });
});
