import { UNIT, type ObjectValues, type Unit } from "@/commons/types";
import type { WaveSourcesKey } from "../../types";

const INPUT_LABEL = {
  HEIGHT: "Height",
  PERIOD: "Period",
  DIRECTION: "Direction",
} as const;

type InputLabel = ObjectValues<typeof INPUT_LABEL>;

interface WaveCalculationInputConfig {
  label: InputLabel;
  unit: Unit;
  key: WaveSourcesKey;
}

export const WAVE_CALCULATION_INPUT_CONFIG: WaveCalculationInputConfig[] = [
  { label: INPUT_LABEL.HEIGHT, unit: UNIT.METER, key: "height" },
  { label: INPUT_LABEL.PERIOD, unit: UNIT.SECOND, key: "period" },
  { label: INPUT_LABEL.DIRECTION, unit: UNIT.DEGREE, key: "direction" },
] as const;
