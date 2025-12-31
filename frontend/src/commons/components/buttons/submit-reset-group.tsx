import { ResetButton, type ResetButtonProps } from "./reset-button";
import { SubmitButton, type SubmitButtonProps } from "./submit-button";
import { Group } from "@chakra-ui/react";

interface SubmitResetGroupProps {
  onSubmit: SubmitButtonProps["onClick"];
  loading?: SubmitButtonProps["loading"];
  onReset: ResetButtonProps["onClick"];
}
export const SubmitResetGroup = (props: SubmitResetGroupProps) => (
  <Group>
    <SubmitButton onClick={props.onSubmit} loading={props.loading} />
    <ResetButton onClick={props.onReset} />
  </Group>
);
