'use client';

import { useState } from 'react';
import TransferModal from '../modals/transfer-modal';
import { Button } from '../ui/button';
import { Track } from '@/app/dashboard/[provider]/[playlistId]/page';
import {
  TrackSelectionContextProps,
  useTrackSelection,
} from '../providers/track-selection-provider';

export default function PlaylistManagerActionBar({
  provider,
  playlistId,
}: {
  provider: string;
  playlistId: string;
}) {
  const [open, setOpen] = useState(false);

  const { nbRowsSelected: nbTracksSelected, setRowSelection } =
    useTrackSelection() as TrackSelectionContextProps<Track>;

  return (
    <div className="flex gap-4 justify-start">
      <TransferModal
        open={open}
        onOpenChange={setOpen}
        {...{ provider, playlistId }}
      />

      {nbTracksSelected === 0 ? (
        <Button onClick={() => setOpen(true)}>
          Transfer ALL tracks to ...
        </Button>
      ) : (
        <>
          <Button onClick={() => setOpen(true)}>
            ({nbTracksSelected}) Transfer tracks to ...
          </Button>
          <Button
            variant="link"
            className="text-red-400 hover:text-red-400/70"
            onClick={() => setRowSelection({})}
          >
            Cancel selection
          </Button>
        </>
      )}
    </div>
  );
}
