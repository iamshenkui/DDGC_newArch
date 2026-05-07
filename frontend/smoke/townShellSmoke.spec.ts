/**
 * KUI-P1-011 — Phase 1 town shell smoke.
 *
 * Focused browser smoke that exercises the kuajiyuanqi (跨纪元契约) town
 * shell only — verifies the route, viewport, town chrome (nameplate,
 * currency strip, side commands, embark control), building collection,
 * asserts that forbidden quick buttons (QuickStart / QuickProgress) are
 * absent, captures a screenshot, collects console + page-error stream,
 * and asserts that no forbidden placeholder / fallback / web-dashboard
 * string is rendered to the user (full-page innerText scan).
 *
 * Run with:
 *   npx playwright test smoke/townShellSmoke.spec.ts --reporter=list
 *
 * Run output:
 *   - PNG screenshot written to test-results/town-shell-phase1/town-shell.png
 *   - JSON capture summary written to test-results/town-shell-phase1/capture.json
 *
 * This task only smokes the town shell — it does not perform visual
 * parity judgment, does not modify product code, and does not assert any
 * downstream expedition/combat state.
 */

import { test, expect, type Page, type ConsoleMessage } from "@playwright/test";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve as resolvePath } from "node:path";
import { fileURLToPath } from "node:url";

const BASE_URL = "http://localhost:4179";
const SPEC_DIR = dirname(fileURLToPath(import.meta.url));
const ARTIFACT_DIR = resolvePath(SPEC_DIR, "../test-results/town-shell-phase1");

/**
 * Forbidden user-visible surface text. The town shell must not render
 * any of these patterns in its innerText.
 */
const FORBIDDEN_TEXT_PATTERNS: ReadonlyArray<RegExp> = [
  /placeholder/i,
  /fallback/i,
  /web.?dashboard/i,
  /skeletal/i,
  /skeleton/i,
  /reserved canvas/i,
  /text.based rendering/i,
  /rendering completion/i,
  /coming soon/i,
  /lorem ipsum/i,
  /TODO/,
  /FIXME/,
  /XXX:/
] as const;

interface ConsoleRecord {
  type: string;
  text: string;
}

function attachConsoleCollectors(page: Page): {
  consoleAll: ConsoleRecord[];
  consoleErrors: string[];
  pageErrors: string[];
} {
  const consoleAll: ConsoleRecord[] = [];
  const consoleErrors: string[] = [];
  const pageErrors: string[] = [];

  page.on("console", (msg: ConsoleMessage) => {
    consoleAll.push({ type: msg.type(), text: msg.text() });
    if (msg.type() === "error") {
      consoleErrors.push(`[console.error] ${msg.text()}`);
    }
  });
  page.on("pageerror", (err) => {
    pageErrors.push(`[page.error] ${err.message}`);
  });

  return { consoleAll, consoleErrors, pageErrors };
}

async function settle(page: Page, ms = 400): Promise<void> {
  await page.waitForTimeout(ms);
}

async function assertNoForbiddenText(
  page: Page,
  stateLabel: string
): Promise<{ matched: string[]; sampledText: string }> {
  const bodyText = await page.locator("body").innerText();
  const matched: string[] = [];
  for (const pattern of FORBIDDEN_TEXT_PATTERNS) {
    if (pattern.test(bodyText)) {
      matched.push(String(pattern));
    }
    expect(
      bodyText,
      `${stateLabel}: user-visible text must not match forbidden pattern ${pattern}`
    ).not.toMatch(pattern);
  }
  return { matched, sampledText: bodyText.slice(0, 2000) };
}

test.describe("KUI-P1-011: town shell smoke", () => {
  test("town shell renders with all chrome, commands, and no forbidden output", async ({
    page,
  }, testInfo) => {
    mkdirSync(ARTIFACT_DIR, { recursive: true });

    const { consoleAll, consoleErrors, pageErrors } =
      attachConsoleCollectors(page);

    const interactions: Array<{ step: string; action: string; ok: boolean }> = [];

    // ── 1. Navigate to startup and boot replay ───────────────────────
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
    interactions.push({
      step: "navigate",
      action: `page.goto(${BASE_URL})`,
      ok: true,
    });

    await expect(
      page.getByRole("button", { name: "Boot Replay" }),
      "Boot Replay button must be present"
    ).toBeVisible();

    await page.getByRole("button", { name: "Boot Replay" }).click();
    interactions.push({
      step: "boot-replay",
      action: "click Boot Replay",
      ok: true,
    });

    // Wait for landscape town viewport to mount
    await page.waitForSelector(".town-viewport", { timeout: 8_000 });
    await settle(page);

    // ── 2. Viewport and route verification ───────────────────────────
    const viewport = page.viewportSize();
    expect(viewport).toEqual({ width: 1440, height: 900 });
    const currentUrl = page.url();
    expect(currentUrl).toBe(BASE_URL + "/");

    // ── 3. Town shell root ───────────────────────────────────────────
    await expect(
      page.locator(".town-viewport"),
      "Town viewport root must mount"
    ).toBeVisible();
    interactions.push({
      step: "town-viewport-mount",
      action: "waitForSelector .town-viewport",
      ok: true,
    });

    // ── 4. Top chrome — nameplate ────────────────────────────────────
    await expect(
      page.locator(".estate-top-panel"),
      "Top nameplate panel must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("城镇中枢"),
      "Estate eyebrow (城镇中枢) must be visible"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "苍灯远征" }),
      "Replay campaign name (苍灯远征) must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("回放快照：当前战役处于城镇整备阶段，可查看名册、建筑与远征准备。"),
      "Campaign summary must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("本周初访"),
      "Fresh visit badge (本周初访) must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("存档9"),
      "Save slot tag (存档9) must be visible"
    ).toBeVisible();

    // ── 5. Currency strip — 5 resource icons ─────────────────────────
    await expect(
      page.locator(".estate-currency-strip"),
      "Currency strip must be visible"
    ).toBeVisible();
    await expect(
      page.locator(".estate-currency-slot"),
      "Currency strip must contain 5 slots"
    ).toHaveCount(5);

    // Verify each currency slot has the correct original asset icon
    const currencySlots = page.locator(".estate-currency-slot");
    const expectedCurrencyIcons = [
      { alt: "", srcFragment: "/original/chrome/bust.png", label: "bust" },
      { alt: "", srcFragment: "/original/chrome/portrait.png", label: "portrait" },
      { alt: "", srcFragment: "/original/chrome/deed.png", label: "deed" },
      { alt: "", srcFragment: "/original/chrome/crest.png", label: "crest" },
      { alt: "", srcFragment: "/original/chrome/gold.png", label: "gold" },
    ];
    for (let i = 0; i < expectedCurrencyIcons.length; i++) {
      const slot = currencySlots.nth(i);
      const icon = slot.locator("img.estate-currency-icon");
      await expect(
        icon,
        `Currency icon ${expectedCurrencyIcons[i].label} must be visible`
      ).toBeVisible();
      await expect(
        icon,
        `Currency icon ${expectedCurrencyIcons[i].label} must use staged original asset`
      ).toHaveAttribute("src", new RegExp(expectedCurrencyIcons[i].srcFragment.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    }

    // Verify currency values
    await expect(page.getByText("100").first(), "Bust amount (100) must be visible").toBeVisible();
    await expect(page.getByText("1250"), "Gold amount (1250) must be visible").toBeVisible();

    // ── 6. Side commands — 3 active, 3 inactive hidden ───────────────
    await expect(
      page.locator(".estate-side-button"),
      "Exactly 3 active side buttons must render"
    ).toHaveCount(3);

    const activeSideButtonLabels = ["饰品仓库", "英雄", "设置"];
    for (const label of activeSideButtonLabels) {
      await expect(
        page.getByRole("button", { name: label, exact: true }),
        `Side command button "${label}" must be visible`
      ).toBeVisible();
    }

    // Inactive side buttons must be hidden from the accessibility tree
    const inactiveLabels = ["ActivityLogButton", "TownEventButton", "GlossaryButton"];
    for (const label of inactiveLabels) {
      await expect(
        page.locator(`button[data-source-prefab*="${label}"]`),
        `Inactive side button "${label}" must be hidden`
      ).toBeHidden();
    }

    // ── 7. Embark command ────────────────────────────────────────────
    await expect(
      page.locator(".estate-embark-button"),
      "Embark button must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("位面探索"),
      "Embark title (位面探索) must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("整备远征"),
      "Embark subtitle (整备远征) must be visible"
    ).toBeVisible();

    // ── 8. Building collection — 11 nodes ────────────────────────────
    await expect(
      page.locator(".estate-building-node"),
      "All 11 building nodes must render"
    ).toHaveCount(11);

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

    // Verify at least one building icon uses the staged original asset path
    await expect(
      page.locator('.building-icon-image[src*="/original/buildings/"]').first(),
      "Building icon must use staged original asset"
    ).toBeVisible();

    // ── 9. Forbidden quick buttons assertion ─────────────────────────
    await expect(
      page.locator(".estate-quick-button"),
      "QuickStart / QuickProgress buttons must not exist in the DOM"
    ).toHaveCount(0);

    interactions.push({
      step: "verify-chrome",
      action: "assert nameplate, currency, side buttons, embark, buildings, no quick buttons",
      ok: true,
    });

    // ── 10. Screenshot capture ───────────────────────────────────────
    const screenshotPath = `${ARTIFACT_DIR}/town-shell.png`;
    await page.screenshot({ path: screenshotPath, fullPage: false });
    interactions.push({
      step: "screenshot",
      action: `screenshot → ${screenshotPath}`,
      ok: true,
    });

    // ── 11. Forbidden text scan ──────────────────────────────────────
    const forbiddenResult = await assertNoForbiddenText(page, "town-shell");

    // ── 12. Error budget ─────────────────────────────────────────────
    expect(
      pageErrors,
      "page errors must be empty across town shell smoke"
    ).toEqual([]);
    expect(
      consoleErrors,
      "console errors must be empty across town shell smoke"
    ).toEqual([]);

    // ── 13. Capture summary JSON ─────────────────────────────────────
    const summary = {
      task: "KUI-P1-011",
      route: BASE_URL,
      viewport,
      finalUrl: currentUrl,
      interactions,
      anchors: {
        townViewport: ".town-viewport",
        topPanel: ".estate-top-panel",
        eyebrow: "城镇中枢",
        campaignName: "苍灯远征",
        campaignSummary: "回放快照：当前战役处于城镇整备阶段，可查看名册、建筑与远征准备。",
        freshVisitBadge: "本周初访",
        saveSlotTag: "存档9",
        currencyStrip: ".estate-currency-strip",
        currencySlots: 5,
        currencyValues: { bust: 100, portrait: 100, deed: 100, crest: 200, gold: 1250 },
        sideButtonsActive: activeSideButtonLabels,
        sideButtonsInactive: inactiveLabels,
        embarkButton: ".estate-embark-button",
        embarkTitle: "位面探索",
        embarkSubtitle: "整备远征",
        buildingNodes: 11,
        buildingLabels: expectedBuildings,
        forbiddenQuickButtons: "absent (count=0)",
      },
      screenshot: "town-shell.png",
      forbiddenScan: {
        patterns: FORBIDDEN_TEXT_PATTERNS.map((p) => String(p)),
        matched: forbiddenResult.matched,
        sampledText: forbiddenResult.sampledText,
      },
      console: {
        totalMessages: consoleAll.length,
        errorCount: consoleErrors.length,
        pageErrorCount: pageErrors.length,
        all: consoleAll,
      },
      assetsObserved: {
        estateNameBg: "/original/chrome/save_name_bg.png",
        bustIcon: "/original/chrome/bust.png",
        portraitIcon: "/original/chrome/portrait.png",
        deedIcon: "/original/chrome/deed.png",
        crestIcon: "/original/chrome/crest.png",
        goldIcon: "/original/chrome/gold.png",
        embarkButton: "/original/chrome/btn_play.png",
        sideButtonSprite: "/original/chrome/btn_white.png",
        buildingLabelBg: "/original/chrome/building_label_bg01.png",
        buildingIconBg: "/original/chrome/building_icon_bg.png",
      },
      generatedAt: new Date().toISOString(),
      testInfo: {
        title: testInfo.title,
        project: testInfo.project.name,
      },
    };
    writeFileSync(
      `${ARTIFACT_DIR}/capture.json`,
      JSON.stringify(summary, null, 2),
      "utf-8"
    );
  });
});
