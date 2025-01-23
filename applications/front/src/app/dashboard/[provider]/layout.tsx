import PlaylistMenu from '@/components/playlist-menu';
import { Suspense } from 'react';

export default async function PlaylistSelectionLayout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ provider: string }>;
}) {
  const { provider } = await params;

  return (
    <div className="max-h-full h-full flex gap-4">
      <Suspense>
        <PlaylistMenu provider={provider} />
      </Suspense>

      <div className="grow h-full">{children}</div>
    </div>
  );
}
