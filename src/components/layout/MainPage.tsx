import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { SearchBar } from './SearchBar'
import { TradContractsAccordion } from '../Accordion/TradContractsAccorrdion'

export const MainPage = () => {
  const { address } = useSorobanReact()
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)
  const contracts = ['GC2VCACI4VCD3RQF3JNQNYYCGXAMZRQ3LKF3Q5425W7RIY6PR2UUNNEB', 'GC2VCACI4VCD3RQF3JNQNYYCGXAMZRQ3LKF3Q5425W7RIY6PR2UUNNEB', 'GC2VCACI4VCD3RQF3JNQNYYCGXAMZRQ3LKF3Q5425W7RIY6PR2UUNNEB']

  if (!address) return null;
  return (
    <>
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
      <SearchBar handleOpenDrawer={setIsDrawerOpen} />

      <TradContractsAccordion />
    </>
  )
}

export default MainPage
