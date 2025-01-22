'use client';

import { useState } from 'react';
import CreatePlaylistModal from './modals/create-playlist-modal';
import PlaylistMenuItem from './playlist-menu-item';
import { Button } from './ui/button';
import AddIcon from '@/lib/theme/assets/icons/add.svg';

export default function PlaylistMenu() {
  const [open, setOpen] = useState(false);

  return (
    <div className="flex flex-col w-96 h-full overflow-scroll px-3 py-5 rounded-sm bg-accent gap-6 shadow-md animate-fade-slide">
      <div className="flex justify-between items-center">
        <h3 className="uppercase font-heading px-1 text-lg">Your playlists</h3>

        <div className="flex">
          <Button onClick={() => setOpen(true)}>
            <AddIcon />
            New playlist
          </Button>
        </div>
      </div>

      <div className="grow flex basis-0 flex-col max-h-full overflow-auto gap-3 cursor-pointer">
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
      </div>
      <CreatePlaylistModal open={open} onOpenChange={setOpen} />
    </div>
  );
}
