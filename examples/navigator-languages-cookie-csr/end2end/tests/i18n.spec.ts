import { test, expect } from "@playwright/test";

test("homepage in browser language at first load", async ({ browser }) => {
  const context = await browser.newContext();
  context.clearCookies();
  const page = await context.newPage();
  await page.goto("http://localhost:8080/");

  await expect(page.locator("h1")).toHaveText("¡Bienvenido a Leptos!");
  await expect(page.locator("p")).toHaveText("Idioma: Español");
  await expect(page.locator("option[selected]")).toHaveText("Español");
});

test("change language", async ({ page }) => {
  await page.goto("http://localhost:8080/");
  await page.selectOption("select", "English");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
  await expect(page.locator("p")).toHaveText("Language: English");
  await expect(page.locator("option[selected]")).toHaveText("English");

  const cookies = await page.context().cookies();
  const firstCookie = cookies[0];
  expect(firstCookie.name).toBe("language");
  expect(firstCookie.value).toBe("en");
  expect(firstCookie.secure).toBe(true);
  expect(firstCookie.path).toBe("/");

  // after realoding, the language must be the same
  await page.reload();

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
  await expect(page.locator("p")).toHaveText("Language: English");
  await expect(page.locator("option[selected]")).toHaveText("English");
});
