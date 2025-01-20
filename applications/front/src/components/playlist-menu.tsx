import PlaylistMenuItem from './playlist-menu-item';

export default function PlaylistMenu() {
  return (
    <div className="flex flex-col w-96 h-full overflow-scroll px-2 py-5 rounded-sm bg-accent gap-6 shadow-md animate-fade-slide">
      <h3 className="uppercase font-heading px-1 text-lg">Your playlists</h3>
      <div className="grow flex basis-0 flex-col max-h-full overflow-auto gap-3 cursor-pointer">
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
        <PlaylistMenuItem />
      </div>
    </div>
  );
}
