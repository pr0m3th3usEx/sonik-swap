import Background from '@/components/background';
import Navbar from '@/components/navbar';
import { cn } from '@/lib/utils';

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className={cn('w-screen h-screen', 'flex flex-col')}>
      <Background />
      <Navbar />

      <div className="flex grow gap-4 px-4 pb-4">
        {/* Side bar provider */}
        <div
          className={cn(
            'sidebar',
            // "after:content-[''] after:w-full after:h-full after:p-14 after:absolute after:-z-10 after:bg-red-500 after:top-1/2 after:left-1/2 after:-translate-x-1/2 after:-translate-y-1/2",
          )}
        >
          <p>Test</p>
        </div>

        {/* Content */}
        <div className="grow h-full">{children}</div>
      </div>
    </div>
  );
}
