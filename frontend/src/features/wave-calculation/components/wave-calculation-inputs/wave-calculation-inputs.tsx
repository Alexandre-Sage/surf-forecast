import { NumberInput } from "@/commons/components";
import { type SetState } from "@/commons/types";
import { Stack, For, Flex, Field } from "@chakra-ui/react";
import type { WaveInput, WaveInputKey } from "../../types";
import { WAVE_CALCULATION_INPUT_CONFIG } from "./type";

export const WaveCalculationInputs = ({
  label,
  onChange,
  waveInputKey,
  waveInput,
}: {
  label: string;
  onChange: SetState<WaveInput>;
  waveInputKey: WaveInputKey;
  waveInput: WaveInput;
}) => {
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
                  onChange((prev) => ({
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
