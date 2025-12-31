import { httpClient, type ApiSuccessResponse } from "@/commons/services";
import type { WaveCalculationResponse, WaveInput } from "../types";

const WAVE_CALCULATION_URL = "/calculate/wave";

export const postWaveCalculation = (
  body: WaveInput
): Promise<ApiSuccessResponse<WaveCalculationResponse>> =>
  httpClient.post(WAVE_CALCULATION_URL, body);
