'use client';

import Link from 'next/link';
import { Button } from './ui/button';
import AddIcon from '@/lib/theme/assets/icons/add.svg';
import { useState } from 'react';
import ConnectAccountModal from './modals/connect-account-modal';

export default function Sidebar() {
  const [open, setOpen] = useState(false);

  return (
    <div className="sidebar flex flex-col justify-between">
      <div className="flex flex-col gap-3">
        <Link href="/dashboard/spotify">
          <Button
            variant="provider"
            size="provider"
            className="bg-spotify hover:bg-spotify/80"
          >
            S
          </Button>
        </Link>

        <Link href="/dashboard/deezer">
          <Button
            variant="provider"
            size="provider"
            className="bg-deezer hover:bg-deezer/80"
          >
            D
          </Button>
        </Link>
      </div>

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
