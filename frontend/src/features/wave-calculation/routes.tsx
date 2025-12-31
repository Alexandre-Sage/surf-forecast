import { createRoute } from "@tanstack/react-router";
import { WaveCalculation } from "./components/wave-calculation";
import { rootRoute } from "@/commons/routes";

export const WAVE_CALCULATION_ROUTE_PATH = "/calculation/wave";

export const waveCalculationRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: WAVE_CALCULATION_ROUTE_PATH,
  component: () => {
    return <WaveCalculation />;
  },
});
