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
import { useMemo, type ReactNode } from "react";

interface TableProps<T> {
  data: T[];
  columns: (ColumnDefBase<T, T[keyof T]> &
    IdIdentifier<T, T[keyof T]> & { field: keyof T })[];
  tableOptions?: TableOptions<T>;
}

export const Table = <T,>(props: TableProps<T>) => {
  const columnHelper = createColumnHelper<T>();
  const columns = props.columns.map((colDef) =>
    columnHelper.accessor((row) => row[colDef.field], {
      ...colDef,
      cell:
        colDef.cell ??
        ((cell) => (
          <ChakraTable.Cell>
            {cell.getValue() as unknown as ReactNode}
          </ChakraTable.Cell>
        )),
    })
  );

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
              {flexRender(header.column.columnDef.header, header.getContext())}
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
