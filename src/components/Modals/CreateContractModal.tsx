import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { Avatar, Button, Icon, Input, InputGroup, InputLeftElement, Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay, NumberDecrementStepper, NumberIncrementStepper, NumberInput, NumberInputField, NumberInputStepper, Select, Stack, Text, useClipboard } from '@chakra-ui/react'
import { AddIcon } from '@chakra-ui/icons'
import { Address, nativeToScVal } from "@stellar/stellar-sdk"
import { PaymentPeriod, PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll'
import { fetchPayrollAddress } from '@/utils/payrollVault'
import { FiFile } from 'react-icons/fi'
import UploadComponent from '../upload'

export const CreateContractModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: (value: boolean) => void }) => {
  const sorobanContext = useSorobanReact()
  const { address, activeChain } = sorobanContext

  const [fullName, setFullName] = useState('')
  const [stellarAddress, setStellarAddress] = useState('')
  const [paymentPeriod, setPaymentPeriod] = useState('')
  const [salary, setSalary] = useState(0)
  const [noticePeriod, setNoticePeriod] = useState(0)

  const [fileHash, setFileHash] = useState('');
  const [signUrl, setSignUrl] = useState('');

  const clipboard = useClipboard(signUrl);

  const invokePayrollVault = usePayrollVaultCallback()

  const handleSubmit = async () => {
    if (!address || !stellarAddress || !fullName || !paymentPeriod || !salary || !activeChain) {
      alert("Please fill all the fields")
      return
    }
    const payrollAddress = fetchPayrollAddress(activeChain?.id)
    let result: any;

    try {
      const employee = new Address(stellarAddress)
      const name = nativeToScVal(fullName, { type: "string" })
      const hash = nativeToScVal("hash", { type: "string" })
      let paymentPeriodTemp;
      switch (paymentPeriod) {
        case "WEEKLY":
          paymentPeriodTemp = PaymentPeriod.WEEKLY
          break;
        case "MONTHLY":
          paymentPeriodTemp = PaymentPeriod.MONTHLY
          break;
        case "YEARLY":
          paymentPeriodTemp = PaymentPeriod.ANNUALY
          break;
        case "SECONDLY":
          paymentPeriodTemp = PaymentPeriod.SECONDLY
          break;

        default:
          break;
      }
      const paymentPeriodScVal = nativeToScVal(paymentPeriodTemp, { type: "u32" })
      const salaryScVal = nativeToScVal(salary * 1_0000000, { type: "i128" })
      const noticePeriodScVal = nativeToScVal(noticePeriod, { type: "u64" })

      const employParams = [
        (new Address(address)).toScVal(),
        employee.toScVal(),
        name,
        paymentPeriodScVal,
        salaryScVal,
        noticePeriodScVal,
        nativeToScVal(fileHash, {type: "string"})
      ]

      result = await invokePayrollVault(
        payrollAddress,
        PayrollVaultMethod.EMPLOY,
        employParams,
        true,
      );
      sorobanContext.connect()

      onClose(false)
    } catch (e: any) {
      console.log("Error while employing", e)
    }

  }

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
              <Input
                placeholder='Full name'
                value={fullName}
                onChange={(e) => setFullName(e.target.value)}
                variant={'outline'}
              />
              <Input
                placeholder='Stellar address'
                value={stellarAddress}
                onChange={(e) => setStellarAddress(e.target.value)}
                variant={'outline'}
              />
              <Select
                placeholder='Select payment frequency'
                value={paymentPeriod}
                onChange={(e) => setPaymentPeriod(e.target.value)}
                variant={'outline'}
              >
                <option value='WEEKLY'>Weekly</option>
                <option value='MONTHLY'>Monthly</option>
                <option value='YEARLY'>Yearly</option>
                <option value='SECONDLY'>Every Second</option>
              </Select> 
              <Text>{"Number of notice periods:"}</Text>
              
              <NumberInput
                min={0}
                value={noticePeriod}
                onChange={(valueString) => setNoticePeriod(!valueString ? 0 : parseFloat(valueString))}
              >
                <NumberInputField />
                <Text>{"Salary per period:"}</Text>

                <NumberInputStepper>
                  <NumberIncrementStepper />
                  <NumberDecrementStepper />
                </NumberInputStepper>
              </NumberInput>
              <InputGroup>
                <InputLeftElement color='gray.300' fontSize='1.2em'>
                  $
                </InputLeftElement>
                <Input
                  placeholder='Salary'
                  value={salary}
                  onChange={(e) => setSalary(!e.target.value ? 0 : parseFloat(e.target.value))}
                  variant={'outline'}
                />
              </InputGroup>
              <InputGroup>
                <UploadComponent setFileHash={setFileHash} setSignUrl={setSignUrl} employee={stellarAddress}/>
              </InputGroup>
              <InputGroup>
                <Input
                  placeholder='File hash'
                  value={fileHash}
                  onChange={(e) => setFileHash(e.target.value)}
                  variant={'outline'}
                  disabled
                />
              </InputGroup>
              <Text>{signUrl}</Text>
              <Button onClick={clipboard.onCopy}>Copy Sign URL</Button>
            </Stack>
          </ModalBody>

          <ModalFooter justifyContent={'center'} pb={12}>
            <Button
              variant='solid'
              colorScheme='green'
              leftIcon={<AddIcon />}
              rounded={18}
              onClick={handleSubmit} // Call the handleSubmit function
              disabled={!fullName || !stellarAddress || !paymentPeriod || !salary || !activeChain || !fileHash}
            >
              Add employee
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  )
}
