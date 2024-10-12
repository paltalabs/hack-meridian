import { shortenAddress } from '@/utils/shortenAdress'
import { Avatar, Button, Drawer, DrawerBody, DrawerContent, DrawerHeader, DrawerOverlay, Grid, GridItem } from '@chakra-ui/react'
import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'

export const MainPage = () => {
  const { address } = useSorobanReact()
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(true)

  if (!address) return null;
  return (
    <>
      <Drawer placement={'left'} onClose={() => setIsDrawerOpen(false)} isOpen={isDrawerOpen}>
        <DrawerOverlay />
        <DrawerContent>
          <DrawerHeader borderBottomWidth='1px'>
            <Grid templateColumns={'repeat(12, 1fr)'}>
              <GridItem colSpan={8}>
                <h2>Business Name</h2>
                <small>{shortenAddress(address, 4)}</small>
              </GridItem>
              <GridItem colSpan={4} alignContent={'center'} textAlign={'center'}>
                <Avatar name='Jhon Doe' src='https://static.vecteezy.com/system/resources/previews/004/511/281/original/default-avatar-photo-placeholder-profile-picture-vector.jpg' />
              </GridItem>
            </Grid>
          </DrawerHeader>
          <DrawerBody p={0} >
            <Button minW={'100%'} textAlign={'left'} pl={0}>
              + Deposit to Business account
            </Button>
            <Button minW={'100%'} textAlign={'left'} pl={0}>
              - Withdraw from Business account
            </Button>
          </DrawerBody>
        </DrawerContent>
      </Drawer>
    </>
  )
}

export default MainPage
