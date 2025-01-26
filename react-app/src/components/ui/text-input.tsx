import { Box, Input, InputProps } from "@chakra-ui/react";
import { Field } from "./field";
import React, { Dispatch, SetStateAction, useState } from "react";
import { PasswordInput } from "./password-input";
import { useTranslation } from "react-i18next";

export interface TextInputProps<T> extends InputProps {
  label?: string;
  helperText?: string;
  setValue: Dispatch<SetStateAction<T>>;
  field?: keyof T;
  password?: boolean;
}

export const TextInput = <T,>(props: TextInputProps<T>) => {
  const { t } = useTranslation();
  const [localValue, setLocalValue] = useState<string>(props.value as string);
  const setValue = (value: string) =>
    props.field
      ? props.setValue((prev: T) => ({ ...prev, [props.field!]: value }))
      : props.setValue(value as T);
  return (
    <Box>
      <Field
        label={props.label && t(props.label)}
        helperText={props.helperText && t(props.helperText)}
      >
        {props.password ? (
          <PasswordInput
            {...props}
            onChange={(_) => setLocalValue(_.target.value)}
            onBlur={(_) => setValue(localValue)}
          />
        ) : (
          <Input
            {...props}
            type="text"
            onChange={(_) => setLocalValue(_.target.value)}
            onBlur={(_) => setValue(localValue)}
          />
        )}
      </Field>
    </Box>
  );
};
