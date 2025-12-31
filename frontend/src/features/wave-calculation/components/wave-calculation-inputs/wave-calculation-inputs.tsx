import { NumberInput } from "@/commons/components";
import { type SetState } from "@/commons/types";
import { Stack, For, Flex, Field } from "@chakra-ui/react";
import type { WaveInput, WaveInputKey } from "../../types";
import { WAVE_CALCULATION_INPUT_CONFIG } from "./type";

interface WaveCalculationInputsProps {
  label: string;
  onInputChange: SetState<WaveInput>;
  waveInputKey: WaveInputKey;
  waveInput: WaveInput;
}

export const WaveCalculationInputs = ({
  label,
  onInputChange,
  waveInputKey,
  waveInput,
}: WaveCalculationInputsProps) => {
  return (
    <Stack>
      <Field.Root>
        <Field.Label>{label}</Field.Label>
        <Flex direction={"row"} gapX={4}>
          <For each={WAVE_CALCULATION_INPUT_CONFIG}>
            {(config) => (
              <NumberInput
                label={config.label}
                unit={config.unit}
                value={waveInput[waveInputKey][config.key] ?? 0.0}
                onChange={(value) =>
                  onInputChange((prev) => ({
                    ...prev,
                    [waveInputKey]: {
                      ...prev[waveInputKey],
                      [config.key]: value,
                    },
                  }))
                }
                mode="decimal"
              />
            )}
          </For>
        </Flex>
      </Field.Root>
    </Stack>
  );
};
