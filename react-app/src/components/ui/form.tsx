import {
  Button,
  ButtonProps,
  Fieldset,
  Heading,
  Stack,
} from "@chakra-ui/react";
import { ReactNode } from "react";

export interface FormProps {
  title?: string;
  helperText?: ReactNode;
  children: ReactNode;
  buttonLabel?: string;
  buttonProps?: ButtonProps;
  withButton?: boolean;
}
export const Form = ({ withButton = true, ...props }: FormProps) => {
  return (
    <Fieldset.Root size="md">
      <Stack>
        {" "}
        {props.title && (
          <Fieldset.Legend>
            <Heading size="md">{props.title}</Heading>
          </Fieldset.Legend>
        )}
        {props.helperText && (
          <Fieldset.HelperText>{props.helperText}</Fieldset.HelperText>
        )}
      </Stack>

      <Fieldset.Content>{props.children}</Fieldset.Content>
      {withButton && (
        <Stack>
          <Button {...props.buttonProps}>{props.buttonLabel}</Button>
        </Stack>
      )}
    </Fieldset.Root>
  );
};
