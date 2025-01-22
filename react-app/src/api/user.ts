import { useMutation } from "@tanstack/react-query";
import { UserPayload } from "../types/user.type";

const baseUrl = "http://localhost:8080";

export const saveUser = (user: UserPayload) =>
  fetch("http://localhost:8080/users", {
    method: "POST",
    body: JSON.stringify(user),
    headers: {
      "content-type": "application/json",
    },
  });

export const authenticateUser = (credentials: {
  password: string;
  email: string;
}) =>
  fetch(`${baseUrl}/users/authenticate`, {
    method: "POST",
    body: JSON.stringify(credentials),
    headers: {
      "content-type": "application/json",
    },
  });
export const useSaveUser = () =>
  useMutation({
    mutationFn: saveUser,
  });

export const useAuthenticateUser = () =>
  useMutation({ mutationFn: authenticateUser });
