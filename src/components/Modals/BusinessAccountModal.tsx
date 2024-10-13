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
} from '@chakra-ui/react'

const BusinessAccountModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} isCentered size={'full'}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Deposit to account</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <InputGroup>
              <InputLeftElement color={'grey.300'}>$</InputLeftElement>
              <Input>
              </Input>
            </InputGroup>
          </ModalBody>

          <ModalFooter>
            <Button colorScheme='blue' mr={3} onClick={onClose}>
              Close
            </Button>
            <Button variant='ghost'>Deposit</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
};

export default BusinessAccountModal;