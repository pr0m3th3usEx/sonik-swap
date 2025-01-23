'use client';

import Image from 'next/image';
import Link from 'next/link';
import { useParams } from 'next/navigation';

type PlaylistMenuItemProps = {
  playlistId: string;
  name: string;
  nbSongs: number;
  cover: string;
};

export default function PlaylistMenuItem({
  playlistId,
  name,
  nbSongs,
  cover,
}: PlaylistMenuItemProps) {
  const { provider } = useParams<{ provider: string }>();

  return (
    <Link href={`/dashboard/${provider}/${playlistId}`}>
      <div className="flex gap-3 w-full py-2 px-2 hover:bg-primary/10 rounded">
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
    </Link>
  );
}
