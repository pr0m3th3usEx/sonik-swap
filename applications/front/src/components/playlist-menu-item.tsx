'use client';

import Image from 'next/image';
import Link from 'next/link';
import { usePathname } from 'next/navigation';

export default function PlaylistMenuItem() {
  const pathname = usePathname();

  return (
    <Link href={`${pathname}/playlistId`}>
      <div className="flex gap-3 w-full py-2 px-2 hover:bg-primary/10 rounded">
        <div className="w-12 aspect-square overflow-hidden rounded-sm">
          <Image
            src="https://image-cdn-ak.spotifycdn.com/image/ab67706c0000d72c785808b8933da7bde038e8a4"
            alt="Playlist logo"
            className="object-cover"
            width={128}
            height={128}
          />
        </div>
        <div className="grow flex flex-col gap-0">
          <p>Test</p>
          <p className="text-primary/50">10 songs</p>
        </div>
      </div>
    </Link>
  );
}
