import { flexRender, type Table as TanstackTable } from "@tanstack/react-table";
import {
  SOURCE_KEYS,
  SOURCE_KEY_MAPPING,
  type WavesSourceValues,
} from "../../types/storm-glass-waves-forecast.type";
import { Table } from "@chakra-ui/react";
import { useMemo } from "react";
import dayjs from "dayjs";
import { Tooltip } from "@/commons/components";
import type { WavesForecastColumnsAccessor } from "./type";

interface ForecastTableHeaderProps {
  table: TanstackTable<WavesForecastColumnsAccessor>;
  time: Date;
}

export const ForecastTableHeader = ({
  table,
  time,
}: ForecastTableHeaderProps) => {
  const headers = useMemo(
    () =>
      table.getHeaderGroups().map((headerGroup) => (
        <Table.Row key={headerGroup.id}>
          {headerGroup.headers.map((header) =>
            header.id === "type" ? (
              <Table.ColumnHeader key={header.id}></Table.ColumnHeader>
            ) : (
              <Tooltip
                key={header.id}
                showArrow
                content={
                  SOURCE_KEY_MAPPING[header.id as keyof WavesSourceValues].long
                }
              >
                <Table.ColumnHeader key={header.id} colSpan={header.colSpan}>
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                </Table.ColumnHeader>
              </Tooltip>
            )
          )}
        </Table.Row>
      )),
    [table]
  );

  return (
    <Table.Header>
      <Table.Row>
        <Table.Cell colSpan={SOURCE_KEYS.length + 1} textAlign="center">
          {dayjs(time).format("DD/MM HH:mm")}
        </Table.Cell>
      </Table.Row>
      {headers}
    </Table.Header>
  );
};
