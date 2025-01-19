import type { CSSVariablesResolver } from '@mantine/core';
import { createTheme, MantineColorsTuple } from '@mantine/core';

const myColor: MantineColorsTuple = [
  '#faedff',
  '#edd9f7',
  '#d8b1ea',
  '#c186dd',
  '#ae62d2',
  '#a34bcb',
  '#9d3fc9',
  '#8931b2',
  '#7a2aa0',
  '#6b218d'
];

export const theme = createTheme({
  colors: {
    myColor,
  }
});


export const resolver: CSSVariablesResolver = (th) => ({
  variables: {},
  light: {
    '--mantine-color-sidebar-gray': th.other.sidebarGrayLight as string,
    '--mantine-color-sidebar-text-hover': th.other
      .sidebarTextHoverLight as string,
  },
  dark: {
    '--mantine-color-sidebar-gray': th.other.sidebarGrayDark as string,
    '--mantine-color-sidebar-text-hover': th.other
      .sidebarTextHoverDark as string,
  },
});
