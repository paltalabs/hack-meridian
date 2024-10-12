import { shortenAddress } from '@/utils/shortenAdress'
import { Avatar, Button, Drawer, DrawerBody, DrawerContent, DrawerFooter, DrawerHeader, DrawerOverlay, Grid, GridItem } from '@chakra-ui/react'
import { AddIcon, MinusIcon } from '@chakra-ui/icons'
import { useSorobanReact } from '@soroban-react/core'
import React from 'react'

export const ProfileDrawer = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void; }) => {
  const { address, disconnect } = useSorobanReact()

  if (!address) return null;
  return (
    <>
      <Drawer placement={'left'} onClose={onClose} isOpen={isOpen}>
        <DrawerOverlay backdropFilter={'blur(10px)'} />
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
            <Button leftIcon={<AddIcon />} minW={'100%'} justifyContent={'left'} pl={4} py={8}>
              Deposit to Business account
            </Button>
            <Button leftIcon={<MinusIcon />} minW={'100%'} justifyContent={'left'} pl={4} py={8}>
              Withdraw from Business account
            </Button>
          </DrawerBody>
          <DrawerFooter justifyContent={'center'}>
            <Button sx={{ mx: 4, px: 6 }} colorScheme='green' onClick={disconnect} rounded={18} mb={{ base: 4, md: 0 }}>
              Logout
            </Button>
          </DrawerFooter>
        </DrawerContent>
      </Drawer>
    </>
  )
}
