import { Group } from "@chakra-ui/react";
import { Fragment } from "react";
import { Page } from "../ui/page";
import { DialogUserForm, LoginForm } from "../user";
import { geoOrthographic } from "d3-geo";
import { LuMapPin } from "react-icons/lu";
import {
  ComposableMap,
  Geographies,
  Geography,
  Marker,
  ZoomableGroup,
} from "react-simple-maps";
import * as geojson from "world-geojson";
const geoUrl = "/globe.geo.json";
//"https://raw.githubusercontent.com/deldersveld/topojson/master/world-countries.json";
export default function MapChart() {
  console.log(geojson);
  return (
    <ComposableMap
      style={{ border: "1px solid blue", background: "white" }}
      mode={2}
      projection="geoAzimuthalEqualArea"
      projectionConfig={{
        rotate: [-10.0, -52.0, 0],
        center: [-5, -3],
        scale: 1100,
      }}
    >
      <ZoomableGroup center={[0, 0]} zoom={30}>
        <Geographies
          geography={
            "https://raw.githubusercontent.com/martynafford/natural-earth-geojson/refs/heads/master/10m/cultural/ne_10m_admin_0_countries.json"
          }
        >
          {({ geographies }) =>
            geographies.map((geo) => (
              <Geography key={geo.rsmKey} geography={geo} />
            ))
          }
        </Geographies>
        <Marker coordinates={[-74.006, 40.7128]}>
          <LuMapPin />
        </Marker>
      </ZoomableGroup>
    </ComposableMap>
  );
}
export const AccessPage = () => {
  return (
    <Page title="welcome">
      <Fragment>
        <MapChart />
        <Group>
          <DialogUserForm
            triggerButtonTitle="Sign up"
            title="New User"
            tipsEnabled
          />
          <LoginForm />
        </Group>
      </Fragment>
    </Page>
  );
};
