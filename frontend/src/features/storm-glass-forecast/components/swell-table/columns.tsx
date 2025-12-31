import { createColumnHelper } from "@tanstack/react-table";
import type { WaveForecastColumns, WavesForecastColumnsAccessor } from "./type";
import { Table, Text } from "@chakra-ui/react";
import {
  FORECAST_KEY_MAPPING,
  SOURCE_KEY_MAPPING,
  SOURCE_KEYS,
} from "../../types";
import { swellFormater } from "./formaters";

const columnHelper = createColumnHelper<WavesForecastColumnsAccessor>();

const base = columnHelper.accessor("type", {
  id: "type",
  cell: (cell) => (
    <Table.Cell key={`${cell.getValue()}-${cell.row.id}-${cell.column.id}`}>
      <Text width={125}>{FORECAST_KEY_MAPPING[cell.getValue()]}</Text>
    </Table.Cell>
  ),
});
const sources = SOURCE_KEYS.map((source) =>
  columnHelper.accessor((row) => row[source], {
    id: source,
    header: SOURCE_KEY_MAPPING[source].short,
    cell: (cell) => {
      const value = cell.getValue();
      return (
        <Table.Cell>
          {value ? swellFormater(cell.row.getValue("type"), value) : "N/A"}
        </Table.Cell>
      );
    },
  })
);

export const WAVES_FORECAST_COLUMNS = [base, ...sources] as WaveForecastColumns;
