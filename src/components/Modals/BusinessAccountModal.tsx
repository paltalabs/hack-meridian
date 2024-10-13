import React, { useState } from 'react';
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

import { Address, nativeToScVal } from '@stellar/stellar-sdk'
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll'
import { useSorobanReact } from '@soroban-react/core';
import { fetchPayrollAddress } from '@/utils/payrollVault';

const BusinessAccountModal = ({ isOpen, onClose, isDeposit }: { isOpen: boolean, onClose: () => void, isDeposit: boolean }) => {

  const sorobanContext = useSorobanReact()
  const { address, activeChain } = sorobanContext
  console.log('🚀 ~ BusinessAccountModal ~ isDeposit:', isDeposit);
  const [amount, setAmount] = useState(0)

  const invokePayrollVault = usePayrollVaultCallback()

  const invoke = () => {
    if (!address || !activeChain) return;
    const caller = new Address(address);
    const employer = new Address(address);
    const amountScval = nativeToScVal(amount * 10000000, { type: "i128" })
    const payrollAddress = fetchPayrollAddress(activeChain?.id)

    const depositParams = [
      caller.toScVal(),
      employer.toScVal(),
      amountScval
    ]
    const withdrawParams = [
      employer.toScVal(),
      amountScval
    ]

    invokePayrollVault(
      payrollAddress,
      isDeposit ? PayrollVaultMethod.DEPOSIT : PayrollVaultMethod.WITHDRAW,
      isDeposit ? depositParams : withdrawParams,
      true
    ).then((result) => {
      console.log('🚀 ~ ).then ~ result:', result);
      sorobanContext.connect()
      onClose()
    }).catch((error) => {
      console.log("Error", error)
    })

  }

  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} isCentered size={'full'}>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>{isDeposit ? "Deposit to" : "Withdraw from"} account</ModalHeader>
          <ModalCloseButton />
          <ModalBody>
            <InputGroup>
              <InputLeftElement color={'grey.300'}>$</InputLeftElement>
              <Input
                placeholder='Salary'
                value={amount}
                onChange={(e) => setAmount(parseFloat(e.target.value))}
                variant={'outline'}
              >
              </Input>
            </InputGroup>
          </ModalBody>

          <ModalFooter justifyContent={'center'} pb={12}>
            <Button
              variant='solid'
              colorScheme='green'
              rounded={18}
            >
              Close
            </Button>
            <Button variant='ghost' onClick={invoke}>{isDeposit ? "Deposit" : "Withdraw"}</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
};

export default BusinessAccountModal;