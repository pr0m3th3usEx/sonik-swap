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

export default function TransferModal({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  // const { dataSelected: tracksSelected, nbRowsSelected } =
  //   useTrackSelection() as { dataSelected: Track[]; nbRowsSelected: number };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="gap-6">
        <DialogHeader>
          <DialogTitle>Transfers tracks</DialogTitle>
          <DialogDescription>
            Where do you want to transfer the tracks of your playlist ?
          </DialogDescription>
        </DialogHeader>
        <div className="flex flex-col gap-4">
          <div className="flex flex-col gap-3">
            <h3>Spotify</h3>

            <PlaylistMenuItem />
            <PlaylistMenuItem />
            <PlaylistMenuItem />
          </div>

          <div className="flex flex-col gap-3">
            <h3>Deezer</h3>

            <PlaylistMenuItem />
            <PlaylistMenuItem />
            <PlaylistMenuItem />
          </div>
        </div>
        <DialogFooter className="sm:justify-end">
          <Button>Transfer tracks</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
