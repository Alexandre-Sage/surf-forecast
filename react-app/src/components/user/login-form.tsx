import { useState } from "react";
import { useAuthenticateUser } from "../../api/user";
import { DialogWithButton, Form, TextInput } from "../ui";

interface Credentials {
  password: string;
  email: string;
}
export const LoginForm = () => {
  const [credentials, setCredentials] = useState<Credentials>(
    {} as Credentials,
  );
  const { mutate } = useAuthenticateUser();
  return (
    <DialogWithButton
      triggerTitle="Login"
      onSave={() => mutate(credentials)}
      triggerButtonProps={{ width: "10vw" }}
    >
      <Form withButton={false}>
        <TextInput label="email" setValue={setCredentials} field="email" />
        <TextInput
          label="password"
          password
          setValue={setCredentials}
          field="password"
        />
      </Form>
    </DialogWithButton>
  );
};
