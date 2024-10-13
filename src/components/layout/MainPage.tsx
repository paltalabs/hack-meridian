import { useSorobanReact } from '@soroban-react/core'
import React, { useEffect, useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { Avatar, Grid, GridItem, IconButton, Input, InputGroup, InputLeftElement, InputRightElement } from '@chakra-ui/react'
import { AddIcon, Search2Icon } from '@chakra-ui/icons'
import { SearchBar } from './SearchBar'
import { useDispatch } from 'react-redux'
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll'
import { fetchPayrollAddress } from '@/utils/payrollVault'
import { Address } from '@stellar/stellar-sdk'

export const MainPage = () => {
  const sorobanContext = useSorobanReact()
  const { address, activeChain } = sorobanContext
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)
  const dispatch = useDispatch(); // Redux dispatch
  const invokePayrollVault = usePayrollVaultCallback(); // Payroll vault hook

  useEffect(() => {

    if (!activeChain || !address) {
      console.log("Not connected")
      return;
    }
    const vaultAddress = fetchPayrollAddress(activeChain.id)
    const employer = new Address(address)

    invokePayrollVault(
      vaultAddress,
      PayrollVaultMethod.EMPLOYER_BALANCE,
      [employer.toScVal()],
      false
    ).then((result) => {
      console.log('🚀 ~ ).then ~ result:', result);
      //@ts-ignore
      // dispatch(setBalance(scValToNative(result)))
      // dispatch(setBalance(0))
    })

  }, [address, activeChain])

  if (!address) return null;
  return (
    <>
      <SearchBar handleOpenDrawer={setIsDrawerOpen} />
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
    </>
  )
}

export default MainPage
