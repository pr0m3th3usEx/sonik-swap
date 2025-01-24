'use client';

import { Track } from '@/app/dashboard/[provider]/[playlistId]/page';
import { ColumnDef, createColumnHelper } from '@tanstack/react-table';
import { Checkbox } from '../ui/checkbox';
import { TableCell, TableHead } from '../ui/table';
import Image from 'next/image';
import { Button } from '../ui/button';
import Link from 'next/link';
import DataTable from '../data-table';
import { useTrackSelection } from '../providers/track-selection-provider';
import SpotifyIcon from '@/lib/theme/assets/icons/spotify.svg';
import DeezerIcon from '@/lib/theme/assets/icons/deezer.svg';

const columnHelper = createColumnHelper<Track>();

const providersIcon: Record<string, React.ReactNode> = {
  spotify: <SpotifyIcon />,
  deezer: <DeezerIcon />,
};
export default function PlaylistManagerTracksSelector() {
  const { data } = useTrackSelection();

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

  return <DataTable columns={columns} data={data} />;
}
