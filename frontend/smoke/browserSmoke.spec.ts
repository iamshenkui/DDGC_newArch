/**
 * Browser smoke tests — fidelity gates for the rendered DDGC frontend.
 *
 * Run with: npx playwright test
 * After build: npm run build && npx playwright test
 * Full suite:  npm run smoke-build
 *
 * Validates:
 * 1. Startup → replay boot → town shell (deterministic replay bridge)
 * 2. Hero detail screen with tab navigation
 * 3. Building detail screen with actions
 * 4. Full meta-loop: provisioning → expedition → result → return → town
 * 5. Live bridge boot path
 * 6. No page errors, no console errors
 * 7. Completed product surfaces free of placeholder/skeletal/reserved canvas language
 * 8. Landscape viewport classes present on core game screens
 */

import { test, expect, type Page } from "@playwright/test";

// ── Constants ──────────────────────────────────────────────────────────────────

const BASE_URL = "http://localhost:4179";

/**
 * Patterns that indicate placeholder, skeletal, or incomplete-rendering
 * content on screens that should be completed product surfaces.
 * The startup/boot screen is exempt — it is a launcher, not a product surface.
 */
const FIDELITY_BLOCKLIST = [
  /placeholder/i,
  /skeletal/i,
  /skeleton/i,
  /reserved canvas/i,
  /text.based rendering/i,
  /rendering completion/i
] as const;

// ── Helpers ────────────────────────────────────────────────────────────────────

/** Attach console/page-error collectors and return shared references. */
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

/** Assert no console or page errors accumulated. */
function expectNoErrors(
  pageErrors: string[],
  consoleErrors: string[],
  phase: string
): void {
  expect(
    pageErrors,
    `${phase}: page errors must be empty`
  ).toEqual([]);
  expect(
    consoleErrors,
    `${phase}: console errors must be empty`
  ).toEqual([]);
}

/**
 * Assert the text content of a locator does not contain any blocklisted
 * fidelity patterns.
 */
async function expectFidelity(
  locator: ReturnType<Page["locator"]>,
  screenName: string
): Promise<void> {
  const text = await locator.innerText();
  for (const pattern of FIDELITY_BLOCKLIST) {
    expect(
      text,
      `${screenName}: must not contain prohibited pattern "${pattern}"`
    ).not.toMatch(pattern);
  }
}

/**
 * Full-page fidelity scan — reads all visible text from the page body
 * and checks it is free of blocklisted placeholder/skeletal language.
 * Catches text outside the primary content locator.
 */
async function expectFullPageFidelity(
  page: Page,
  screenName: string
): Promise<void> {
  const bodyText = await page.locator("body").innerText();
  for (const pattern of FIDELITY_BLOCKLIST) {
    expect(
      bodyText,
      `${screenName} (full page): must not contain prohibited pattern "${pattern}"`
    ).not.toMatch(pattern);
  }
}

async function expectOriginalAssetImage(
  locator: ReturnType<Page["locator"]>,
  srcFragment: string,
  description: string
): Promise<void> {
  await expect(locator, `${description}: image must be visible`).toBeVisible();
  await expect(locator, `${description}: image src must use staged original assets`).toHaveAttribute(
    "src",
    new RegExp(srcFragment.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"))
  );
}

/** Wait for Solid.js reactive updates to settle after a dispatch */
async function settle(page: Page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

// ── Tests ──────────────────────────────────────────────────────────────────────

test.describe("browser smoke: fidelity gates", () => {
  test("replay boot → full meta-loop → fidelity → viewport", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    // ── Phase 0: Navigate ──────────────────────────────────
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    // ── Phase 1: Startup screen ────────────────────────────
    await expect(
      page.getByRole("heading", { name: "DDGC" }),
      "Startup title must be visible"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "New Campaign" }),
      "New Campaign button must be present"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "Load Campaign" }),
      "Load Campaign button must be present"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "Boot Replay" }),
      "Replay boot button must be present"
    ).toBeVisible();

    await expect(
      page.getByRole("button", { name: "Boot Live" }),
      "Live boot button must be present"
    ).toBeVisible();

    // ── Phase 2: Replay boot → Town shell ──────────────────
    await page.getByRole("button", { name: "Boot Replay" }).click();

    // Wait for landscape town viewport to mount
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // Verify town content from replay fixtures
    // Note: town shell renders Chinese labels matching the source DDGC game (UIR-005C).
    await expect(
      page.getByText("城镇中枢"),
      "Estate eyebrow (城镇中枢) must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "苍灯远征" }),
      "Replay campaign name (苍灯远征) must be visible"
    ).toBeVisible();
    await expect(page.getByText("1250"), "Gold amount must be visible").toBeVisible();

    // Buildings — full 11-building screenshot anchor (Chinese display names).
    const expectedBuildings = [
      "次元感知塔", "试炼场", "锻造舱", "细胞修复站",
      "信仰祭坛", "迷情乐园", "英雄档案馆", "天国花园",
      "维度灯塔", "交易市场", "空间分析"
    ];
    for (const label of expectedBuildings) {
      await expect(
        page.getByText(label).first(),
        `Building "${label}" must be visible`
      ).toBeVisible();
    }
    await expect(
      page.locator(".estate-building-node"),
      "All 11 building nodes must render in the estate stage"
    ).toHaveCount(11);

    await expectOriginalAssetImage(
      page.locator('.building-icon-image[src*="/original/buildings/"]').first(),
      "/original/buildings/",
      "Town building marker"
    );

    // Fidelity — town is a completed product surface
    await expectFidelity(page.locator(".town-viewport"), "Town screen");
    await expectFullPageFidelity(page, "Town screen");

    // Landscape viewport check for town screen
    await expect(
      page.locator(".town-viewport"),
      "Town must use .town-viewport for landscape layout"
    ).toBeVisible();

    // Estate layout check — town uses three-zone estate layout from Unity EstateManagement.unity.
    // UIR-005D screenshot anchors: top nameplate, side buttons (6), embark control, currency strip (5), 11 buildings.
    await expect(
      page.locator(".town-viewport"),
      "Town must use .town-viewport for landscape layout"
    ).toBeVisible();
    await expect(
      page.locator(".estate-top-panel"),
      "Anchor A-NAMEPLATE: top nameplate (EstateNameplate) must be present"
    ).toBeVisible();
    await expect(
      page.locator(".estate-stage-shell"),
      "Town must have an estate central stage (UI_Estate building collection)"
    ).toBeVisible();
    await expect(
      page.locator(".estate-bottom-panel"),
      "Town must have an estate bottom panel (EmbarkButton)"
    ).toBeVisible();
    await expect(
      page.locator(".estate-side-button"),
      "Anchor A-SIDEBUTTONS: 3 top-right utility buttons must render"
    ).toHaveCount(3);
    await expect(
      page.locator(".estate-embark-button"),
      "Anchor A-EMBARK: embark control must be present"
    ).toBeVisible();
    await expect(
      page.locator(".estate-currency-slot"),
      "Anchor A-CURRENCY: currency strip must contain 5 slots (bust/portrait/deed/crest/gold)"
    ).toHaveCount(5);

    expectNoErrors(pageErrors, consoleErrors, "Phase 2 (town)");

    // ── Phase 3: Hero detail ───────────────────────────────
    // Click the "英雄" top-right utility button to open hero detail
    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("Hero Detail"),
      "Hero detail eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Shen — Hunter"),
      "Hero detail title (name + class) must be visible"
    ).toBeVisible();
    await expectOriginalAssetImage(
      page.locator('.hero-portrait-image[src*="hunter_portrait_roster.png"]'),
      "hunter_portrait_roster.png",
      "Hero detail portrait"
    );

    // Cycle through each tab to verify reactive rendering
    // Use data-source-component for unique identification (Chinese labels
    // "战斗技能" and "扎营技能" both contain "技能")
    const tabComponents = [
      "EquipButton",
      "CombatSkillButton",
      "StateButton",
      "InfoButton",
      "CampingSkillButton",
    ];
    for (const comp of tabComponents) {
      const tabBtn = page.locator(`.hero-tab-btn[data-source-component="${comp}"]`);
      await expect(tabBtn, `Tab "${comp}" must be visible`).toBeVisible();
      await tabBtn.click();
      await settle(page, 200);
    }

    // Fidelity — hero detail is a completed product surface
    await expectFidelity(page.locator(".hero-detail-layout"), "Hero detail screen");
    await expectFullPageFidelity(page, "Hero detail screen");

    // Landscape viewport check for hero detail
    await expect(
      page.locator(".hero-detail-layout"),
      "Hero detail screen must use .hero-detail-layout landscape class"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Phase 3 (hero detail)");

    // ── Phase 4: Building detail ───────────────────────────
    // Return to town first
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    // Open the Guild building
    await page.locator('[data-building-id="guild"]').click();
    await settle(page, 800);

    await expect(
      page.locator(".building-detail-eyebrow"),
      "Building eyebrow must be visible"
    ).toHaveText("Building");
    // Building detail names render the source DDGC Chinese display names (UIR-005C).
    await expect(
      page.locator(".building-detail-name"),
      "Building name must be visible (DDGC display name 试炼场 = Guild)"
    ).toHaveText("试炼场");
    await expect(
      page.locator(".building-action-card-header").filter({ hasText: "Train Combat Skill" }),
      "Building action must be visible"
    ).toBeVisible();

    // Fidelity — building detail is a completed product surface
    await expectFidelity(page.locator(".app-frame"), "Building detail screen");
    await expectFullPageFidelity(page, "Building detail screen");

    // Landscape viewport check — building detail uses .app-frame as its landscape layout container
    await expect(
      page.locator(".app-frame"),
      "Building detail screen must use .app-frame landscape layout"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Phase 4 (building detail)");

    // ── Phase 5: Full meta-loop ─────────────────────────────
    // Town → Provisioning → Expedition → Result → Return → Town

    // 5a. Return to town
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    // 5b. Town → Provisioning
    await page.locator(".estate-embark-button").click();
    await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("Provisioning"),
      "Provisioning eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Provision Expedition"),
      "Provisioning title must be visible"
    ).toBeVisible();
    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Provisioning screen"
    );
    await expectFullPageFidelity(page, "Provisioning screen");

    // Landscape viewport check for provisioning
    await expect(
      page.locator(".expedition-viewport"),
      "Provisioning screen must use .expedition-viewport landscape layout"
    ).toBeVisible();

    // 5c. Provisioning → Expedition
    await page
      .getByRole("button", { name: "Confirm & Launch Expedition" })
      .click();
    await settle(page);

    await expect(
      page.getByRole("heading", { name: "Expedition Launch" }),
      "Expedition launch title must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Launch Expedition"),
      "Launch button must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".hud-pill").filter({ hasText: "Difficulty:" }),
      "Difficulty hud-pill must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".details-overlay-value").filter({ hasText: "Challenging" }),
      "Difficulty detail must be visible"
    ).toBeVisible();
    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Expedition launch screen"
    );
    await expectFullPageFidelity(page, "Expedition launch screen");

    // Landscape viewport check for expedition launch
    await expect(
      page.locator(".expedition-viewport"),
      "Expedition launch screen must use .expedition-viewport landscape layout"
    ).toBeVisible();

    // 5d. Expedition → Combat
    await page.getByRole("button", { name: "Launch Expedition" }).click();
    await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("Dungeon Combat"),
      "Combat eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "The Depths Await" }),
      "Combat expedition name must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".combat-unit--hero"),
      "Hero units must be visible"
    ).toHaveCount(2);
    await expect(
      page.locator(".combat-unit--enemy"),
      "Enemy units must be visible"
    ).toHaveCount(2);
    await expect(
      page.locator(".combat-map-node"),
      "Dungeon map nodes must be visible"
    ).toHaveCount(4);
    await expect(
      page.locator(".combat-skill-btn"),
      "Skill buttons must be visible"
    ).toHaveCount(4);
    await expectFidelity(
      page.locator(".combat-viewport"),
      "Combat screen"
    );
    await expectFullPageFidelity(page, "Combat screen");

    // Landscape viewport check for combat screen
    await expect(
      page.locator(".combat-viewport"),
      "Combat screen must use .combat-viewport landscape layout"
    ).toBeVisible();

    // 5e. Combat → Result (success)
    await page.getByRole("button", { name: "Finish Combat" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Complete" }),
      "Result screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Expedition Complete" }),
      "Result screen heading must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Victory"),
      "Victory outcome must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Ancient Gold Coin"),
      "Loot must be visible"
    ).toBeVisible();
    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Result screen"
    );
    await expectFullPageFidelity(page, "Result screen");

    // Landscape viewport check for result screen
    await expect(
      page.locator(".expedition-viewport"),
      "Result screen must use .expedition-viewport landscape layout"
    ).toBeVisible();

    // 5f. Result → Return
    await page.getByRole("button", { name: "Proceed to Return" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Concluded" }),
      "Return screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Returning to Town" }),
      "Return screen heading must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Resume Town Activities" }),
      "Resume button must be visible"
    ).toBeVisible();
    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Return screen"
    );
    await expectFullPageFidelity(page, "Return screen");

    // Landscape viewport check for return screen
    await expect(
      page.locator(".expedition-viewport"),
      "Return screen must use .expedition-viewport landscape layout"
    ).toBeVisible();

    // 5f. Return → Town (back to the meta-loop)
    await page.getByRole("button", { name: "Resume Town Activities" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("城镇中枢"),
      "Must be back at town (城镇中枢) after resume"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "苍灯远征" }),
      "Replay campaign name must persist after meta-loop"
    ).toBeVisible();
    await expect(
      page.locator(".estate-building-node"),
      "All 11 building nodes must re-render after meta-loop return"
    ).toHaveCount(11);
    await expect(
      page.locator(".estate-embark-button"),
      "Embark control must be present after meta-loop return"
    ).toBeVisible();

    // Landscape viewport check — town screen still valid after meta-loop
    await expect(
      page.locator(".town-viewport"),
      "Town must use .town-viewport landscape layout after meta-loop return"
    ).toBeVisible();

    // Final fidelity and error check for entire meta-loop
    await expectFidelity(page.locator(".town-viewport"), "Town after meta-loop");
    await expectFullPageFidelity(page, "Town after meta-loop");
    expectNoErrors(pageErrors, consoleErrors, "Phase 5 (meta-loop)");
  });

  // ── Live boot test ─────────────────────────────────────────
  test("live boot exercises live bridge path", async ({ page }) => {
    const { consoleErrors, pageErrors } = setupErrorCollectors(page);

    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await expect(
      page.getByRole("button", { name: "Boot Live" }),
      "Live boot button must be present"
    ).toBeVisible();

    await page.getByRole("button", { name: "Boot Live" }).click();

    // Wait for landscape town viewport (live bridge boots to town)
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // Verify live-specific content (Chinese labels match source DDGC town shell — UIR-005C).
    await expect(
      page.getByRole("heading", { name: "新档位面" }),
      "Live town campaign name (新档位面) must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("城镇中枢"),
      "Live town eyebrow (城镇中枢) must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".estate-building-node"),
      "Live town must render all 11 building nodes"
    ).toHaveCount(11);
    await expect(
      page.locator(".estate-side-button"),
      "Live town side buttons (3) must render"
    ).toHaveCount(3);
    await expect(
      page.locator(".estate-currency-slot"),
      "Live town currency strip (5 slots) must render"
    ).toHaveCount(5);

    // Fidelity — live town is a completed product surface
    await expectFidelity(page.locator(".town-viewport"), "Live town screen");
    await expectFullPageFidelity(page, "Live town screen");

    // Landscape viewport check for live town screen
    await expect(
      page.locator(".town-viewport"),
      "Live town screen must use .town-viewport landscape layout"
    ).toBeVisible();

    // Open hero detail from live bridge via the "英雄" utility button
    await page.getByRole("button", { name: "英雄", exact: true }).click();
    await page.waitForSelector(".hero-detail-layout", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("Hero Detail"),
      "Hero detail must be reachable from live boot"
    ).toBeVisible();

    // Fidelity check on live hero detail
    await expectFidelity(
      page.locator(".hero-detail-layout"),
      "Live hero detail screen"
    );
    await expectFullPageFidelity(page, "Live hero detail screen");

    // Landscape viewport check for live hero detail
    await expect(
      page.locator(".hero-detail-layout"),
      "Live hero detail screen must use .hero-detail-layout landscape class"
    ).toBeVisible();

    // Return and open a building
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-building-id="stagecoach"]').click();
    await settle(page, 800);

    await expect(
      page.locator(".building-detail-eyebrow"),
      "Building eyebrow must be visible"
    ).toHaveText("Building");
    // Building detail names render the source DDGC Chinese display names (UIR-005C).
    await expect(
      page.locator(".building-detail-name"),
      "Stagecoach building name must be visible (DDGC display name 次元感知塔 = Stagecoach)"
    ).toHaveText("次元感知塔");
    await expectFidelity(
      page.locator(".app-frame"),
      "Live building detail screen"
    );
    await expectFullPageFidelity(page, "Live building detail screen");

    // Landscape viewport check for live building detail
    await expect(
      page.locator(".app-frame"),
      "Live building detail screen must use .app-frame landscape layout"
    ).toBeVisible();

    // Full live flow: provisioning → expedition → result → return
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    // Launch expedition from live (button text may differ)
    await page.locator(".estate-embark-button").click();
    await page.waitForSelector(".expedition-viewport", { timeout: 5_000 });
    await settle(page);

    await page.getByRole("button", { name: "Confirm & Launch Expedition" }).click();
    await settle(page);

    await expect(
      page.getByRole("heading", { name: "Expedition Launch" }),
      "Live expedition launch title must be visible"
    ).toBeVisible();

    await page.getByRole("button", { name: "Launch Expedition" }).click();
    await page.waitForSelector(".combat-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.locator(".combat-viewport"),
      "Live combat screen must use .combat-viewport landscape layout"
    ).toBeVisible();
    await expect(
      page.locator(".combat-unit--hero"),
      "Live combat hero units must render"
    ).toHaveCount(2);
    await expect(
      page.locator(".combat-unit--enemy"),
      "Live combat enemy units must render"
    ).toHaveCount(2);

    await page.getByRole("button", { name: "Finish Combat" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Complete" }),
      "Live result screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Expedition Complete" }),
      "Live result screen heading must be visible"
    ).toBeVisible();

    await page.getByRole("button", { name: "Proceed to Return" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Concluded" }),
      "Live return screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Returning to Town" }),
      "Live return screen heading must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Resume Town Activities" }),
      "Resume button must be visible on live return screen"
    ).toBeVisible();

    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Live expedition screens"
    );
    await expectFullPageFidelity(page, "Live expedition screens");

    // Landscape viewport check for live expedition screens
    await expect(
      page.locator(".expedition-viewport"),
      "Live expedition screens must use .expedition-viewport landscape layout"
    ).toBeVisible();

    // Return to Town — complete the live meta-loop
    await page.getByRole("button", { name: "Resume Town Activities" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("城镇中枢"),
      "Must be back at town (城镇中枢) after resume in live path"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "新档位面" }),
      "Live campaign name must persist after live meta-loop"
    ).toBeVisible();
    await expect(
      page.locator(".estate-building-node"),
      "All building nodes must re-render after live meta-loop return"
    ).toHaveCount(11);
    await expect(
      page.locator(".estate-embark-button"),
      "Embark control must be present after live meta-loop return"
    ).toBeVisible();

    // Landscape viewport check — town screen still valid after live meta-loop
    await expect(
      page.locator(".town-viewport"),
      "Town must use .town-viewport landscape layout after live meta-loop"
    ).toBeVisible();

    // Final fidelity check after live meta-loop
    await expectFidelity(page.locator(".town-viewport"), "Town after live meta-loop");
    await expectFullPageFidelity(page, "Town after live meta-loop");

    expectNoErrors(pageErrors, consoleErrors, "Live boot flow");
  });
});
