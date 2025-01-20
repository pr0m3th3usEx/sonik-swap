import NextImage from 'next/image';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import SpotifyLogo from '@/lib/theme/assets/icons/spotify.svg';
import DeezerLogo from '@/lib/theme/assets/icons/deezer.svg';

export default function Home() {
  return (
    <div className={cn('w-screen h-screen', 'flex flex-col gap-12')}>
      <header className={cn('flex justify-center', 'py-5')}>
        <div className="flex gap-3 items-center">
          <NextImage
            src="logo/with_border.svg"
            alt="Sonik Swap Logo"
            width={48}
            height={48}
          />
          <h1 className={cn('font-bold text-3xl')}>SonikSwap</h1>
        </div>
      </header>
      <main className="flex-grow flex flex-col justify-center items-center gap-12 px-14 uppercase">
        <h2 className={cn('text-center text-5xl')}>
          Transfer your Spotify favourite tracks & playlists to Deezer and
          vice-versa
        </h2>

        <div className="flex flex-col gap-3">
          <Button
            variant="spotify"
            size="lg"
            className="font-semibold flex gap-4"
          >
            <SpotifyLogo className="!w-6 !h-6" />
            Login with Spotify
          </Button>
          <Button
            variant="deezer"
            size="lg"
            className="font-semibold flex gap-4"
          >
            <DeezerLogo className="!w-6 !h-6" />
            Login with Deezer
          </Button>
        </div>
      </main>
      <footer className="flex justify-center py-5">
        <p className="text-foreground/90">
          Made with ❤️ by{' '}
          <a className="hover:underline" href="https://github.com/pr0m3th3usex">
            pr0m3th3usEx
          </a>
        </p>
      </footer>
    </div>
  );
}
