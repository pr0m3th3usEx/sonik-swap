import PlaylistMenu from '@/components/playlist-menu';

export default function PlaylistSelectionLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="max-h-full h-full flex gap-4">
      <PlaylistMenu />

      <div className="grow h-full">{children}</div>
    </div>
  );
}
