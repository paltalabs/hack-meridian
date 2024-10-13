import { useSorobanReact } from '@soroban-react/core'
import React, { useEffect, useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { SearchBar } from './SearchBar'
import { useDispatch } from 'react-redux'
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll'
import { fetchPayrollAddress } from '@/utils/payrollVault'
import { Address, scValToNative } from '@stellar/stellar-sdk'
import { TradContractsAccordion } from '../Accordion/TradContractsAccorrdion'
import { Stack, Text } from '@chakra-ui/react'
import { setBalance } from '@/store/features/employerStore'

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
      dispatch(setBalance(Number(scValToNative(result))))

    })

  }, [address, activeChain])

  if (!address) return null;
  return (
    <>
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
      <SearchBar handleOpenDrawer={setIsDrawerOpen} />
      <Stack>
        <Text>Business account balance:</Text>
        <Text as={'b'} fontSize={'3xl'}>$ 1000</Text>
      </Stack>
      <TradContractsAccordion />
    </>
  )
}

export default MainPage
