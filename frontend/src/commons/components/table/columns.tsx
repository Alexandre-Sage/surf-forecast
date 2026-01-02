import { Table as ChakraTable } from "@chakra-ui/react";
import {
  createColumnHelper,
  type ColumnDefBase,
  type IdIdentifier,
} from "@tanstack/react-table";
import React, { useMemo, type ReactNode } from "react";

export type Column<T, X extends keyof T = keyof T> = ColumnDefBase<T, T[X]> &
  IdIdentifier<T, T[X]> & { field: X };

export const useColumns = <T,>(columns: Column<T>[]) =>
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
