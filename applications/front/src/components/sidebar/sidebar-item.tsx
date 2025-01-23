import Link from 'next/link';
import { Button } from '../ui/button';

export default function SidebarItem({
  providerName,
}: {
  providerName: string;
}) {
  return (
    <Link href={`/dashboard/${providerName}`}>
      <Button
        variant="provider"
        size="provider"
        className={`bg-${providerName} hover:bg-${providerName}/80`}
      >
        {providerName[0].toUpperCase()}
      </Button>
    </Link>
  );
}
