import { Fragment } from "react";
import { Page } from "../ui/page";
import { DialogUserForm, LoginForm } from "../user";
import { Button, createToaster, Group } from "@chakra-ui/react";
import { toaster } from "../ui/toaster-fn";
import { Toaster } from "../ui/toaster";

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
