/**
 * Hero Panel — Character Status tab fidelity test.
 *
 * Validates the 英雄面板-人物状态 page against the reference image:
 * - Left roster sidebar with hero thumbnails
 * - State tab active by default
 * - After-effects (后遗症) section visible
 * - Personality/traits (性格) section visible
 * - Hero switching via roster thumbnails
 */

import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page, ms = 400) {
  await page.waitForTimeout(ms);
}

test.describe("hero panel — character status tab", () => {
  test("state tab renders with personality traits and hero roster", async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    // Boot replay and open hero detail
    await page.getByRole("button", { name: "Boot Replay" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page);

    // ── Roster sidebar ──
    await expect(
      page.locator(".hero-detail-roster"),
      "Hero roster sidebar must be visible"
    ).toBeVisible();

    await expect(
      page.locator(".hero-roster-thumb"),
      "Roster must contain hero thumbnails"
    ).toHaveCount(3);

    const activeThumb = page.locator(".hero-roster-thumb--active");
    await expect(activeThumb, "Exactly one roster thumb must be active").toHaveCount(1);
    await expect(
      activeThumb,
      "Active thumb must match current hero"
    ).toHaveAttribute("data-hero-id", "hero-hunter-01");

    // ── State tab is active by default ──
    await expect(
      page.locator('.hero-tab-btn--active[data-source-component="StateButton"]'),
      "State tab must be active by default"
    ).toBeVisible();

    await expect(
      page.locator('[data-testid="hero-state-panel"]'),
      "State panel must be rendered"
    ).toBeVisible();

    // ── After-effects (后遗症) ──
    await expect(
      page.locator('[data-source-component="AftereffectsPanel"]'),
      "After-effects panel must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("后遗症", { exact: true }),
      "After-effects heading (后遗症) must be visible"
    ).toBeVisible();

    await expect(
      page.getByText("创伤"),
      "Wound label (创伤) must be visible"
    ).toBeVisible();

    // ── Personality / Traits (性格) ──
    await expect(
      page.locator('[data-source-component="PersonalityPanel"]'),
      "Personality panel must be visible"
    ).toBeVisible();

    await expect(
      page.locator('[data-source-component="PersonalityPanel"] .hero-panel-heading'),
      "Personality heading (性格) must be visible"
    ).toContainText("性格");

    await expect(
      page.locator(".personality-chip"),
      "Personality chips must be rendered"
    ).toHaveCount(3);

    await expect(
      page.locator(".personality-chip--positive").filter({ hasText: "谨慎" }),
      "Positive personality trait (谨慎) must be visible"
    ).toBeVisible();

    await expect(
      page.locator(".personality-chip--negative").filter({ hasText: "固执" }),
      "Negative personality trait (固执) must be visible"
    ).toBeVisible();

    // ── Resistances still visible in state tab ──
    await expect(
      page.getByText("Resistances"),
      "Resistances section must still be visible"
    ).toBeVisible();

    // ── Hero switching via roster ──
    const secondHeroThumb = page.locator('.hero-roster-thumb[data-hero-id="hero-white-01"]');
    await secondHeroThumb.click();
    await settle(page, 600);

    await expect(
      page.locator(".hero-roster-thumb--active"),
      "Active thumb must switch to selected hero"
    ).toHaveAttribute("data-hero-id", "hero-white-01");

    await expect(
      page.getByText("Bai Xiu — White"),
      "Hero detail title must update to switched hero"
    ).toBeVisible();

    // State tab should still be active after switching
    await expect(
      page.locator('[data-testid="hero-state-panel"]'),
      "State panel must remain visible after hero switch"
    ).toBeVisible();
  });
});
