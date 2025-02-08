'use client';

import { useState } from 'react';

import CreatePlaylistModal from '../modals/create-playlist-modal';
import AddIcon from '@/lib/theme/assets/icons/add.svg';
import { Button } from '../ui/button';

export default function PlaylistMenuAction({
  provider: _provider,
}: {
  provider: string;
}) {
  const [open, setOpen] = useState(false);

  return (
    <>
      <CreatePlaylistModal open={open} onOpenChange={setOpen} />
      <div className="flex">
        <Button onClick={() => setOpen(true)}>
          <AddIcon />
          New playlist
        </Button>
      </div>
    </>
  );
}
