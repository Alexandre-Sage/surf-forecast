import { Table as ChakraTable } from "@chakra-ui/react";
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  useReactTable,
  type ColumnDefBase,
  type IdIdentifier,
  type TableOptions,
} from "@tanstack/react-table";
import React, { useMemo, type ReactNode } from "react";

export type Column<T, X extends keyof T = keyof T> = ColumnDefBase<T, T[X]> &
  IdIdentifier<T, T[X]> & { field: X };

interface TableProps<T> {
  data: T[];
  columns: Column<T, keyof T>[];
  tableOptions?: TableOptions<T>;
}

const useColumns = <T,>(columns: Column<T, keyof T>[]) =>
  useMemo(() => {
    const columnHelper = createColumnHelper<T>();
    return columns.map((colDef) =>
      columnHelper.accessor((row) => row[colDef.field], {
        ...colDef,
        cell:
          colDef.cell ??
          ((cell) => {
            return (
              <ChakraTable.Cell>
                {cell.getValue() as unknown as ReactNode}
              </ChakraTable.Cell>
            );
          }),
      })
    );
  }, [columns]);

export const Table = <T,>(props: TableProps<T>) => {
  const columns = useColumns(props.columns);

  // eslint-disable-next-line react-hooks/incompatible-library
  const table = useReactTable({
    data: props.data,
    columns: columns,
    getCoreRowModel: getCoreRowModel(),
    ...props.tableOptions,
  });

  const headers = useMemo(
    () =>
      table.getHeaderGroups().map((headerGroup) => (
        <ChakraTable.Row key={headerGroup.id}>
          {headerGroup.headers.map((header) => (
            <ChakraTable.ColumnHeader key={header.id}>
              {header.isPlaceholder
                ? null
                : flexRender(
                    header.column.columnDef.header,
                    header.getContext()
                  )}
            </ChakraTable.ColumnHeader>
          ))}
        </ChakraTable.Row>
      )),
    [table]
  );

  const rows = useMemo(
    () =>
      table.getRowModel().rows.map((row) => (
        <ChakraTable.Row key={row.id}>
          {row.getVisibleCells().map((cell) => {
            return flexRender(cell.column.columnDef.cell, cell.getContext());
          })}
        </ChakraTable.Row>
      )),
    [table]
  );

  return (
    <ChakraTable.Root
      showColumnBorder
      variant={"outline"}
      size={"sm"}
      width={"sm"}
      fontSize={"2xs"}
    >
      <ChakraTable.Header>{headers}</ChakraTable.Header>
      <ChakraTable.Body>{rows}</ChakraTable.Body>
    </ChakraTable.Root>
  );
};
