import { cn } from '@/lib/utils';
import Image from 'next/image';

const style: Record<'default' | 'sm' | 'lg', { icon: string; title: string }> =
  {
    default: {
      icon: 'w-8 h-8',
      title: 'text-2xl',
    },
    sm: {
      icon: 'w-6 h-6',
      title: 'text-xl',
    },
    lg: {
      icon: 'w-12 h-12',
      title: 'text-3xl',
    },
  };

export default function Logo({
  size = 'default',
}: {
  size?: 'default' | 'sm' | 'lg';
}) {
  return (
    <div className="flex gap-3 items-center">
      <Image
        src="/logo/with_border.svg"
        alt="Sonik Swap Logo"
        className={style[size].icon}
        width={48}
        height={48}
      />
      <h1 className={cn('font-bold font-heading', style[size].title)}>
        SonikSwap
      </h1>
    </div>
  );
}
