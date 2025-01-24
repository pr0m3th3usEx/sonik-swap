import { MenuIcon } from 'lucide-react';
import Logo from './logo';
import { Button } from './ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from './ui/dropdown-menu';
import Link from 'next/link';

export default function Navbar() {
  return (
    <header className="flex justify-center px-4 py-4 h-24">
      <div className="flex justify-between items-center w-full py-4 px-4 rounded-sm bg-accent/80 shadow-md">
        <Link href="/">
          <Logo size="sm" />
        </Link>

        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" asChild>
              <div className="cursor-pointer w-8 h-8 bg-accent hover:bg-accent !rounded-full border-2 border-primary hover:rotate-180 transition-transform duration-700 shadow-md">
                <MenuIcon className="text-primary" />
              </div>
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-56">
            <DropdownMenuItem>My account</DropdownMenuItem>
            <DropdownMenuItem>Log out</DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </header>
  );
}
