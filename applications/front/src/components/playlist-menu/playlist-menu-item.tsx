import { cn } from '@/lib/utils';
import Image from 'next/image';

type PlaylistMenuItemProps = {
  provider: string;
  playlistId: string;
  name: string;
  nbSongs: number;
  cover: string;
  isSelected?: boolean;
  onClick?: ({
    provider,
    playlistId,
  }: {
    provider: string;
    playlistId: string;
  }) => void;
};

export default function PlaylistMenuItem({
  provider,
  playlistId,
  name,
  nbSongs,
  cover,
  isSelected,
  onClick,
}: PlaylistMenuItemProps) {
  const onSelect = () => {
    if (onClick) onClick({ provider, playlistId });
  };
  return (
    <div
      className={cn(
        'cursor-pointer flex gap-3 w-full py-2 px-2 hover:bg-primary/10 rounded',
        ...(isSelected ? ['bg-primary/10'] : []),
      )}
      onClick={onSelect}
    >
      <div className="w-12 aspect-square overflow-hidden rounded-sm">
        <Image
          src={cover}
          alt="Playlist logo"
          className="object-cover"
          width={128}
          height={128}
        />
      </div>
      <div className="grow flex flex-col gap-0">
        <p>{name}</p>
        <p className="text-primary/50">{nbSongs} songs</p>
      </div>
    </div>
  );
}
