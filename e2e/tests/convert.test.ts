import { afterAll, beforeAll, expect, test } from "bun:test";
import { type Browser, chromium } from "playwright";

// @playwright/test runner exits silently under bun, so drive the library directly
const BASE_URL = "http://127.0.0.1:3000";

// 1x1 PNG, inline so no binary fixture in repo
const PNG_1X1 = Buffer.from(
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==",
  "base64",
);

let server: Bun.Subprocess | undefined;
let browser: Browser;

async function isUp(): Promise<boolean> {
  try {
    return (await fetch(BASE_URL)).ok;
  } catch {
    return false;
  }
}

beforeAll(async () => {
  if (!(await isUp())) {
    server = Bun.spawn(["trunk", "serve", "--port", "3000"], {
      cwd: `${import.meta.dir}/../..`,
      stdout: "ignore",
      stderr: "inherit",
    });
    // first wasm build is slow
    const deadline = Date.now() + 300_000;
    while (!(await isUp())) {
      if (Date.now() > deadline) throw new Error("trunk serve did not start");
      await Bun.sleep(500);
    }
  }
  browser = await chromium.launch();
}, 310_000);

afterAll(async () => {
  await browser?.close();
  server?.kill();
});

test("converts PNG to WebP and downloads it", async () => {
  const page = await browser.newPage();
  await page.goto(BASE_URL);

  await page.locator('input[type="file"]').setInputFiles({
    name: "photo.png",
    mimeType: "image/png",
    buffer: PNG_1X1,
  });

  await page.getByRole("button", { name: "WebP", exact: true }).click();

  // enabled only after async file read finishes
  const convert = page.locator("button:not([disabled])", { hasText: "Convert Image" });
  await convert.waitFor();

  const [download] = await Promise.all([page.waitForEvent("download"), convert.click()]);
  expect(download.suggestedFilename()).toBe("photo.webp");
  expect(await page.getByRole("alert").count()).toBe(0);
});
