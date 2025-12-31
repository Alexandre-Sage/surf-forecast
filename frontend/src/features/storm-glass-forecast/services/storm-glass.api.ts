import { httpClient, type ApiSuccessResponse } from "@/commons/services";
import type { Coordinates } from "@/commons/types";
import { URL_PATH } from "./api-path";
import type { StormGlassWavesForecast } from "../types/storm-glass-waves-forecast.type";

export const stormGlassWavesForecast = (
  coordinates: Coordinates
): Promise<ApiSuccessResponse<StormGlassWavesForecast>> =>
  httpClient.get(
    `${URL_PATH.STORM_GLASS_WAVES}/${coordinates.latitude()}/${coordinates.longitude()}`
  );
