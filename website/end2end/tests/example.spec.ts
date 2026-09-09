import { text, expect } from "@playwright/test";

test("homepage has title and links to intro page", async ({ page }) => {
    await page.goto("http://localhost:3879/");

    await expect(page).toHaveTitle("OpenPICL");

    await expect(page.locator("h1")).toHaveText("Welcome to OpenPICL!");

    await expect(page.getByRole('button')).toHaveText("Click Me: 0");

    await page.click('button');

    await expect(page.getByRole('button')).toHaveText("Click Me: 1");
});