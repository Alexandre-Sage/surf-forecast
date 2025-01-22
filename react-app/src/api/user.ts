import { useMutation } from "@tanstack/react-query";
import { UserPayload } from "../types/user.type";
import { toaster } from "../components/ui/toaster-fn";
import { useTranslation } from "react-i18next";

const baseUrl = "http://localhost:8080";

export const saveUser = (user: UserPayload) =>
  fetch("http://localhost:8080/users", {
    method: "POST",
    body: JSON.stringify(user),
    headers: {
      "content-type": "application/json",
    },
  }).then(async (res) => {
    if (!res.ok) throw new Error((await res.json()).error);
    return await res.json();
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
export const useSaveUser = () => {
  const { t } = useTranslation();
  return useMutation({
    mutationFn: saveUser,
    onSuccess: () => {
      return toaster.success({
        title: t("succes"),
        description: t("user_created"),
        duration: 10000,
      });
    },
    onError: (e) => {
      return toaster.error({
        title: t("error"),
        description: t(e.message),
        duration: 10000,
      });
    },
  });
};

export const useAuthenticateUser = () =>
  useMutation({ mutationFn: authenticateUser });
