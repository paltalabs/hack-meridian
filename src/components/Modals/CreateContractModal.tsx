import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { TradContractsAccordion } from '../Accordion/TradContractsAccorrdion'
import { Avatar, Button, Input, InputGroup, InputLeftElement, Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay, NumberDecrementStepper, NumberIncrementStepper, NumberInput, NumberInputField, NumberInputStepper, Select, Stack, Text } from '@chakra-ui/react'
import { AddIcon } from '@chakra-ui/icons'

export const CreateContractModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: (value: boolean) => void }) => {
  const { address } = useSorobanReact()
  const [isCreateContractModalOpen, setIsCreateContractModalOpen] = useState<boolean>(true)

  if (!address) return null;
  return (
    <>
      <Modal
        isOpen={isOpen}
        onClose={() => onClose(false)}
        isCentered
        size={'full'}
      >
        <ModalOverlay />
        <ModalContent>
          <ModalHeader textAlign={'center'} pt={12}>
            <Avatar size='lg' name='Dan Abrahmov' src='https://bit.ly/dan-abramov' />
          </ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <Stack spacing={4}>

              <Input placeholder='Full name' variant={'outline'} ></Input>
              <Input placeholder='Stellar address' variant={'outline'} ></Input>
              <Select placeholder='Select option' variant={'outline'} >
                <option value='option1'>Weekly</option>
                <option value='option2'>Monthly</option>
                <option value='option3'>Yearly</option>
              </Select>
              <NumberInput min={0}>
                <NumberInputField />
                <NumberInputStepper>
                  <NumberIncrementStepper />
                  <NumberDecrementStepper />
                </NumberInputStepper>
              </NumberInput>
              <InputGroup>
                <InputLeftElement color='gray.300' fontSize='1.2em'>
                  $
                </InputLeftElement>
                <Input placeholder='Salary' variant={'outline'} />
              </InputGroup>
            </Stack>
          </ModalBody>

          <ModalFooter justifyContent={'center'} pb={12}>
            <Button
              variant='solid'
              colorScheme='green'
              leftIcon={<AddIcon />}
              onClick={() => onClose(false)}
            >Add employee</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}
