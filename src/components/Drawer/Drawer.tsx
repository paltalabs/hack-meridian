import { shortenAddress } from '@/utils/shortenAdress'
import {
  Avatar,
  Button,
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  Grid,
  GridItem,
  Stack,
  StackDivider
} from '@chakra-ui/react'
import { AddIcon, MinusIcon } from '@chakra-ui/icons'
import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import BusinessAccountModal from '../Modals/BusinessAccountModal'

export const ProfileDrawer = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void; }) => {
  const { address, disconnect } = useSorobanReact()
  const [isAccountModalOpen, setIsAccountModalOpen] = useState<boolean>(false)
  const [isDeposit, setIsDeposit] = useState(true);


  if (!address) return null;
  return (
    <>
      <BusinessAccountModal isOpen={isAccountModalOpen} onClose={() => { setIsAccountModalOpen(false) }} isDeposit={isDeposit} />
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
            <Stack spacing={0} >
              <Button
                leftIcon={<AddIcon />}
                variant={'outline'}
                minW={'100%'}
                justifyContent={'left'}
                pl={4} py={8}
                onClick={() => {
                  setIsAccountModalOpen(true)
                  setIsDeposit(true)
                }}
              >
                Deposit to Business account
              </Button>
              <Button
                leftIcon={<MinusIcon />}
                variant={'outline'}
                minW={'100%'}
                justifyContent={'left'}
                pl={4} py={8}
                onClick={() => {
                  setIsAccountModalOpen(true)
                  setIsDeposit(false)
                }}
              >
                Withdraw from Business account
              </Button>
            </Stack>
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
