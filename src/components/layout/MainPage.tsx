import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { SearchBar } from './SearchBar'
import { TradContractsAccordion } from '../Accordion/TradContractsAccorrdion'
import { Avatar, Button, Input, InputGroup, InputLeftElement, Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay, NumberDecrementStepper, NumberIncrementStepper, NumberInput, NumberInputField, NumberInputStepper, Select, Stack, Text } from '@chakra-ui/react'
import { AddIcon } from '@chakra-ui/icons'
import { CreateContractModal } from '../Modals/CreateContractModal'

export const MainPage = () => {
  const { address } = useSorobanReact()
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)
  const [isCreateContractModalOpen, setIsCreateContractModalOpen] = useState<boolean>(true) 

  if (!address) return null;
  return (
    <>
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
      <SearchBar handleOpenDrawer={setIsDrawerOpen} handleCreateContract={setIsCreateContractModalOpen} />
      <CreateContractModal isOpen={isCreateContractModalOpen} onClose={setIsCreateContractModalOpen} />
      <Stack>
        <Text>Business account balance:</Text>
        <Text as={'b'} fontSize={'3xl'}>$ 1000</Text>
      </Stack>
      <TradContractsAccordion />
    </>
  )
}

export default MainPage
