import { useMemo, useState, type ReactNode } from "react";
import type {
  WaveCalculationResponse,
  WaveInput,
  WaveInputKey,
} from "../types";
import { WaveCalculationInputs } from "./wave-calculation-inputs/wave-calculation-inputs";
import { Flex, For, Stack } from "@chakra-ui/react";
import type { ObjectValues } from "@/commons/types";
import { useWaveCalculation } from "../hooks/wave-calculation";
import { SubmitResetGroup } from "@/commons/components";
import { Table } from "@/commons/components/table";

// interface WaveCalculationProps {}

const WAVE_CALCULATION_LABEL = {
  SECONDARY_SWELL: "Secondary swell",
  PRIMARY_WELL: "Primary swell",
  WIND_WAVES: "Wind waves",
} as const;

type WaveCalculationLabel = ObjectValues<typeof WAVE_CALCULATION_LABEL>;

interface WaveCalculationConfig {
  label: WaveCalculationLabel;
  key: WaveInputKey;
}

const WAVE_CALCULATION_CONFIG: WaveCalculationConfig[] = [
  { key: "primarySwell", label: WAVE_CALCULATION_LABEL.PRIMARY_WELL },
  { key: "secondarySwell", label: WAVE_CALCULATION_LABEL.SECONDARY_SWELL },
  { key: "windWaves", label: WAVE_CALCULATION_LABEL.WIND_WAVES },
] as const;

const defaultWaveInput = {
  primarySwell: { direction: 0.0, period: 0.0, height: 0.0 },
  secondarySwell: { direction: 0.0, period: 0.0, height: 0.0 },
  windWaves: { direction: 0.0, period: 0.0, height: 0.0 },
};

const ResultTable = ({ data }: { data: WaveCalculationResponse }) => {
  return (
    <Table
      columns={[
        { field: "rss", id: "rss" },
        {
          field: "rssDirectional",
          id: "rssDirectional",
        },
      ]}
      data={[data]}
    />
  );
};

export const WaveCalculation = () => {
  const [value, setValue] = useState<WaveInput>(defaultWaveInput);

  const { mutateAsync, isPending, data } = useWaveCalculation();

  return (
    <Stack alignItems={"center"} gapY={5}>
      <Flex direction={"column"} gapY={5}>
        <For each={WAVE_CALCULATION_CONFIG}>
          {(config) => (
            <WaveCalculationInputs
              label={config.label}
              waveInputKey={config.key}
              waveInput={value}
              onInputChange={setValue}
            />
          )}
        </For>
      </Flex>
      <SubmitResetGroup
        onSubmit={async () => await mutateAsync(value)}
        onReset={() => setValue(defaultWaveInput)}
        loading={isPending}
      />
      {data && <ResultTable data={data.payload} />}
    </Stack>
  );
};
