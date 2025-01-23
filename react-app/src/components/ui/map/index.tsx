import { LuMapPin } from "react-icons/lu";
import {
  ComposableMap,
  Geographies,
  Geography,
  Marker,
  ZoomableGroup,
} from "react-simple-maps";
export default function MapChart() {
  return (
    <ComposableMap
      style={{ border: "1px solid blue", background: "white" }}
      mode={2}
      //projection={"geoOrthographic"}
      //projectionConfig={{
      //  rotate: [-10.0, -52.0, 0],
      //  center: [-5, -3],
      //  scale: 1100,
      //}}
    >
      <ZoomableGroup center={[0, 0]} zoom={100}>
        <Geographies geography={"/countries-10m.geo.json"}>
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
