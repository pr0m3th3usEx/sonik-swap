import TrackSelectionProvider from '@/components/providers/track-selection-provider';
import PlaylistManagerActionBar from '@/components/playlist-menu/playlist-manager-actionbar';
import PlaylistManagerTracksSelector from '@/components/playlist-manager/tracks-selector';

export type Track = {
  ids: Record<string, string>;
  coverImg: string;
  title: string;
  artist: string;
  durationMs: number;
  album: string;
  urls: Record<string, string>;
};

export default async function PlaylistManagerPage({
  params,
}: {
  params: Promise<{ provider: string; playlistId: string }>;
}) {
  const { provider, playlistId } = await params;
  const data = await new Promise<Track[]>((resolve) =>
    setTimeout(
      () =>
        resolve([
          {
            ids: {
              spotify: 'key1',
            },
            coverImg:
              'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
            album: 'MUTT',
            artist: 'Leon Thomas',
            durationMs: 180000,
            title: 'MUTT',
            urls: {
              spotify: 'https://www.spotify.com',
              deezer: 'https://www.deezer.com',
            },
          },
          {
            ids: {
              spotify: 'key2',
            },
            coverImg:
              'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
            album: 'MUTT',
            artist: 'Leon Thomas',
            durationMs: 180000,
            title: 'ANSWER THE PHONE',
            urls: {
              spotify: 'https://www.spotify.com',
              deezer: 'https://www.deezer.com',
            },
          },
          {
            ids: {
              spotify: 'key3',
            },
            coverImg:
              'https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4',
            album: 'MUTT',
            artist: 'Leon Thomas',
            durationMs: 180000,
            title: 'HOW FAST',
            urls: {
              spotify: 'https://www.spotify.com',
              deezer: 'https://www.deezer.com',
            },
          },
        ]),
      1500,
    ),
  );

  return (
    <TrackSelectionProvider data={data}>
      <div className="flex flex-col gap-12 p-8">
        <div className="flex flex-col gap-2">
          <h2 className="font-heading text-3xl">Playlist : Test</h2>
          <h3 className="text-primary/80">50 songs</h3>
        </div>
        <PlaylistManagerActionBar {...{ provider, playlistId }} />
        <PlaylistManagerTracksSelector />
      </div>
    </TrackSelectionProvider>
  );
}
