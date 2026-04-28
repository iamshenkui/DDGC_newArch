/**
 * Package smoke tests — validate the rendered frontend build output
 * after the local build/package step (P10-US-009-d).
 *
 * Run with: npm run smoke-build
 * Requires: npm run build (run first, producing dist/)
 *
 * These tests validate:
 * 1. The dist/ directory exists with expected structure
 * 2. index.html references valid JS and CSS bundle files
 * 3. The HTML entry point has correct structural elements
 * 4. All referenced assets exist and contain content
 */

import { describe, expect, it } from "vitest";
import { existsSync, readFileSync, readdirSync, statSync } from "fs";
import { resolve } from "path";

const DIST_DIR = resolve(__dirname, "../../dist");
const INDEX_HTML = resolve(DIST_DIR, "index.html");
const ASSETS_DIR = resolve(DIST_DIR, "assets");

describe("package smoke: build output exists", () => {
  it("dist directory exists after build", () => {
    expect(existsSync(DIST_DIR)).toBe(true);
  });

  it("index.html is present in dist", () => {
    expect(existsSync(INDEX_HTML)).toBe(true);
  });

  it("assets directory is present in dist", () => {
    expect(existsSync(ASSETS_DIR)).toBe(true);
  });
});

describe("package smoke: index.html references valid bundles", () => {
  it("references a JS bundle file that exists and is non-empty", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    const match = html.match(/src="\/(assets\/index-[^.]+\.js)"/);
    expect(match, "index.html must reference a JS bundle with <script src=...>").not.toBeNull();
    const jsPath = resolve(DIST_DIR, match![1]);
    expect(existsSync(jsPath), `referenced JS bundle ${match![1]} must exist`).toBe(true);
    expect(statSync(jsPath).size).toBeGreaterThan(0);
  });

  it("references a CSS bundle file that exists and is non-empty", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    const match = html.match(/href="\/(assets\/index-[^.]+\.css)"/);
    expect(match, "index.html must reference a CSS bundle with <link href=...>").not.toBeNull();
    const cssPath = resolve(DIST_DIR, match![1]);
    expect(existsSync(cssPath), `referenced CSS bundle ${match![1]} must exist`).toBe(true);
    expect(statSync(cssPath).size).toBeGreaterThan(0);
  });

  it("all referenced asset files resolve correctly", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    const jsRefs = html.match(/src="\/([^"]+)"/g) || [];
    const cssRefs = html.match(/href="\/([^"]+)"/g) || [];
    const refs = [...jsRefs, ...cssRefs]
      .map((r) => {
        const m = r.match(/"\/?(assets\/[^"]+)"/);
        return m ? m[1] : null;
      })
      .filter(Boolean);

    expect(refs.length).toBeGreaterThan(0);
    for (const ref of refs) {
      const fullPath = resolve(DIST_DIR, ref!);
      expect(existsSync(fullPath), `referenced asset "${ref}" must exist`).toBe(true);
      expect(statSync(fullPath).size, `referenced asset "${ref}" must not be empty`).toBeGreaterThan(0);
    }
  });
});

describe("package smoke: index.html structure", () => {
  it("has a root mount point element", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    expect(html).toContain('<div id="root">');
  });

  it("has a charset meta tag", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    expect(html).toContain('charset="UTF-8"');
  });

  it("has a viewport meta tag", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    expect(html).toContain('name="viewport"');
  });

  it("uses module script type", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    expect(html).toContain('type="module"');
  });

  it("has a page title", () => {
    const html = readFileSync(INDEX_HTML, "utf-8");
    expect(html).toContain("<title>");
    expect(html).toContain("</title>");
    const titleMatch = html.match(/<title>([^<]+)<\/title>/);
    expect(titleMatch).not.toBeNull();
    expect(titleMatch![1].trim().length).toBeGreaterThan(0);
  });
});

describe("package smoke: assets directory integrity", () => {
  it("contains at least one file", () => {
    const files = readdirSync(ASSETS_DIR);
    expect(files.length).toBeGreaterThan(0);
  });

  it("contains a .js bundle file", () => {
    const files = readdirSync(ASSETS_DIR);
    const jsFiles = files.filter((f) => f.endsWith(".js"));
    expect(jsFiles.length).toBeGreaterThan(0);
  });

  it("contains a .css bundle file", () => {
    const files = readdirSync(ASSETS_DIR);
    const cssFiles = files.filter((f) => f.endsWith(".css"));
    expect(cssFiles.length).toBeGreaterThan(0);
  });
});
