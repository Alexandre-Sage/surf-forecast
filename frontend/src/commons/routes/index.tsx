import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { Fragment } from "react";
import { FORECAST_ROUTE_PATH } from "@/features/storm-glass-forecast";
import { WAVE_CALCULATION_ROUTE_PATH } from "@/features/wave-calculation";
import { Text } from "@chakra-ui/react";
import { ROUTE_LABEL } from "./route-label";

export const rootRoute = createRootRoute({
  component: () => (
    <Fragment>
      <Link
        to={FORECAST_ROUTE_PATH}
        params={{ latitude: "47.5", longitude: "-3.2" }}
      >
        <Text>{ROUTE_LABEL.STORM_GLASS_FORECAST}</Text>
      </Link>
      <Link to={WAVE_CALCULATION_ROUTE_PATH}>
        <Text>{ROUTE_LABEL.WAVE_CALCULATION}</Text>
      </Link>
      <Outlet />
      <TanStackRouterDevtools />
    </Fragment>
  ),
});
