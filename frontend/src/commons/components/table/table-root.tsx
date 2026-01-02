import { Table as ChakraTable } from "@chakra-ui/react";
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
  type TableOptions,
} from "@tanstack/react-table";
import { useMemo } from "react";
import { useColumns, type Column } from "./columns";

interface TableProps<T> {
  data: T[];
  columns: Column<T, keyof T>[];
  tableOptions?: TableOptions<T>;
}

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

  const rows = table.getRowModel().rows.map((row) => (
    <ChakraTable.Row key={row.id}>
      {row.getVisibleCells().map((cell) => {
        return flexRender(cell.column.columnDef.cell, cell.getContext());
      })}
    </ChakraTable.Row>
  ));

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
