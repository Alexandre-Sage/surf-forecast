import { getCoreRowModel, useReactTable } from "@tanstack/react-table";
import { type StormGlassWavesData } from "../../types/storm-glass-waves-forecast.type";
import { Table } from "@chakra-ui/react";
import { ForecastTableHeader } from "./headers";
import { ForecastTableBody } from "./body";
import { WAVES_FORECAST_COLUMNS } from "./columns";

interface ForecastTableProps {
  data: StormGlassWavesData;
  time: Date;
}

export const StormGlassWavesForecastTable = ({
  data,
  time,
}: ForecastTableProps) => {
  const table = useReactTable({
    columns: WAVES_FORECAST_COLUMNS,
    data: [],
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <Table.Root
      showColumnBorder
      variant={"outline"}
      size={"sm"}
      width={"sm"}
      fontSize={"2xs"}
    >
      <ForecastTableHeader table={table} time={time} />
      <ForecastTableBody data={data} />
    </Table.Root>
  );
};
