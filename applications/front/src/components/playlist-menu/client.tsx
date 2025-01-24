'use client';

import Link from 'next/link';
import { Playlist } from '.';
import PlaylistMenuItem from './playlist-menu-item';

export default function PlaylistMenuClient({
  provider,
  playlists,
}: {
  provider: string;
  playlists: Playlist[];
}) {
  return (
    <div className="grow flex basis-0 flex-col max-h-full overflow-auto gap-3 cursor-pointer">
      {playlists.map((playlist) => (
        <Link key={playlist.id} href={`/dashboard/${provider}/${playlist.id}`}>
          <PlaylistMenuItem
            provider={provider}
            playlistId={playlist.id}
            name={playlist.name}
            cover={playlist.cover}
            nbSongs={playlist.nbSongs}
          />
        </Link>
      ))}
    </div>
  );
}
