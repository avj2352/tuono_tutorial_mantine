import type { JSX } from 'react'
import { AppShell, Box, Button, Flex } from '@mantine/core'
import { Link } from 'tuono'
import Actions from "@/components/Actions";


export default function Navbar(): JSX.Element {
  return (
    <AppShell.Header p="sm">
      <Flex justify="space-between">
        <Button
          component={Link}
          href="/"
          variant="transparent"
          p={0}
          fz={28}
        >
          Tuono Tutorial
        </Button>
        <Box />
        <Flex align="center" gap={8}>
          <Actions />
        </Flex>
      </Flex>
    </AppShell.Header>
  )
}

