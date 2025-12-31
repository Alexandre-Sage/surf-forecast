import { Button } from "./button";

export interface ResetButtonProps {
  onClick: () => void;
}

export const ResetButton = (props: ResetButtonProps) => (
  <Button label="Reset" variant={"outline"} onClick={props.onClick} />
);
