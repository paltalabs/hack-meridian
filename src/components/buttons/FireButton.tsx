import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import { Button, IconButton } from '@chakra-ui/react'
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll';
import { fetchPayrollAddress } from '@/utils/payrollVault';
import { Address, scValToNative, xdr } from '@stellar/stellar-sdk';
import { BsTrash } from 'react-icons/bs';

export const FireButton = ({employee} : {employee: string}) => {
  const sorobanContext = useSorobanReact()
  const { address, activeChain, connect } = sorobanContext
  const invokePayrollVault = usePayrollVaultCallback();

  const handleFire = () => { 
    if (!address) return;
    if (!activeChain) return;
    
    const vaultAddress = fetchPayrollAddress(activeChain.id)
    const employer = new Address(address)
    
    invokePayrollVault(
      vaultAddress,
      PayrollVaultMethod.FIRE,
      [employer.toScVal(), new Address(employee).toScVal()],
      true
    ).then((result) => {
      //@ts-ignore
      console.log('🚀 « result:', scValToNative(result.returnValue as xdr.ScVal));
      sorobanContext.connect()
    })
  }

  return (
    <IconButton size={'md'} variant={'ghost'} aria-label='delete-contract' icon={<BsTrash />} py={4} onClick={handleFire} />
  )
}
