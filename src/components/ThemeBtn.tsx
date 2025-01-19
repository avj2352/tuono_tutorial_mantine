import type { JSX } from 'react'
import {
  ActionIcon,
  useMantineColorScheme,
  useComputedColorScheme,
} from '@mantine/core'
import { SunIcon, MoonIcon } from "lucide-react";

export default function ThemeBtn(): JSX.Element {
  const { setColorScheme } = useMantineColorScheme()
  const computedColorScheme = useComputedColorScheme('light', {
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
    {computedColorScheme === 'light' ? <MoonIcon/> : <SunIcon/>}
    </ActionIcon>
  )
}

