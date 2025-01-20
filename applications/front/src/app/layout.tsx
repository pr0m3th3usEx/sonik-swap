import { ThemeProvider } from '@/components/providers/theme-provider';
import './globals.css';
import { intro, poppins } from '@/lib/theme/fonts';
import { cn } from '@/lib/utils';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <>
      <html
        lang="en"
        suppressHydrationWarning
        className={cn(intro.variable, poppins.variable)}
      >
        <head />
        <body>
          <ThemeProvider
            attribute="class"
            defaultTheme="system"
            enableSystem
            disableTransitionOnChange
          >
            {children}
          </ThemeProvider>
        </body>
      </html>
    </>
  );
}
