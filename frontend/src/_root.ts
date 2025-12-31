import { rootRoute } from "@/commons/routes";
import { waveCalculationRoute } from "@/features/wave-calculation";
import { createRouter } from "@tanstack/react-router";
import { stormGlassForecastRoute } from "@/features/storm-glass-forecast";

const routeTree = rootRoute.addChildren([
  waveCalculationRoute,
  stormGlassForecastRoute,
]);

export const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
