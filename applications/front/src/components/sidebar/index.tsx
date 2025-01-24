'use client';

import { Button } from '../ui/button';
import AddIcon from '@/lib/theme/assets/icons/add.svg';
import { Suspense, useState } from 'react';
import ConnectAccountModal from '../modals/connect-account-modal';
import { Skeleton } from '../ui/skeleton';
import SidebarItem from './sidebar-item';

export default function Sidebar() {
  const providers = ['spotify', 'deezer'];
  const [open, setOpen] = useState(false);

  const Loader = () => (
    <div className="flex flex-col gap-3">
      <Skeleton className="w-full aspect-square" />
      <Skeleton className="w-full aspect-square" />
    </div>
  );

  return (
    <div className="sidebar flex flex-col justify-between">
      <Suspense fallback={<Loader />}>
        <div className="flex flex-col gap-3">
          {providers.map((provider) => (
            <SidebarItem key={provider} providerName={provider} />
          ))}
        </div>
      </Suspense>

      <Button
        variant="provider"
        size="provider"
        className="border border-primary hover:bg-primary/50"
        onClick={() => setOpen(true)}
      >
        <AddIcon className="!w-5 !h-5" />
      </Button>
      <ConnectAccountModal open={open} onOpenChange={setOpen} />
    </div>
  );
}
