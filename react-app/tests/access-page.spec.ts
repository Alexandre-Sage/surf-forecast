import { test, expect, firefox, Page } from "@playwright/test";
import { UserPayload } from "../src/types/user.type";
import { fakeUser } from "./fixtures/user";

export const signUpTest = async (page: Page, data: UserPayload) => {
  await page.getByRole("button", { name: "Sign up" }).click();
  await page.getByLabel("Pseudo").fill(data.userName);
  await page.getByLabel("Email").fill(data.email);
  await page.getByLabel("Prénom").fill(data.firstName);
  await page.getByLabel("Nom", { exact: true }).fill(data.lastName);
  await page.getByLabel("Mot de passe", { exact: true }).fill(data.password);
  await page.getByLabel("Confirmation mot de passe").fill(data.confirmPassword);
};

test("Should fill sign up form and post data successfully", async ({
  page,
}) => {
  await page.goto("http://localhost:5173/");
  await signUpTest(page, fakeUser());
  await page.route("**/users", async (_) => {
    return _.fulfill({ body: JSON.stringify({}) });
  });
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByText("Utilisateur créer")).toBeVisible();
});

test("Should fill sign up form and post data with password mismatch error", async ({
  page,
}) => {
  await page.goto("http://localhost:5173/");
  await signUpTest(page, fakeUser({ confirmPassword: "xyz" }));
  await page.route("**/users", async (_) => {
    return _.fulfill({
      body: JSON.stringify({ error: "PASSWORD_MISMATCH" }),
      status: 400,
    });
  });
  await page.getByRole("button", { name: "Save" }).click();
  await expect(
    page.getByText(
      "Les mots de passe ne correspondent pas. Veuillez vous assurer que les deux mots de passe sont identiques.",
    ),
  ).toBeVisible();
});

test("Should fill sign up form and post data with password mismatch length", async ({
  page,
}) => {
  await page.goto("http://localhost:5173/");
  await signUpTest(page, fakeUser({ password: "xyz", confirmPassword: "xyz" }));
  await page.route("**/users", async (_) => {
    return _.fulfill({
      body: JSON.stringify({ error: "PASSWORD_LENGTH" }),
      status: 400,
    });
  });
  await page.getByRole("button", { name: "Save" }).click();
  await expect(
    page.getByText(
      "Votre mot de passe est trop court. Veuillez choisir un mot de passe d'au moins 8 caractères.",
    ),
  ).toBeVisible();
});

test("Should fill sign up form and post data with username taken", async ({
  page,
}) => {
  await page.goto("http://localhost:5173/");
  await signUpTest(page, fakeUser());
  await page.route("**/users", async (_) => {
    return _.fulfill({
      body: JSON.stringify({ error: "USERNAME_TAKEN" }),
      status: 400,
    });
  });
  await page.getByRole("button", { name: "Save" }).click();
  await expect(
    page.getByText(
      "Ce nom d'utilisateur est déjà pris. Veuillez en choisir un autre.",
    ),
  ).toBeVisible();
});

test("Should fill sign up form and post data with email exist", async ({
  page,
}) => {
  await page.goto("http://localhost:5173/");
  await signUpTest(page, fakeUser());
  await page.route("**/users", async (_) => {
    return _.fulfill({
      body: JSON.stringify({ error: "EMAIL_EXIST" }),
      status: 400,
    });
  });
  await page.getByRole("button", { name: "Save" }).click();
  await expect(
    page.getByText(
      "Cet email est déjà associé à un compte. Veuillez utiliser un autre email.",
    ),
  ).toBeVisible();
});
