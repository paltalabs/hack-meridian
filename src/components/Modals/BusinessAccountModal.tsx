import React from 'react';
import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  Button,
  Input,
  InputGroup,
  InputLeftElement,
  NumberInput,
  NumberInputField,
} from '@chakra-ui/react'
import { BusinessAccountOptions } from '../Drawer/Drawer';

const BusinessAccountModal = ({ isOpen, onClose, method }: { isOpen: boolean, onClose: () => void, method: BusinessAccountOptions | undefined; }) => {
  if (!method) return null;
  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} isCentered size={'full'}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader textAlign={'center'} pt={12}>{method === BusinessAccountOptions.DEPOSIT ? 'Deposit to' : 'Withdraw from'} account</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <InputGroup>
              <InputLeftElement color={'grey.300'}>$</InputLeftElement>
              <Input type='number' />
            </InputGroup>
          </ModalBody>

          <ModalFooter justifyContent={'center'} pb={12}>
            <Button
              variant='solid'
              colorScheme='green'
              rounded={18}
            >
              {method.charAt(0).toUpperCase() + method.slice(1)}
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
};

export default BusinessAccountModal;