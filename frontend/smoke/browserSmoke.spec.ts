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
 * Assert the text content of an element does not contain any blocklisted
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
    await expect(
      page.getByText("Estate"),
      "Estate label must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("The Azure Lantern"),
      "Campaign name must be visible"
    ).toBeVisible();
    await expect(page.getByText("1250"), "Gold amount must be visible").toBeVisible();

    // Roster heroes
    for (const name of ["Shen", "Bai Xiu", "Hei Zhen"]) {
      await expect(
        page.locator(".roster-scroll").getByText(name),
        `Hero "${name}" must appear in roster`
      ).toBeVisible();
    }

    // Buildings
    for (const label of ["Stagecoach", "Guild", "Blacksmith", "Sanitarium"]) {
      await expect(
        page.getByText(label),
        `Building "${label}" must be visible`
      ).toBeVisible();
    }

    await expectOriginalAssetImage(
      page.locator('.building-icon-image[src*="/original/buildings/"]').first(),
      "/original/buildings/",
      "Town building marker"
    );
    await expectOriginalAssetImage(
      page.locator('.roster-portrait-image[src*="/original/heroes/"]').first(),
      "/original/heroes/",
      "Town roster portrait"
    );

    // Fidelity — town is a completed product surface
    await expectFidelity(page.locator(".town-viewport"), "Town screen");

    // Landscape viewport check
    await expect(
      page.locator(".town-viewport"),
      "Town must use .town-viewport for landscape layout"
    ).toBeVisible();
    await expect(
      page.locator(".viewport-hud"),
      "Town must have a .viewport-hud bar"
    ).toBeVisible();
    await expect(
      page.locator(".viewport-roster"),
      "Town must have a .viewport-roster bar"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Phase 2 (town)");

    // ── Phase 3: Hero detail ───────────────────────────────
    // Click first hero in the roster (Shen)
    await page.locator(".roster-hero").first().click();
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
    const tabs = ["装备", "技能", "信息", "状态"];
    for (const tabLabel of tabs) {
      const tabBtn = page.locator(".hero-tab-btn").filter({ hasText: tabLabel });
      await expect(tabBtn, `Tab "${tabLabel}" must be visible`).toBeVisible();
      await tabBtn.click();
      await settle(page, 200);
    }

    // Fidelity — hero detail is a completed product surface
    await expectFidelity(page.locator(".hero-detail-layout"), "Hero detail screen");
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
      page.getByText("Building — Guild"),
      "Guild building eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Guild").first(),
      "Building label must be visible"
    ).toBeVisible();
    await expect(
      page.locator("strong").filter({ hasText: "Train Combat Skill" }),
      "Building action must be visible"
    ).toBeVisible();

    // Fidelity — building detail is a completed product surface
    await expectFidelity(page.locator("main.app-frame"), "Building detail screen");
    expectNoErrors(pageErrors, consoleErrors, "Phase 4 (building detail)");

    // ── Phase 5: Full meta-loop ─────────────────────────────
    // Town → Provisioning → Expedition → Result → Return → Town

    // 5a. Return to town
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    // 5b. Town → Provisioning
    await page.getByRole("button", { name: "Provision Expedition" }).click();
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

    // 5d. Expedition → Result (success)
    await page.getByRole("button", { name: "Launch Expedition" }).click();
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

    // 5e. Result → Return
    await page.getByRole("button", { name: "Continue to Town" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Concluded" }),
      "Return screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Expedition Concluded" }),
      "Return screen heading must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("Resume Town Activities"),
      "Resume button must be visible"
    ).toBeVisible();
    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Return screen"
    );

    // Landscape viewport check for expedition-family screens
    await expect(
      page.locator(".expedition-viewport"),
      "Expedition screens must use .expedition-viewport"
    ).toBeVisible();

    // 5f. Return → Town (back to the meta-loop)
    await page.getByRole("button", { name: "Resume Town Activities" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await expect(
      page.getByText("Estate"),
      "Must be back at town after resume"
    ).toBeVisible();
    await expect(
      page.getByText("The Azure Lantern"),
      "Campaign name must persist after loop"
    ).toBeVisible();

    // Final fidelity and error check for entire loop
    await expectFidelity(page.locator(".town-viewport"), "Town after meta-loop");
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

    // Verify live-specific content
    await expect(
      page.getByRole("heading", { name: "Fresh Campaign" }),
      "Live town campaign name must be visible"
    ).toBeVisible();

    // Live fixture heroes
    for (const name of ["Yuan", "Mei"]) {
      await expect(
        page.locator(".roster-scroll").getByText(name),
        `Live hero "${name}" must appear in roster`
      ).toBeVisible();
    }

    // Open hero detail from live bridge
    await page.locator(".roster-hero").first().click();
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

    // Return and open a building
    await page.getByRole("button", { name: "Return to Town" }).click();
    await page.waitForSelector(".town-viewport", { timeout: 5_000 });
    await settle(page);

    await page.locator('[data-building-id="stagecoach"]').click();
    await settle(page, 800);

    await expect(
      page.getByText("Building — Stagecoach"),
      "Stagecoach building eyebrow must be visible"
    ).toBeVisible();
    await expectFidelity(
      page.locator("main.app-frame"),
      "Live building detail screen"
    );

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
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Complete" }),
      "Live result screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Expedition Complete" }),
      "Live result screen heading must be visible"
    ).toBeVisible();

    await page.getByRole("button", { name: "Continue to Town" }).click();
    await settle(page);

    await expect(
      page.locator(".eyebrow").filter({ hasText: "Expedition Concluded" }),
      "Live return screen eyebrow must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Expedition Concluded" }),
      "Live return screen heading must be visible"
    ).toBeVisible();

    await expectFidelity(
      page.locator(".expedition-viewport"),
      "Live expedition screens"
    );

    // Landscape viewport check for live expedition screens
    await expect(
      page.locator(".expedition-viewport"),
      "Live expedition screens must use .expedition-viewport"
    ).toBeVisible();

    expectNoErrors(pageErrors, consoleErrors, "Live boot flow");
  });
});
