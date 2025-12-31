import type { SetState } from "@/commons/types";
import type { Unit } from "@/commons/types/units";
import {
  NumberInput as ChakraNumberInput,
  Field,
  type NumberInputRootProps,
} from "@chakra-ui/react";

interface NumberInputProps {
  label?: string;
  onChange: (value: number) => void | SetState<number | null>;
  value?: number;
  mode?: NumberInputRootProps["inputMode"];
  unit?: Unit;
  unitDisplay?: Intl.NumberFormatOptions["unitDisplay"];
}

export const NumberInput = (props: NumberInputProps) => {
  const step = props.mode === "decimal" ? 0.1 : 1;

  const formatOptions = props.unit
    ? {
        style: "unit" as Intl.NumberFormatOptions["style"],
        unit: props.unit,
        unitDisplay: props.unitDisplay ?? "narrow",
      }
    : {};

  return (
    <Field.Root>
      <Field.Label>{props.label}</Field.Label>
      <ChakraNumberInput.Root
        onValueChange={(event) => props.onChange(event.valueAsNumber)}
        inputMode={props.mode ?? "numeric"}
        spinOnPress
        value={props.value?.toString() ?? "0"}
        step={step}
        formatOptions={formatOptions}
        width="200px"
        allowMouseWheel
      >
        <ChakraNumberInput.Control />
        <ChakraNumberInput.Input />
      </ChakraNumberInput.Root>
    </Field.Root>
  );
};
