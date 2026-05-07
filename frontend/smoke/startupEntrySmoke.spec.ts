/**
 * KUI-P1-006 — Phase 1 startup entry smoke.
 *
 * Focused browser smoke that exercises the kuajiyuanqi (跨纪元契约) startup
 * entry only — verifies the route, viewport, all four menu state surfaces
 * (main, save-select, settings, transition), captures one screenshot per
 * state, collects console + page-error stream, and asserts that no
 * forbidden placeholder / fallback / web-dashboard string is rendered to
 * the user (full-page innerText scan, not just attribute presence).
 *
 * Run with:
 *   npx playwright test smoke/startupEntrySmoke.spec.ts --reporter=list
 *
 * Run output:
 *   - PNG screenshots written to test-results/startup-phase1/*.png
 *   - JSON capture summary written to test-results/startup-phase1/capture.json
 *
 * This task only smokes the startup entry — it does not perform visual
 * parity judgment, does not modify product code, and does not assert any
 * downstream town/expedition state.
 */

import { test, expect, type Page, type ConsoleMessage } from "@playwright/test";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve as resolvePath } from "node:path";
import { fileURLToPath } from "node:url";

const BASE_URL = "http://localhost:4179";
const SPEC_DIR = dirname(fileURLToPath(import.meta.url));
const ARTIFACT_DIR = resolvePath(SPEC_DIR, "../test-results/startup-phase1");

/**
 * Forbidden user-visible surface text. The startup screen must not render
 * any of these patterns in its innerText — they signal placeholder / CSS
 * fallback / generic web-dashboard scaffolding rather than the staged
 * Unity startup composition.
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

async function settle(page: Page, ms = 250): Promise<void> {
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
  return { matched, sampledText: bodyText.slice(0, 1500) };
}

test.describe("KUI-P1-006: startup entry smoke", () => {
  test("startup route renders all four menu states without forbidden output", async ({
    page,
  }, testInfo) => {
    mkdirSync(ARTIFACT_DIR, { recursive: true });

    const { consoleAll, consoleErrors, pageErrors } =
      attachConsoleCollectors(page);

    const interactions: Array<{ step: string; action: string; ok: boolean }> = [];
    const stateRecords: Array<{
      state: string;
      screenshot: string;
      url: string;
      visibleAnchors: string[];
      forbiddenMatches: string[];
      sampledText: string;
    }> = [];

    // ── 1. Navigate to startup route ─────────────────────────────────
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
    interactions.push({
      step: "navigate",
      action: `page.goto(${BASE_URL})`,
      ok: true,
    });

    // Confirm the startup-screen container is mounted at root.
    await expect(
      page.locator(".startup-screen"),
      "Startup screen root container must mount"
    ).toBeVisible();

    // ── 2. State: main menu ──────────────────────────────────────────
    await expect(
      page.locator('[data-startup-panel="main"]'),
      "Main menu panel must mount on initial load"
    ).toBeVisible();

    const titleLocator = page.getByRole("heading", { name: "跨纪元契约" });
    await expect(
      titleLocator,
      "Title wordmark heading 跨纪元契约 must be visible"
    ).toBeVisible();

    const titleImage = page.locator(
      '.startup-title-wordmark[src*="/original/startup/game_logo.png"]'
    );
    await expect(
      titleImage,
      "Title wordmark must use staged Unity game_logo.png asset"
    ).toBeVisible();

    const mainAnchors = [
      "新游戏",
      "继续游戏 / 读取存档",
      "设置",
      "退出游戏",
      "Boot Replay",
      "Boot Live",
    ];
    for (const label of mainAnchors) {
      await expect(
        page.getByRole("button", { name: label }),
        `Main menu button "${label}" must be visible`
      ).toBeVisible();
    }

    const mainShot = `${ARTIFACT_DIR}/01-main.png`;
    await page.screenshot({ path: mainShot, fullPage: false });
    const mainResult = await assertNoForbiddenText(page, "state=main");
    stateRecords.push({
      state: "main",
      screenshot: "01-main.png",
      url: page.url(),
      visibleAnchors: mainAnchors.concat(["跨纪元契约 wordmark"]),
      forbiddenMatches: mainResult.matched,
      sampledText: mainResult.sampledText,
    });
    interactions.push({
      step: "main-state-render",
      action: "verify .startup-screen + main panel anchors",
      ok: true,
    });

    // ── 3. State: save-select ────────────────────────────────────────
    await page.getByRole("button", { name: "继续游戏 / 读取存档" }).click();
    await settle(page);
    interactions.push({
      step: "open-save-select",
      action: "click 继续游戏 / 读取存档",
      ok: true,
    });

    await expect(
      page.locator('[data-startup-panel="save-select"]'),
      "Save selection panel must mount"
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "存档选择" }),
      "存档选择 panel header must be visible"
    ).toBeVisible();
    const placeholderSlot = page.locator(
      '.startup-save-slot[data-slot-placeholder="true"]'
    );
    await expect(
      placeholderSlot,
      "Empty placeholder save slot row must mount"
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: "返回主菜单" }),
      "返回主菜单 (back to main) button must be visible"
    ).toBeVisible();

    const saveShot = `${ARTIFACT_DIR}/02-save-select.png`;
    await page.screenshot({ path: saveShot, fullPage: false });
    const saveResult = await assertNoForbiddenText(page, "state=save-select");
    stateRecords.push({
      state: "save-select",
      screenshot: "02-save-select.png",
      url: page.url(),
      visibleAnchors: ["存档选择", "点击开始新的冒险...", "返回主菜单"],
      forbiddenMatches: saveResult.matched,
      sampledText: saveResult.sampledText,
    });

    // Return to main before continuing.
    await page.getByRole("button", { name: "返回主菜单" }).click();
    await settle(page);
    interactions.push({
      step: "back-to-main",
      action: "click 返回主菜单",
      ok: true,
    });
    await expect(
      page.locator('[data-startup-panel="main"]'),
      "Main menu must be re-visible after closing save selection"
    ).toBeVisible();

    // ── 4. State: settings ───────────────────────────────────────────
    await page.getByRole("button", { name: "设置" }).click();
    await settle(page);
    interactions.push({
      step: "open-settings",
      action: "click 设置",
      ok: true,
    });

    await expect(
      page.locator('[data-startup-panel="settings"]'),
      "Settings panel must mount"
    ).toBeVisible();
    await expect(
      page.locator(".startup-settings-row"),
      "Seven settings rows (audio sliders + display selects) must render"
    ).toHaveCount(7);
    await expect(
      page.getByRole("button", { name: "关闭设置" }),
      "关闭设置 (close) button must be visible"
    ).toBeVisible();

    const settingsShot = `${ARTIFACT_DIR}/03-settings.png`;
    await page.screenshot({ path: settingsShot, fullPage: false });
    const settingsResult = await assertNoForbiddenText(
      page,
      "state=settings"
    );
    stateRecords.push({
      state: "settings",
      screenshot: "03-settings.png",
      url: page.url(),
      visibleAnchors: ["设置", "主音量", "音乐", "音效", "分辨率", "语言", "窗体化", "垂直同步", "关闭设置"],
      forbiddenMatches: settingsResult.matched,
      sampledText: settingsResult.sampledText,
    });

    // Close settings, back to main.
    await page.getByRole("button", { name: "关闭设置" }).click();
    await settle(page);
    interactions.push({
      step: "close-settings",
      action: "click 关闭设置",
      ok: true,
    });
    await expect(
      page.locator('[data-startup-panel="main"]'),
      "Main menu must be re-visible after closing settings"
    ).toBeVisible();

    // ── 5. State: transition (新游戏) ────────────────────────────────
    await page.getByRole("button", { name: "新游戏" }).click();
    await settle(page);
    interactions.push({
      step: "open-transition",
      action: "click 新游戏",
      ok: true,
    });

    await expect(
      page.locator('[data-startup-panel="transition"]'),
      "Region transition panel must mount"
    ).toBeVisible();
    await expect(
      page.getByText("朱雀大陆"),
      "Region title 朱雀大陆 must be visible"
    ).toBeVisible();
    await expect(
      page.getByText("按【空格】或【点击】继续游戏"),
      "Continue hint must be visible"
    ).toBeVisible();

    const transitionShot = `${ARTIFACT_DIR}/04-transition.png`;
    await page.screenshot({ path: transitionShot, fullPage: false });
    const transitionResult = await assertNoForbiddenText(
      page,
      "state=transition"
    );
    stateRecords.push({
      state: "transition",
      screenshot: "04-transition.png",
      url: page.url(),
      visibleAnchors: [
        "朱雀大陆",
        "绝望的炎熔，仿佛连太阳都能被禁尽",
        "按【空格】或【点击】继续游戏",
      ],
      forbiddenMatches: transitionResult.matched,
      sampledText: transitionResult.sampledText,
    });

    // ── 6. Finalize: error budget ────────────────────────────────────
    expect(
      pageErrors,
      "page errors must be empty across all four startup states"
    ).toEqual([]);
    expect(
      consoleErrors,
      "console errors must be empty across all four startup states"
    ).toEqual([]);

    // ── 7. Capture summary JSON for the artifact ─────────────────────
    const viewport = page.viewportSize();
    const summary = {
      task: "KUI-P1-006",
      route: BASE_URL,
      viewport,
      finalUrl: page.url(),
      states: stateRecords,
      interactions,
      console: {
        totalMessages: consoleAll.length,
        errorCount: consoleErrors.length,
        pageErrorCount: pageErrors.length,
        all: consoleAll,
      },
      forbiddenPatterns: FORBIDDEN_TEXT_PATTERNS.map((p) => String(p)),
      assetsObserved: {
        sceneBackground: "/original/startup/scene_bg.png",
        titleWordmark: "/original/startup/game_logo.png",
        parchmentDialog: "/original/startup/dialog07.png",
        saveFrameDialog: "/original/startup/dialog02.png",
        titleBg: "/original/startup/save_title_bg.png",
        menuButtonSprite: "/original/startup/btn_red.png",
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
