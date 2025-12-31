import type { AccessorFnColumnDef } from "@tanstack/react-table";
import type {
  ForecastKey,
  WavesSourceValues,
} from "../../types/storm-glass-waves-forecast.type";

export type WaveForecastColumns = AccessorFnColumnDef<
  WavesForecastColumnsAccessor,
  ForecastKey | number | null
>[];

export interface WavesForecastColumnsAccessor extends WavesSourceValues {
  type: ForecastKey;
}
