import { Fragment } from "react";
import { Page } from "./components/ui";
import { DialogUserForm } from "./components/user";
import { Group } from "@chakra-ui/react";

type ObjectValue<T> = T[keyof T];

export const LoginPage = () => {
  return (
    <Page title="welcome">
      <Fragment>
        <Group>
          <DialogUserForm triggerButtonTitle="Sign up" title="New User" />
        </Group>
      </Fragment>
    </Page>
  );
};

function App() {
  return <LoginPage />;
}

export default App;
