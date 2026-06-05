import { test, expect } from "@playwright/test";

const BASE_URL = "http://localhost:4179";

async function settle(page, ms = 400) {
  await page.waitForTimeout(ms);
}

test("HB-5: hero panel 人物信息 capture and verification", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  // Boot replay and navigate to hero detail
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await settle(page, 400);

  await page.getByRole("button", { name: "英雄", exact: true }).click();
  await page.waitForSelector('[data-testid="hero-detail-screen"]', { timeout: 5_000 });
  await settle(page, 400);

  // ── Screenshot anchors ──
  const viewportShot = await page.locator('[data-testid="hero-detail-screen"]').screenshot({
    path: "test-results/hb5-hero-panel-viewport.png"
  });
  expect(viewportShot).toBeTruthy();

  // ── Layout anchors ──
  await expect(
    page.locator('[data-testid="hero-detail-screen"]'),
    "Hero panel viewport must render"
  ).toBeVisible();

  await expect(
    page.locator(".hero-panel-sidebar"),
    "Left sidebar must render"
  ).toBeVisible();

  await expect(
    page.locator(".hero-panel-center"),
    "Center artwork area must render"
  ).toBeVisible();

  await expect(
    page.locator(".hero-panel-right"),
    "Right panel must render"
  ).toBeVisible();

  // ── Sidebar content anchors ──
  await expect(
    page.locator(".hero-panel-name"),
    "Hero name must be visible"
  ).toHaveText("Shen");

  await expect(
    page.locator(".hero-panel-desc"),
    "Hero description must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".hero-panel-talent"),
    "Hero talent section must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".hero-panel-exile"),
    "Exile button must be visible"
  ).toBeVisible();

  // ── Tab strip anchors ──
  const expectedTabs = ["info", "state", "camping", "combat", "equipment"];
  for (const key of expectedTabs) {
    const tab = page.locator(`.hero-panel-tab[data-tab-key="${key}"]`);
    await expect(tab, `Tab "${key}" must be visible`).toBeVisible();
  }

  // ── Default tab (info) content anchors ──
  await expect(
    page.locator('.hero-panel-tab--active[data-tab-key="info"]'),
    "Info tab must be active by default"
  ).toBeVisible();

  await expect(
    page.locator(".hero-class-name"),
    "Class name must be visible"
  ).toBeVisible();

  await expect(
    page.locator(".hero-stats-grid"),
    "Stats grid must be visible"
  ).toBeVisible();

  // Stats should contain real data from viewModel
  await expect(
    page.locator(".hero-stats-grid"),
    "Stats must contain HP"
  ).toContainText("生命");

  await expect(
    page.locator(".hero-stats-grid"),
    "Stats must contain dodge"
  ).toContainText("闪避");

  await expect(
    page.locator(".hero-stats-grid"),
    "Stats must contain crit"
  ).toContainText("暴击率");

  await expect(
    page.locator(".hero-stats-grid"),
    "Stats must contain damage"
  ).toContainText("伤害");

  await expect(
    page.locator(".hero-stats-grid"),
    "Stats must contain speed"
  ).toContainText("速度");

  // Traits section
  await expect(
    page.locator(".hero-traits-section"),
    "Traits section must be visible"
  ).toBeVisible();

  // ── Tab navigation: cycle through all tabs ──
  for (const key of expectedTabs) {
    await page.locator(`.hero-panel-tab[data-tab-key="${key}"]`).click();
    await settle(page, 200);

    await expect(
      page.locator(`.hero-panel-tab--active[data-tab-key="${key}"]`),
      `Tab "${key}" must become active after click`
    ).toBeVisible();

    // Capture each tab state
    await page.locator('[data-testid="hero-detail-screen"]').screenshot({
      path: `test-results/hb5-hero-panel-tab-${key}.png`
    });
  }

  // ── Back navigation ──
  await page.getByRole("button", { name: "返回" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 5_000 });
  await settle(page);

  await expect(
    page.locator(".town-viewport"),
    "Must return to town after clicking back"
  ).toBeVisible();

  // ── Full page screenshot for visual review ──
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");
  await page.getByRole("button", { name: "Boot Replay" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await settle(page, 400);
  await page.getByRole("button", { name: "英雄", exact: true }).click();
  await page.waitForSelector('[data-testid="hero-detail-screen"]', { timeout: 5_000 });
  await settle(page, 600);

  await page.screenshot({
    path: "test-results/hb5-hero-panel-fullpage.png",
    fullPage: true
  });
});

test("HB-5: live boot hero panel verification", async ({ page }) => {
  await page.setViewportSize({ width: 1600, height: 1000 });
  await page.goto(BASE_URL);
  await page.waitForLoadState("networkidle");

  await page.getByRole("button", { name: "Boot Live" }).click();
  await page.waitForSelector(".town-viewport", { timeout: 10_000 });
  await settle(page, 400);

  await page.getByRole("button", { name: "英雄", exact: true }).click();
  await page.waitForSelector('[data-testid="hero-detail-screen"]', { timeout: 5_000 });
  await settle(page, 400);

  await expect(
    page.locator(".hero-panel-name"),
    "Live hero detail name must be visible"
  ).toHaveText("Yuan");

  await expect(
    page.locator(".hero-class-name"),
    "Live hero detail class must be visible"
  ).toHaveText("Hunter");

  await page.locator('[data-testid="hero-detail-screen"]').screenshot({
    path: "test-results/hb5-hero-panel-live.png"
  });
});
