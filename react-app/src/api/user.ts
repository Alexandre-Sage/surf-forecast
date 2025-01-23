import { useMutation } from "@tanstack/react-query";
import { User, UserPayload } from "../types/user.type";
import { toaster } from "../components/ui/toaster-fn";
import { useTranslation } from "react-i18next";

const baseUrl = "http://localhost:8080";

const fetchWithError = <R>(url: string, opts: RequestInit) =>
  fetch(url, opts).then(async (_): Promise<R> => {
    const json = await _.json();
    if (!_.ok) throw new Error(json.error);
    return json;
  });

const post = <T, R>(url: string, body: T) =>
  fetchWithError<R>(url, {
    method: "POST",
    body: JSON.stringify(body),
    headers: { "content-type": "application/json" },
  });

export const saveUser = (user: UserPayload) => post(`${baseUrl}/users`, user);

export const authenticateUser = (credentials: {
  password: string;
  email: string;
}) => post<any, [User, string]>(`${baseUrl}/users/authenticate`, credentials);

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

export const useAuthenticateUser = () => {
  const { t } = useTranslation();
  return useMutation({
    mutationFn: authenticateUser,
    onSuccess: (res) => {
      localStorage.setItem("token", res[1]);
      toaster.success({
        title: t("success"),
        description: t("login_sucess", { user: res[0].userName }),
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
