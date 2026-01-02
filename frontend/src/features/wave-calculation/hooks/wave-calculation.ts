import { useMutation, type UseMutationOptions } from "@tanstack/react-query";
import { postWaveCalculation } from "../services/wave-calculation.api";
import type { ApiSuccessResponse } from "@/commons/services";
import type { WaveCalculationResponse, WaveInput } from "../types";

type Opts = Omit<
  UseMutationOptions<
    ApiSuccessResponse<WaveCalculationResponse>,
    Error,
    WaveInput,
    unknown
  >,
  "mutationFn"
>;
export const useWaveCalculation = (options?: Opts) =>
  useMutation({
    ...options,
    mutationFn: postWaveCalculation,
  });
