import NextImage from 'next/image';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import SpotifyLogo from '@/lib/theme/assets/icons/spotify.svg';
import DeezerLogo from '@/lib/theme/assets/icons/deezer.svg';
import Background from '@/components/background';
import Logo from '@/components/logo';

export default function Home() {
  return (
    <div className={cn('w-screen h-screen', 'flex flex-col gap-12')}>
      <Background />
      <header className={cn('flex justify-center', 'py-5')}>
        <Logo size="lg" />
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
