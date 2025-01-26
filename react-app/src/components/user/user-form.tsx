import { Fragment, useState } from "react";
import { DialogWithButton, Form, TextInput } from "../ui";
import { Translated } from "../ui/text";
import { UserPayload } from "../../types/user.type";
import { useSaveUser } from "../../api/user";

interface UserFormProps {
  tipsEnabled?: boolean;
}

//export const UserForm = (props: UserFormProps) => {
//  const [user, setUser] = useState<UserPayload>({} as UserPayload);
//  const tips = (
//    <Fragment>
//      <Translated>press_tab_tips</Translated>
//      <Translated>collect_data_info</Translated>
//    </Fragment>
//  );
//  return (
//    <Form
//      buttonLabel="SEND"
//      title="register"
//      buttonProps={{ variant: "solid" }}
//      helperText={props.tipsEnabled && tips}
//    >
//      <TextInput label={"user_name"} setValue={setUser} field="userName" />
//      <TextInput label={"email"} setValue={setUser} field="email" />
//      <TextInput label={"first_name"} setValue={setUser} field="firstName" />
//      <TextInput label={"last_name"} setValue={setUser} field="lastName" />
//      <TextInput
//        label={"password"}
//        setValue={setUser}
//        field="password"
//        password
//      />
//      <TextInput
//        label={"confirm_password"}
//        setValue={setUser}
//        field="confirmPassword"
//        password
//      />
//    </Form>
//  );
//};

interface DialogUserForm extends UserFormProps {
  triggerButtonTitle: string;
  title: string;
}
export const DialogUserForm = (props: DialogUserForm) => {
  const [user, setUser] = useState<UserPayload>({} as UserPayload);
  const { mutateAsync } = useSaveUser();
  const actionEnabled =
    user.confirmPassword &&
    user.password &&
    user.firstName &&
    user.lastName &&
    user.email &&
    user.userName;
  const tips = (
    <Fragment>
      <Translated>press_tab_tips</Translated>
      <Translated>collect_data_info</Translated>
    </Fragment>
  );
  return (
    <DialogWithButton
      triggerTitle={props.triggerButtonTitle}
      actionDisabled={!actionEnabled}
      onSave={(_) =>
        mutateAsync(user).then(() => {
          setUser({} as UserPayload);
          return _.setOpen(false);
        })
      }
      triggerButtonProps={{ width: "10vw" }}
    >
      <Form
        withButton={false}
        title={props.title}
        buttonProps={{ variant: "solid" }}
        helperText={props.tipsEnabled && tips}
      >
        <TextInput
          value={user.userName}
          label={"user_name"}
          setValue={setUser}
          field="userName"
          required
        />
        <TextInput
          value={user.email}
          label={"email"}
          setValue={setUser}
          field="email"
          required
        />
        <TextInput
          value={user.firstName}
          label={"first_name"}
          setValue={setUser}
          field="firstName"
          required
        />
        <TextInput
          value={user.lastName}
          label={"last_name"}
          setValue={setUser}
          field="lastName"
          required
        />
        <TextInput
          value={user.password}
          label={"password"}
          setValue={setUser}
          field="password"
          password
          required
        />
        <TextInput
          value={user.confirmPassword}
          label={"confirm_password"}
          setValue={setUser}
          field="confirmPassword"
          password
          required
        />
      </Form>
    </DialogWithButton>
  );
};
