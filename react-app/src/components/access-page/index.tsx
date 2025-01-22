import { Fragment } from "react";
import { Page } from "../ui/page";
import { DialogUserForm, LoginForm } from "../user";
import { Group } from "@chakra-ui/react";

export const AccessPage = () => {
  return (
    <Page title="welcome">
      <Fragment>
        <Group>
          <DialogUserForm
            triggerButtonTitle="Sign up"
            title="New User"
            tipsEnabled
          />
          <LoginForm />
        </Group>
      </Fragment>
    </Page>
  );
};
