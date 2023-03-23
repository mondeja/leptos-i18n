import { test, expect } from "@playwright/test";

test("homepage in default language", async ({ page }) => {
  await page.goto("http://localhost:8080/");

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
  await expect(page.locator("p")).toHaveText("Language: English");
  await expect(page.locator("option[selected]")).toHaveText("English");
});

test("change language", async ({ page }) => {
  await page.goto("http://localhost:8080/");
  await page.selectOption("select", "Español");

  await expect(page.locator("h1")).toHaveText("¡Bienvenido a Leptos!");
  await expect(page.locator("p")).toHaveText("Idioma: Español");
  await expect(page.locator("option[selected]")).toHaveText("Español");

  // after realoding, the language is reset to default
  await page.reload();

  await expect(page.locator("h1")).toHaveText("Welcome to Leptos!");
  await expect(page.locator("p")).toHaveText("Language: English");
  await expect(page.locator("option[selected]")).toHaveText("English");
});
