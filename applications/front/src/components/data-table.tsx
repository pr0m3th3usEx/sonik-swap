'use client';

import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from '@tanstack/react-table';
import { Table, TableBody, TableHeader, TableRow } from './ui/table';
import React, { useEffect } from 'react';
import { useTrackSelection } from './providers/track-selection-provider';

interface DataTableProps<TData, TValue> {
  columns: ColumnDef<TData, TValue>[];
  data: TData[];
}

export default function DataTable<TData, TValue>({
  data,
  columns,
}: DataTableProps<TData, TValue>) {
  const { rowSelection, setRowSelection, setDataSelected } =
    useTrackSelection();

  const tracksTable = useReactTable({
    columns,
    data,
    getCoreRowModel: getCoreRowModel(),
    enableRowSelection: true,
    state: {
      rowSelection,
    },
    onRowSelectionChange: setRowSelection,
  });

  useEffect(() => {
    setDataSelected(
      tracksTable.getSelectedRowModel().rows.map((row) => row.original),
    );
  }, [tracksTable, rowSelection, setDataSelected]);

  return (
    <Table>
      <TableHeader>
        {tracksTable.getHeaderGroups().map((headerGroup) => (
          <TableRow key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <React.Fragment key={header.id}>
                {flexRender(
                  header.column.columnDef.header,
                  header.getContext(),
                )}
              </React.Fragment>
            ))}
          </TableRow>
        ))}
      </TableHeader>
      <TableBody>
        {tracksTable.getRowModel().rows.map((row) => {
          return (
            <TableRow
              key={row.id}
              onClick={() => row.toggleSelected(!row.getIsSelected())}
              data-state={row.getIsSelected() ? 'selected' : undefined}
              className="cursor-pointer"
            >
              {row.getVisibleCells().map((cell) => {
                return (
                  <React.Fragment key={cell.id}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </React.Fragment>
                );
              })}
            </TableRow>
          );
        })}
      </TableBody>
    </Table>
  );
}
