import Background from '@/components/background';
import Logo from '@/components/logo';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from '@/components/ui/dropdown-menu';
import MenuIcon from '@/lib/theme/assets/icons/menu.svg';
import { cn } from '@/lib/utils';

export default function DashboardMainContent() {
  return (
    <div className={cn('w-screen h-screen', 'flex flex-col')}>
      <Background />
      <header className="flex justify-center px-4 py-4 h-24">
        <div className="flex justify-between items-center w-full py-4 px-4 rounded-sm bg-accent/80 shadow-md">
          <Logo size="sm" />

          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" asChild>
                <div className="cursor-pointer w-8 h-8 bg-accent hover:bg-background rounded-full border-2 border-primary hover:rotate-180 transition-all duration-700 shadow-md">
                  <MenuIcon className="text-primary" />
                </div>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-56">
              <DropdownMenuItem>Log out</DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </header>
    </div>
  );
}
