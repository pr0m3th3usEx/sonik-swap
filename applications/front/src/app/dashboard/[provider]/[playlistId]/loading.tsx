import { Loader2 } from 'lucide-react';

export default function PlaylistTracksLoading() {
  return (
    <div className="flex justify-center items-center h-full">
      <Loader2 className="animate-spin" />
    </div>
  );
}
