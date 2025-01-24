import AccountConnectionManagerListItem from '../account-connection-item';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '../ui/dialog';

export default function ConnectAccountModal({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="gap-6">
        <DialogHeader>
          <DialogTitle>Add music accounts</DialogTitle>
          <DialogDescription>
            Link your music accounts and start transfering your favourite
            playlists & tracks
          </DialogDescription>
        </DialogHeader>
        <div className="grid grid-cols-4 flex-col gap-4">
          <AccountConnectionManagerListItem />
          <AccountConnectionManagerListItem />
        </div>
      </DialogContent>
    </Dialog>
  );
}
