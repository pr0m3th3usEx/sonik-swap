'use client';

import Image from 'next/image';
import Link from 'next/link';
import { createColumnHelper, ColumnDef } from '@tanstack/react-table';
import { Button } from '@/components/ui/button';
import { TableCell, TableHead } from '@/components/ui/table';
import SpotifyIcon from '@/lib/theme/assets/icons/spotify.svg';
import DeezerIcon from '@/lib/theme/assets/icons/deezer.svg';
import React, { useMemo, useState } from 'react';
import { Checkbox } from '@/components/ui/checkbox';
import DataTable from '@/components/data-table';
import TrackSelectionProvider from '@/components/providers/track-selection-provider';
import PlaylistManagerActionBar from '@/components/playlist-manager-actionbar';

export type Track = {
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
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const columns: ColumnDef<Track, any>[] = [
    columnHelper.display({
      id: 'select',
      header: () => <TableHead></TableHead>,
      cell: ({ row }) => (
        <TableCell>
          <div>
            <Checkbox
              checked={row.getIsSelected()}
              onCheckedChange={(value) => row.toggleSelected(!!value)}
              arial-label="select row"
              onClick={(e) => {
                e.stopPropagation();
              }}
            />
          </div>
        </TableCell>
      ),
    }),
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
                <Link target="_blank" href={link as string}>
                  {providersIcon[provider]}
                </Link>
              </Button>
            ))}
          </div>
        </TableCell>
      ),
    }),
  ];

  const data: Track[] = useMemo(
    () => [
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
      {
        ids: {
          spotify: 'key2',
        },
        coverImg:
          'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
        album: 'MUTT',
        artist: 'Leon Thomas',
        durationMs: 180000,
        title: 'ANSWER THE PHONE',
        urls: {
          spotify: 'https://www.spotify.com',
          deezer: 'https://www.deezer.com',
        },
      },
      {
        ids: {
          spotify: 'key3',
        },
        coverImg:
          'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
        album: 'MUTT',
        artist: 'Leon Thomas',
        durationMs: 180000,
        title: 'HOW FAST',
        urls: {
          spotify: 'https://www.spotify.com',
          deezer: 'https://www.deezer.com',
        },
      },
    ],
    [],
  );

  return (
    <TrackSelectionProvider>
      <div className="flex flex-col gap-12 p-8">
        <div className="flex flex-col gap-2">
          <h2 className="font-heading text-3xl">Playlist : Test</h2>
          <h3 className="text-primary/80">50 songs</h3>
        </div>
        <PlaylistManagerActionBar />
        <DataTable columns={columns} data={data} />
      </div>
    </TrackSelectionProvider>
  );
}
