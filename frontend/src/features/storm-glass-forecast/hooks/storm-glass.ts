import type { Coordinates } from "@/commons/types";
import { useQuery } from "@tanstack/react-query";
import { stormGlassWavesForecast } from "../services/storm-glass.api";
import { FORECAST_QUERY_KEYS } from "./query-keys";
import type { StormGlassWavesForecast } from "../types/storm-glass-waves-forecast.type";
import type { ApiErrorResponse, ApiSuccessResponse } from "@/commons/services";

export const useCoordinatesForecast = (coordinates: Coordinates) =>
  useQuery<ApiSuccessResponse<StormGlassWavesForecast>, ApiErrorResponse>({
    queryKey: [
      FORECAST_QUERY_KEYS.STORM_GLASS_WAVES_FORECAST,
      coordinates.latitude(),
      coordinates.longitude(),
    ],
    queryFn: () => stormGlassWavesForecast(coordinates),
    select: (data) => {
      const sorted = data.payload.sort(
        (datePrev, dateCur) =>
          new Date(datePrev.time).getTime() - new Date(dateCur.time).getTime()
      );

      return {
        ...data,
        payload: sorted,
      };
    },
  });
