/**
 * Focused browser smoke test for the first-combat demo entry.
 *
 * Run with:
 *   cd frontend && npm run build && npx playwright test smoke/firstCombatDemo.spec.ts
 *
 * This proves the first-combat demo launches from the existing startup flow,
 * renders the migrated CombatScreen (not a new isolated page), and supports a
 * complete combat interaction: select skill → confirm attack → acknowledge hit
 * → reach the result screen.
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

test.describe("first-combat demo browser smoke", () => {
  test("startup → First Combat Demo → combat → result", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await expect(
      page.getByRole("heading", { name: "DDGC" }),
      "Startup title must be visible"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "First Combat Demo" }),
      "First Combat Demo entry must be present on startup"
    ).toBeVisible();

    await page.getByRole("button", { name: "First Combat Demo" }).click();
    await page.waitForSelector(".combat-viewport", { timeout: 8_000 });
    await settle(page);

    await expect(
      page.locator(".combat-viewport"),
      "Existing CombatScreen viewport must render"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "初战：苍灯林地" }),
      "First-combat title must be visible"
    ).toBeVisible();

    await expect(
      page.locator(".combat-hero-stand"),
      "First-combat party must render"
    ).toHaveCount(4);
    await expect(
      page.locator(".combat-enemy-stand"),
      "First-combat enemies must render"
    ).toHaveCount(3);

    await expect(
      page.locator(".combat-skill-slot"),
      "Active hero skills must render"
    ).toHaveCount(5);

    await expect(
      page.getByRole("button", { name: "Confirm Attack" }),
      "Confirm Attack button must be present"
    ).toBeVisible();

    await expect(
      page.locator(".combat-target-cell--selected"),
      "A default enemy target must be selected"
    ).toBeVisible();

    // Use a combat interaction: select a skill, confirm attack, acknowledge hit.
    // The skill selection updates the bridge view model; Confirm Attack becomes
    // enabled because a skill is now selected.
    await expect(
      page.getByRole("button", { name: "Confirm Attack" }),
      "Confirm Attack must start disabled until a skill is selected"
    ).toBeDisabled();

    await page.locator(".combat-skill-slot").first().click();
    await settle(page);

    await expect(
      page.getByRole("button", { name: "Confirm Attack" }),
      "Confirm Attack must become enabled after selecting a skill"
    ).toBeEnabled();

    await page.getByRole("button", { name: "Confirm Attack" }).click();
    await settle(page);

    await expect(
      page.getByRole("button", { name: "Acknowledge" }),
      "Acknowledge button must appear after resolving the attack"
    ).toBeVisible();

    await page.getByRole("button", { name: "Acknowledge" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Complete" }),
      "Result screen must be reached after acknowledging the hit"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Expedition Complete" }),
      "Expedition Complete heading must be visible"
    ).toBeVisible();

    expect(pageErrors).toEqual([]);
    expect(consoleErrors).toEqual([]);
  });
});
