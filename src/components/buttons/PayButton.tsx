import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import { Button } from '@chakra-ui/react'
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll';
import { fetchPayrollAddress } from '@/utils/payrollVault';
import { Address, scValToNative, xdr } from '@stellar/stellar-sdk';

export const PayButton = () => {
  const { address, activeChain } = useSorobanReact()
  const invokePayrollVault = usePayrollVaultCallback();

  const handlePay = () => { 
    if (!address) return;
    if (!activeChain) return;
    
    const vaultAddress = fetchPayrollAddress(activeChain.id)
    const employer = new Address(address)
    
    invokePayrollVault(
      vaultAddress,
      PayrollVaultMethod.PAY_EMPLOYEES,
      [employer.toScVal()],
      true
    ).then((result) => {
      //@ts-ignore
      console.log('🚀 « result:', scValToNative(result.returnValue as xdr.ScVal));
    })

  }

  return (
    <Button colorScheme="pink" size="md" width={28} ml="auto" onClick={handlePay}>
      Pay
    </Button>
  )
}
