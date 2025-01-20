import { Button } from './ui/button';
import AddIcon from '@/lib/theme/assets/icons/add.svg';

export default function Sidebar() {
  return (
    <div className="sidebar flex flex-col justify-between">
      <div className="flex flex-col gap-3">
        <Button
          variant="provider"
          size="provider"
          className="bg-spotify hover:bg-spotify/80"
        >
          S
        </Button>

        <Button
          variant="provider"
          size="provider"
          className="bg-deezer hover:bg-deezer/80"
        >
          D
        </Button>
      </div>

      <Button
        variant="provider"
        size="provider"
        className="border border-primary hover:bg-primary/50"
      >
        <AddIcon className="!w-5 !h-5" />
      </Button>
    </div>
  );
}
