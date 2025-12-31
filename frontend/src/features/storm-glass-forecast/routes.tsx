import { createRoute } from "@tanstack/react-router";
import { StormGlassForecast } from "./components/storm-glass-forecast";
import { rootRoute } from "@/commons/routes";
import { Coordinates } from "../../commons/types";

export const FORECAST_ROUTE_PATH = "/forecast/$latitude/$longitude";

export const stormGlassForecastRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: FORECAST_ROUTE_PATH,
  loader: ({ params }) => ({
    coordinates: Coordinates.fromString(params.latitude, params.longitude),
  }),
  component: () => {
    const { coordinates } = stormGlassForecastRoute.useLoaderData();
    return <StormGlassForecast coordinates={coordinates} />;
  },
});
