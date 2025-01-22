'use client';

import Image from 'next/image';
import Link from 'next/link';
import {
  getCoreRowModel,
  useReactTable,
  createColumnHelper,
  flexRender,
} from '@tanstack/react-table';
import { Button } from '@/components/ui/button';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import SpotifyIcon from '@/lib/theme/assets/icons/spotify.svg';
import DeezerIcon from '@/lib/theme/assets/icons/deezer.svg';
import React from 'react';

type Track = {
  ids: Record<string, string>;
  coverImg: string;
  title: string;
  artist: string;
  durationMs: number;
  album: string;
  urls: Record<string, string>;
};

const providersIcon: Record<string, React.ReactNode> = {
  spotify: <SpotifyIcon />,
  deezer: <DeezerIcon />,
};

const columnHelper = createColumnHelper<Track>();

export default function PlaylistManagerPage() {
  const columns = [
    columnHelper.display({
      id: 'title',
      header: () => <TableHead className="w-[300px]">Title</TableHead>,
      cell: (props) => (
        <TableCell>
          <div className="flex gap-4 items-center">
            <div className="w-12 aspect-square overflow-hidden rounded-sm">
              <Image
                src={props.row.original.coverImg}
                alt="album logo"
                className="object-cover"
                width={48}
                height={48}
              />
            </div>
            <div className="flex flex-col gap-0">
              <h3 className="text-base">{props.row.original.title}</h3>
              <p className="text-primary/80">{props.row.original.artist}</p>
            </div>
          </div>
        </TableCell>
      ),
    }),
    columnHelper.accessor('album', {
      header: () => <TableHead>Album</TableHead>,
      cell: (props) => <TableCell>{props.getValue()}</TableCell>,
    }),
    columnHelper.accessor('durationMs', {
      header: () => <TableHead className="text-right">Duration</TableHead>,
      cell: (props) => (
        <TableCell className="text-right">
          {`${props.getValue() / 1000 / 60}:${((props.getValue() / 1000) % 60).toString().padStart(2, '0')}`}
        </TableCell>
      ),
    }),
    columnHelper.accessor('urls', {
      header: () => <TableHead className="text-right">Links</TableHead>,
      cell: (props) => (
        <TableCell className="text-right">
          <div className="flex justify-end gap-1">
            {Object.entries(props.getValue()).map(([provider, link]) => (
              <Button key={provider} variant="outline">
                <Link target="_blank" href={link}>
                  {providersIcon[provider]}
                </Link>
              </Button>
            ))}
          </div>
        </TableCell>
      ),
    }),
  ];

  const data: Track[] = [
    {
      ids: {
        spotify: 'key1',
      },
      coverImg:
        'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
      album: 'MUTT',
      artist: 'Leon Thomas',
      durationMs: 180000,
      title: 'MUTT',
      urls: {
        spotify: 'https://www.spotify.com',
        deezer: 'https://www.deezer.com',
      },
    },
  ];

  // TODO Put table in a child client component
  // TODO Encapsulate state management into a provider

  const tracksTable = useReactTable({
    columns,
    data,
    getCoreRowModel: getCoreRowModel(),
    getRowId: (track) => `${track.title} - ${track.artist} - ${track.album}`,
  });

  return (
    <div className="flex flex-col gap-12 p-8">
      <div className="flex flex-col gap-2">
        <h2 className="font-heading text-3xl">Playlist : Test</h2>
        <h3 className="text-primary/80">50 songs</h3>
      </div>
      {/* Action bar */}
      <div className="flex gap-4 justify-start">
        <Button>Transfer all tracks to ...</Button>
      </div>

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
              <TableRow key={row.id}>
                {row.getVisibleCells().map((cell) => {
                  return (
                    <React.Fragment key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext(),
                      )}
                    </React.Fragment>
                  );
                })}
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </div>
  );
}
