import { type Coordinates } from "@/commons/types";
import { useCoordinatesForecast } from "../hooks/storm-glass";
import { DatePicker, Loading } from "@/commons/components";
import dayjs, { Dayjs } from "dayjs";
import utc from "dayjs/plugin/utc";
import { Flex, Grid, GridItem } from "@chakra-ui/react";
import { useMemo, useState } from "react";
import "react-day-picker/style.css";
import { HoursControl } from "@/commons/components/hours-control/hours-control";
import { Time } from "@/commons/types/time";
import { FORECAST_KEYS } from "../types/storm-glass-waves-forecast.type";
import { Table } from "@/commons/components";
import { WAVES_FORECAST_COLUMNS } from "./swell-table/columns";
dayjs.extend(utc);

interface StormGlassForecastProps {
  coordinates: Coordinates;
}

export const StormGlassForecast = (props: StormGlassForecastProps) => {
  const { isLoading, data } = useCoordinatesForecast(props.coordinates);
  const [forecastDate, setForecastDate] = useState<Dayjs>(dayjs().local());
  const [forecastTime, setForecastTime] = useState<Time | null>(
    Time.fromString(forecastDate.format("HH:00"))!
  );

  const memoData = useMemo(() => {
    const hourForecast = (data?.payload || []).find((data) => {
      const time = dayjs(data.time);

      return (
        time.isSame(forecastDate, "date") && time.hour() === forecastDate.hour()
      );
    });

    return hourForecast
      ? FORECAST_KEYS.map((key) => ({
          ...hourForecast[key],
          type: key,
        }))
      : [];
  }, [data, forecastDate]);

  if (isLoading) return <Loading />;

  return (
    <Grid flexDir={"column"} justifyContent={"center"}>
      <GridItem>
        <Flex
          flexDir={"row"}
          alignItems={"center"}
          justifyItems={"center"}
          gapX={2}
        >
          <DatePicker
            onChange={(date) => setForecastDate(dayjs(date))}
            selected={forecastDate.toDate()}
          />
          <HoursControl
            value={forecastTime}
            onChange={(value) => {
              setForecastTime(value);
              const updated = forecastDate.set("hours", value?.hours ?? 0);
              setForecastDate(updated);
            }}
          />
        </Flex>
      </GridItem>
      <Table columns={WAVES_FORECAST_COLUMNS} data={memoData} />
    </Grid>
  );
};
