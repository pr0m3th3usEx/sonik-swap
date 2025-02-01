import PlaylistMenuAction from './playlist-menu-action';
import PlaylistMenuClient from './client';

export type Playlist = {
  id: string;
  name: string;
  nbSongs: number;
  cover: string;
};

export default async function PlaylistMenu({ provider }: { provider: string }) {
  const playlists = await new Promise<Playlist[]>((resolve) => {
    setTimeout(() => {
      resolve([
        {
          id: '1',
          name: 'Test',
          nbSongs: 10,
          cover:
            'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
        },
        {
          id: '2',
          name: 'Test',
          nbSongs: 10,
          cover:
            'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
        },
        {
          id: '3',
          name: 'Test',
          nbSongs: 10,
          cover:
            'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
        },
      ]);
    }, 1500);
  });

  return (
    <div className="flex flex-col w-96 h-full overflow-scroll px-3 py-5 rounded-sm bg-accent gap-6 shadow-md animate-fade-slide">
      <div className="flex justify-between items-center">
        <h3 className="uppercase font-heading px-1 text-lg">Your playlists</h3>
        <PlaylistMenuAction provider={provider} />
      </div>

      <PlaylistMenuClient provider={provider} playlists={playlists} />
    </div>
  );
}
