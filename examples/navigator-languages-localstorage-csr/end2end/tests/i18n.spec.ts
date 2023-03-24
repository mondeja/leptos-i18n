import { test, expect } from "@playwright/test";

test("homepage in browser language at first load", async ({ page }) => {
  await page.goto("http://localhost:8080/");
  await page.evaluate(() => window.localStorage.clear());
  // Reload here because we need to access the page to clear localStorage
  await page.reload();

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

  const languageCode = await page.evaluate(() =>
    window.localStorage.getItem("language"),
  );
  expect(languageCode).toBe("en");

  // after realoding, the language must be the same
  await page.reload();

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
  await expect(page.locator("p")).toHaveText("Language: English");
  await expect(page.locator("option[selected]")).toHaveText("English");
});
