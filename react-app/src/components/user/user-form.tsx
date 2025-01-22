import { Fragment, useState } from "react";
import { UserPayload } from "./type";
import { DialogWithButton, Form, TextInput } from "../ui";
import { Text } from "@chakra-ui/react";
import { useTranslation } from "react-i18next";
import { Translated } from "../ui/text";
import { useMutation } from "@tanstack/react-query";

interface UserFormProps {
  tipsEnabled?: boolean;
}

export const UserForm = (props: UserFormProps) => {
  const [user, setUser] = useState<UserPayload>({} as UserPayload);
  const tips = (
    <Fragment>
      <Translated>press_tab_tips</Translated>
      <Translated>collect_data_info</Translated>
    </Fragment>
  );
  return (
    <Form
      buttonLabel="SEND"
      title="register"
      buttonProps={{ variant: "solid" }}
      helperText={props.tipsEnabled && tips}
    >
      <TextInput label={"user_name"} setValue={setUser} field="userName" />
      <TextInput label={"email"} setValue={setUser} field="email" />
      <TextInput label={"first_name"} setValue={setUser} field="firstName" />
      <TextInput label={"last_name"} setValue={setUser} field="lastName" />
      <TextInput
        label={"password"}
        setValue={setUser}
        field="password"
        password
      />
      <TextInput
        label={"confirm_password"}
        setValue={setUser}
        field="confirmPassword"
        password
      />
    </Form>
  );
};

interface DialogUserForm extends UserFormProps {
  triggerButtonTitle: string;
  title: string;
}
const useSaveUser = () => {
  const mut = useMutation({
    mutationFn: (user: UserPayload) =>
      fetch("http://localhost:8080/users", {
        method: "POST",
        body: JSON.stringify(user),
        headers: {
          "content-type": "application/json",
        },
      }),
  });
  return mut;
};
export const DialogUserForm = (props: DialogUserForm) => {
  const [user, setUser] = useState<UserPayload>({} as UserPayload);
  const { mutate } = useSaveUser();
  const tips = (
    <Fragment>
      <Translated>press_tab_tips</Translated>
      <Translated>collect_data_info</Translated>
    </Fragment>
  );
  return (
    <DialogWithButton
      triggerTitle={props.triggerButtonTitle}
      onSave={() => mutate(user)}
    >
      <Form
        withButton={false}
        title={props.title}
        buttonProps={{ variant: "solid" }}
        helperText={props.tipsEnabled && tips}
      >
        <TextInput label={"user_name"} setValue={setUser} field="userName" />
        <TextInput label={"email"} setValue={setUser} field="email" />
        <TextInput label={"first_name"} setValue={setUser} field="firstName" />
        <TextInput label={"last_name"} setValue={setUser} field="lastName" />
        <TextInput
          label={"password"}
          setValue={setUser}
          field="password"
          password
        />
        <TextInput
          label={"confirm_password"}
          setValue={setUser}
          field="confirmPassword"
          password
        />
      </Form>
    </DialogWithButton>
  );
};
