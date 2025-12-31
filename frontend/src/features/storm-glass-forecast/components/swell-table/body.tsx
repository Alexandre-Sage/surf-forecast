import {
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import {
  FORECAST_KEYS,
  type ForecastKey,
  type StormGlassWavesData,
  type WavesSourceValues,
} from "../../types/storm-glass-waves-forecast.type";
import { Table } from "@chakra-ui/react";
import { useMemo } from "react";
import { WAVES_FORECAST_COLUMNS } from "./columns";

interface TableRowProps {
  data: WavesSourceValues & { type: ForecastKey };
}

const ForecastTableRows = ({ data }: TableRowProps) => {
  const table = useReactTable({
    columns: WAVES_FORECAST_COLUMNS,
    data: [data],
    getCoreRowModel: getCoreRowModel(),
    columnResizeMode: "onEnd",
  });

  const rows = useMemo(
    () =>
      table.getRowModel().rows.map((row) => {
        return (
          <Table.Row key={row.id}>
            {row
              .getVisibleCells()
              .map((cell) =>
                flexRender(cell.column.columnDef.cell, cell.getContext())
              )}
          </Table.Row>
        );
      }),
    [table]
  );

  return rows;
};

interface TableBodyProps {
  data: StormGlassWavesData;
}

export const ForecastTableBody = ({ data }: TableBodyProps) => {
  return (
    <Table.Body>
      {FORECAST_KEYS.map((key) => (
        <ForecastTableRows
          key={`${key}-${data.time.toString()}`}
          data={{ ...data[key], type: key }}
        />
      ))}
    </Table.Body>
  );
};
