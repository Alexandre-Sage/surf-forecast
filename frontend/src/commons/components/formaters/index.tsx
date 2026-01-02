import { match } from "ts-pattern";
import { PeriodFormater } from "./period";
import { SwellFormater } from "./swell";
import { DirectionFormater } from "./direction";
import type { ForecastKey } from "../../../types";

export const swellFormater = (key: ForecastKey, value: number) =>
  match(key)
    .with(
      "secondarySwellDirection",
      "swellDirection",
      "windWaveDirection",
      "waveDirection",
      () => <DirectionFormater value={value} />
    )
    .with(
      "secondarySwellPeriod",
      "swellPeriod",
      "wavePeriod",
      "windWavePeriod",
      () => <PeriodFormater value={value} />
    )
    .with(
      "waveHeight",
      "swellHeight",
      "secondarySwellHeight",
      "windWaveHeight",
      () => <SwellFormater value={value} />
    )
    .exhaustive();
