import type { WavesForecastColumnsAccessor } from "./type";
import { Table, Text } from "@chakra-ui/react";
import {
  FORECAST_KEY_MAPPING,
  SOURCE_KEY_MAPPING,
  SOURCE_KEYS,
} from "../../types";
import { swellFormater } from "@/commons/components";
import type { Column } from "@/commons/components";

const base: Column<WavesForecastColumnsAccessor, "type"> = {
  id: "type",
  field: "type",
  cell: (cell) => {
    const value = cell.getValue();
    return (
      <Table.Cell key={`${cell.getValue()}-${cell.row.id}-${cell.column.id}`}>
        <Text width={125}>
          {value !== null && FORECAST_KEY_MAPPING[cell.getValue()]}
        </Text>
      </Table.Cell>
    );
  },
};

const sources: Column<
  WavesForecastColumnsAccessor,
  keyof Omit<WavesForecastColumnsAccessor, "type">
>[] = SOURCE_KEYS.map((source) => ({
  id: source,
  field: source,
  header: SOURCE_KEY_MAPPING[source].short,
  cell: (cell) => {
    const value = cell.getValue();
    return (
      <Table.Cell>
        {value ? swellFormater(cell.row.getValue("type"), value) : "N/A"}
      </Table.Cell>
    );
  },
}));

export const WAVES_FORECAST_COLUMNS = [
  base,
  ...sources,
] as Column<WavesForecastColumnsAccessor>[];
