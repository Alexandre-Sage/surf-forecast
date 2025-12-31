import { Button } from "./button";

export interface SubmitButtonProps {
  onClick: () => void;
  loading?: boolean;
}

export const SubmitButton = (props: SubmitButtonProps) => (
  <Button label="Submit" loading={props.loading} onClick={props.onClick} />
);
