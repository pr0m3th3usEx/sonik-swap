'use client';

import { Track } from '@/app/dashboard/[provider]/[playlistId]/page';
import { useTrackSelection } from '../providers/track-selection-provider';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '../ui/dialog';
import PlaylistMenuItem from '../playlist-menu/playlist-menu-item';
import { Button } from '../ui/button';
import { useState } from 'react';

export default function TransferModal({
  open,
  onOpenChange,
  provider,
  playlistId,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  provider: string;
  playlistId: string;
}) {
  const { dataSelected: tracksSelected, nbRowsSelected } =
    useTrackSelection() as { dataSelected: Track[]; nbRowsSelected: number };

  const [playlistSelected, setPlaylistSelected] = useState<{
    playlistId: string;
    provider: string;
  }>();

  const spotifyPlaylists = [
    {
      id: '1',
      name: 'Test',
      nbSongs: 10,
      cover:
        'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
    },
    {
      id: '2',
      name: 'Test',
      nbSongs: 12,
      cover:
        'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
    },
    {
      id: '3',
      name: 'Test',
      nbSongs: 15,
      cover:
        'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
    },
  ];

  const onSubmit = () => {
    // TODO Transfer selected tracks to selected playlist
    if (!playlistSelected) return;

    console.log(tracksSelected, playlistSelected);
  };

  const onChangeVisibility = (o: boolean) => {
    if (!o) setPlaylistSelected(undefined);

    onOpenChange(o);
  };
  return (
    <Dialog open={open} onOpenChange={onChangeVisibility}>
      <DialogContent className="gap-6">
        <DialogHeader>
          <DialogTitle>
            Transfers {nbRowsSelected === 0 ? 'all' : nbRowsSelected} tracks
          </DialogTitle>
          <DialogDescription>
            Where do you want to transfer the tracks of your playlist ?
          </DialogDescription>
        </DialogHeader>
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-3">
            <h3>Spotify</h3>

            {spotifyPlaylists.map((playlist) => {
              if (playlist.id === playlistId && provider === 'spotify')
                return null;

              return (
                <PlaylistMenuItem
                  key={playlist.id}
                  provider="spotify"
                  playlistId={playlist.id}
                  name={playlist.name}
                  cover={playlist.cover}
                  nbSongs={playlist.nbSongs}
                  onClick={setPlaylistSelected}
                  isSelected={
                    playlistSelected?.provider === 'spotify' &&
                    playlistSelected.playlistId === playlist.id
                  }
                />
              );
            })}
          </div>
        </div>
        <DialogFooter className="sm:justify-end">
          <Button disabled={!playlistSelected} onClick={onSubmit}>
            Transfer tracks
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
