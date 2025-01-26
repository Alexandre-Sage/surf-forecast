import { UserPayload } from "../../src/types/user.type";
import { faker } from "@faker-js/faker";

export const fakeUser = (_?: Partial<UserPayload>): UserPayload => {
  const password = faker.internet.password();
  return {
    userName: _?.userName ?? faker.internet.username(),
    firstName: _?.firstName ?? faker.person.firstName(),
    lastName: _?.lastName ?? faker.person.lastName(),
    email: _?.email ?? faker.internet.email(),
    password: _?.password ?? password,
    confirmPassword: _?.confirmPassword ?? password,
  };
};
