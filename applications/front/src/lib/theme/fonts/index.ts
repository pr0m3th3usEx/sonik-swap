import localFont from 'next/font/local';
import { Poppins } from 'next/font/google';

export const poppins = Poppins({
  variable: '--body-font',
  weight: ['400', '500', '600', '700'],
  subsets: ['latin'],
  preload: true,
});

export const intro = localFont({
  variable: '--heading-font',
  src: [
    {
      path: './intro/IntroRegular.otf',
      weight: '400',
      style: 'normal',
    },
    {
      path: './intro/IntroRegularItalic.otf',
      weight: '400',
      style: 'italic',
    },
    {
      path: './intro/IntroBold.otf',
      weight: '700',
      style: 'normal',
    },
    {
      path: './intro/IntroBoldItalic.otf',
      weight: '700',
      style: 'italic',
    },
    {
      path: './intro/IntroBlack.otf',
      weight: '900',
      style: 'normal',
    },
    {
      path: './intro/IntroBlackItalic.otf',
      weight: '900',
      style: 'italic',
    },
  ],
});
