import type { JSX } from 'react';
import {
  ActionIcon,
  useMantineColorScheme,
  useComputedColorScheme,
} from '@mantine/core';
import { SunIcon, MoonIcon } from "lucide-react";
import cx from 'clsx';
import classes from "./theme-btn.module.css";

export default function ThemeSwitchBtn(): JSX.Element {
  const { setColorScheme } = useMantineColorScheme()
  const computedColorScheme = useComputedColorScheme('dark', {
    getInitialValueInEffect: true,
  })

  return (
    <ActionIcon
      onClick={() => {
        setColorScheme(computedColorScheme === 'light' ? 'dark' : 'light')
      }}
      variant="default"
      size="lg"
      aria-label="Toggle color scheme"
    >
      <SunIcon className={cx(classes.icon, classes.light)} />
      <MoonIcon className={cx(classes.icon, classes.dark)} />
    </ActionIcon>
  )
}

