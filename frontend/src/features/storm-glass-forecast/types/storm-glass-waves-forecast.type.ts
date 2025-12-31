export interface WavesSourceValues {
  nationalOceanicAndAtmosphericAdministration: number | null;
  stormGlass: number | null;
  deutscherWetterdienst: number | null;
  meteoFrance: number | null;
  norwegianMeteorologicalInstitute: number | null;
  europeanCentreForMediumRangeWeatherForecasts: number | null;
  danishMeteorologicalInstitute: number | null;
  finnishMeteorologicalInstitute: number | null;
  average: number | null;
}

export interface StormGlassWavesData {
  secondarySwellDirection: WavesSourceValues;
  secondarySwellHeight: WavesSourceValues;
  secondarySwellPeriod: WavesSourceValues;
  swellDirection: WavesSourceValues;
  swellHeight: WavesSourceValues;
  swellPeriod: WavesSourceValues;
  time: Date;
  waveDirection: WavesSourceValues;
  waveHeight: WavesSourceValues;
  wavePeriod: WavesSourceValues;
  windWaveDirection: WavesSourceValues;
  windWaveHeight: WavesSourceValues;
  windWavePeriod: WavesSourceValues;
}

export type StormGlassWavesForecast = StormGlassWavesData[];

export type ForecastKey = keyof Omit<StormGlassWavesData, "time">;

export const FORECAST_KEYS: ReadonlyArray<ForecastKey> & { length: 12 } = [
  "swellDirection",
  "swellHeight",
  "swellPeriod",
  "secondarySwellDirection",
  "secondarySwellHeight",
  "secondarySwellPeriod",
  "waveDirection",
  "waveHeight",
  "wavePeriod",
  "windWaveDirection",
  "windWaveHeight",
  "windWavePeriod",
] as const;

export const FORECAST_KEY_MAPPING: Record<ForecastKey, string> = {
  secondarySwellDirection: "Secondary swell direction",
  secondarySwellHeight: "Secondary swell height",
  secondarySwellPeriod: "Secondary swell period",
  swellDirection: "Swell direction",
  swellHeight: "Swell height",
  swellPeriod: "Swell period",
  waveDirection: "Wave direction",
  waveHeight: "Wave height",
  wavePeriod: "Wave period",
  windWaveDirection: "Wind wave direction",
  windWaveHeight: "Wind wave height",
  windWavePeriod: "Wind wave period",
} as const;

export const SOURCE_KEYS: ReadonlyArray<keyof WavesSourceValues> = [
  "average",
  "stormGlass",
  "meteoFrance",
  "europeanCentreForMediumRangeWeatherForecasts",
  "nationalOceanicAndAtmosphericAdministration",
  "deutscherWetterdienst",
  "danishMeteorologicalInstitute",
  "finnishMeteorologicalInstitute",
  "norwegianMeteorologicalInstitute",
];

export const SOURCE_KEY_MAPPING: Record<
  keyof WavesSourceValues,
  { short: string; long: string }
> = {
  average: { short: "AVG", long: "Average" },
  stormGlass: { short: "SG", long: "Storm glass" },
  meteoFrance: { short: "METEO", long: "Meteo France" },
  deutscherWetterdienst: { short: "DWD", long: "Deutscher Wetterdienst" },
  danishMeteorologicalInstitute: {
    short: "FCOO",
    long: "Danish Meteorological Institute",
  },
  finnishMeteorologicalInstitute: {
    short: "FMI",
    long: "Finnish Meteorological Institute",
  },
  norwegianMeteorologicalInstitute: {
    short: "METNO",
    long: "Norwegian Meteorological Institute",
  },
  nationalOceanicAndAtmosphericAdministration: {
    short: "NOAA",
    long: "National Oceanic and Atmospheric Administration",
  },
  europeanCentreForMediumRangeWeatherForecasts: {
    short: "ECMWF",
    long: "European Centre for Medium-Range Weather Forecasts",
  },
} as const;

// export type Shape = Record<
//   keyof Omit<StormGlassWavesData, "time">,
//   ReadonlyArray<keyof WavesSourceValues> & { length: 9 }
// >;
// export const shape: Shape = Object.fromEntries(
//   forecastKey.map((key) => [key, sourceKeys]),
// ) as Shape;
