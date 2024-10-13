import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { SearchBar } from './SearchBar'
import { TradContractsAccordion } from '../Accordion/TradContractsAccorrdion'
import { Stack, Text } from '@chakra-ui/react'

export const MainPage = () => {
  const { address } = useSorobanReact()
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)

  if (!address) return null;
  return (
    <>
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
      <SearchBar handleOpenDrawer={setIsDrawerOpen} />
      <Stack>
        <Text>Business account balance:</Text>
        <Text as={'b'} fontSize={'3xl'}>$ 1000</Text>
      </Stack>
      <TradContractsAccordion />
    </>
  )
}

export default MainPage
