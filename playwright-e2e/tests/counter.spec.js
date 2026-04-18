const { test, expect } = require("@playwright/test");

test("renders the wasm app shell", async ({ page }) => {
  await page.goto("/");

  await expect(page.locator('[data-cy="demo-title"]')).toHaveText(
    "rust-fel Playwright demo",
  );
  await expect(page.locator('[data-cy="count-value"]')).toHaveText("0");
  await expect(page.locator('[data-cy="increment"]')).toHaveText("Increment");
  await expect(page.locator('[data-cy="decrement"]')).toHaveText("Decrement");
});

test("updates real DOM text when buttons are clicked", async ({ page }) => {
  await page.goto("/");

  const value = page.locator('[data-cy="count-value"]');

  await page.locator('[data-cy="increment"]').click();
  await expect(value).toHaveText("1");

  await page.locator('[data-cy="increment"]').click();
  await expect(value).toHaveText("2");

  await page.locator('[data-cy="decrement"]').click();
  await expect(value).toHaveText("1");
});
