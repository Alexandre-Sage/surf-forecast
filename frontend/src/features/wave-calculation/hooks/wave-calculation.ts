import { useMutation } from "@tanstack/react-query";
import { postWaveCalculation } from "../services/wave-calculation.api";

export const useWaveCalculation = () =>
  useMutation({
    mutationFn: postWaveCalculation,
  });
