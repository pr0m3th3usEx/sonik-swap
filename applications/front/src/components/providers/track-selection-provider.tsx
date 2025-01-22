/* eslint-disable @typescript-eslint/no-explicit-any */
'use client';

import { createContext, SetStateAction, useContext, useState } from 'react';

export interface TrackSelectionContextProps<TData> {
  rowSelection: Record<string, boolean>;
  setRowSelection: (value: SetStateAction<Record<string, boolean>>) => void;
  setDataSelected: (value: SetStateAction<TData[]>) => void;
  nbRowsSelected: number;
  dataSelected: TData[];
};

const defaultValue: TrackSelectionContextProps<any> = {
  rowSelection: {},
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  setRowSelection: (_) => {},
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  setDataSelected: (_) => {},
  nbRowsSelected: 0,
  dataSelected: [],
};

const TrackSelectionContext =
  createContext<TrackSelectionContextProps<any>>(defaultValue);

export const useTrackSelection = () => useContext(TrackSelectionContext);

export default function TrackSelectionProvider<TData>({
  children,
}: {
  children: React.ReactNode;
}) {
  const [rowSelection, setRowSelection] = useState<Record<string, boolean>>({});
  const [dataSelected, setDataSelected] = useState<TData[]>([]);
  const nbRowsSelected = Object.keys(rowSelection).length;

  return (
    <TrackSelectionContext.Provider
      value={{
        rowSelection,
        setRowSelection,
        nbRowsSelected,
        setDataSelected,
        dataSelected,
      }}
    >
      {children}
    </TrackSelectionContext.Provider>
  );
}
