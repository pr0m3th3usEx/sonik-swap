import Background from '@/components/background';
import Navbar from '@/components/navbar';
import { cn } from '@/lib/utils';
import Sidebar from '@/components/sidebar';

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
        <Sidebar />

        {/* Content */}
        <div className="grow h-full">{children}</div>
      </div>
    </div>
  );
}
