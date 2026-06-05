import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

/**
 * HB-8: Hero Panel — Camping Skills page screenshot and verification.
 *
 * Navigates to the hero detail screen, opens the camping-skills tab,
 * and verifies the two-pane layout (skill list + detail pane) with
 * selectable slots and runtime data fidelity.
 */
test("HB-8: camping skills panel layout and interaction", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay → town
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await page.waitForTimeout(400);

  // Open hero detail via the "英雄" utility button
  await page.getByRole("button", { name: "英雄", exact: true }).click();
  await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
  await page.waitForTimeout(400);

  // Switch to Camping Skills tab
  const campingTab = page.locator(
    '.hero-tab-btn[data-source-component="CampingSkillButton"]'
  );
  await expect(campingTab, "Camping tab must be visible").toBeVisible();
  await campingTab.click();
  await page.waitForTimeout(300);

  // ── Layout anchors ──
  await expect(
    page.locator(".camping-skills-panel"),
    "Camping skills panel must mount"
  ).toBeVisible();

  await expect(
    page.locator(".camping-skills-list"),
    "Skill list pane must render"
  ).toBeVisible();

  await expect(
    page.locator(".camping-skill-detail"),
    "Skill detail pane must render"
  ).toBeVisible();

  // ── Skill slots from replay fixtures ──
  const skillSlots = page.locator(".camping-skill-slot");
  await expect(skillSlots, "Must have camping skill slots").toHaveCount(4);

  // First slot should be selected by default
  await expect(
    skillSlots.first(),
    "First skill slot must be selected by default"
  ).toHaveClass(/camping-skill-slot--selected/);

  // ── Detail pane shows first skill data ──
  await expect(
    page.locator(".camping-skill-detail-name"),
    "Selected skill name must be visible"
  ).toHaveText("放松");

  await expect(
    page.locator(".camping-skill-detail-desc"),
    "Selected skill description must be visible"
  ).toContainText("缓解自身压力");

  await expect(
    page.locator(".camping-skill-detail-meta"),
    "Skill meta (level + target) must be visible"
  ).toContainText("Self");

  // ── Interaction: select second skill ──
  await skillSlots.nth(1).click();
  await page.waitForTimeout(200);

  await expect(
    skillSlots.nth(1),
    "Second slot must be selected after click"
  ).toHaveClass(/camping-skill-slot--selected/);

  await expect(
    page.locator(".camping-skill-detail-name"),
    "Detail pane must update to second skill"
  ).toHaveText("营火之歌");

  await expect(
    page.locator(".camping-skill-detail-desc"),
    "Second skill description must show"
  ).toContainText("为全队恢复");

  // ── Screenshot capture ──
  await page.locator(".hero-detail-layout").screenshot({
    path: "test-results/hb8-camping-skills-panel.png"
  });

  // ── Return to town — flow completeness ──
  await page.getByRole("button", { name: "Return to Town" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await page.waitForTimeout(400);

  await expect(
    page.getByText("城镇中枢"),
    "Must return to town after closing hero detail"
  ).toBeVisible();
});
