import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { Avatar, Grid, GridItem, IconButton, Input, InputGroup, InputLeftElement, InputRightElement } from '@chakra-ui/react'
import { AddIcon, Search2Icon } from '@chakra-ui/icons'
import { SearchBar } from './SearchBar'

export const MainPage = () => {
  const { address } = useSorobanReact()
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)

  if (!address) return null;
  return (
    <>
      <SearchBar handleOpenDrawer={setIsDrawerOpen} />
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
    </>
  )
}

export default MainPage
