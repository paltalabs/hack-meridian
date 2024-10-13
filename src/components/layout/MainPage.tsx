import { useSorobanReact } from '@soroban-react/core'
import React, { useEffect, useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { SearchBar } from './SearchBar'
import { useDispatch } from 'react-redux'
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll'
import { fetchPayrollAddress } from '@/utils/payrollVault'
import { Address, scValToNative, xdr } from '@stellar/stellar-sdk'
import { TradContractsAccordion } from '../Accordion/TradContractsAccorrdion'
import { Button, Flex, Stack, Text } from '@chakra-ui/react'
import { PaymentPeriod, addEmployee, setBalance, setName } from '@/store/features/employerStore'
import { CreateContractModal } from '../Modals/CreateContractModal'
import { PayButton } from '../buttons/PayButton'

export const MainPage = () => {
  const sorobanContext = useSorobanReact()
  const { address, activeChain } = sorobanContext
  const [isDrawerOpen, setIsDrawerOpen] = useState<boolean>(false)
  const dispatch = useDispatch(); // Redux dispatch
  const invokePayrollVault = usePayrollVaultCallback(); // Payroll vault hook
  const [companyBalance, setCompanyBalance] = useState(0)

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
      //@ts-ignore
      dispatch(setBalance(Number(scValToNative(result))))
      //@ts-ignore
      setCompanyBalance(Number(scValToNative(result)))
      invokePayrollVault(
        vaultAddress,
        PayrollVaultMethod.GET_EMPLOYER,
        [employer.toScVal()],
        false
      ).then((result) => {
        console.log("result", scValToNative(result as xdr.ScVal))
        const nativeResult = scValToNative(result as xdr.ScVal)
        dispatch(setName(nativeResult.name))

        const employees = {
          "GDRPBET7UV3NDFIW34MMG6XOI2BS44BXGI76KOQ3XCZXGLOPNNTXECB3": {
            employee: {
              address: "GDRPBET7UV3NDFIW34MMG6XOI2BS44BXGI76KOQ3XCZXGLOPNNTXECB3",
              name: "Pedro Urdemales"
            },
            payment_period: PaymentPeriod.Weekly,
            salary: 1000_0000000,
            notice_period: 1,
            employment_start_date: 0,
            last_payment_date: 0,
            is_active: true,
          },
          "GCNM6ABSY5VPPPU4BTUDSBNPMGTYWOI6UK3MZFN3EJUFYYUJRU5QXVYD": {
            employee: {
              address: "GCNM6ABSY5VPPPU4BTUDSBNPMGTYWOI6UK3MZFN3EJUFYYUJRU5QXVYD",
              name: "Aureliano Buendia"
            },
            payment_period: PaymentPeriod.Weekly,
            salary: 1200_0000000,
            notice_period: 1,
            employment_start_date: 0,
            last_payment_date: 0,
            is_active: true,
          },
          "GAXG7JCGN4V73PYTGCW2JLIRJLMD7I42BZKRPBZCFLA2D6BHRVRCXHOG": {
            employee: {
              address: "GAXG7JCGN4V73PYTGCW2JLIRJLMD7I42BZKRPBZCFLA2D6BHRVRCXHOG",
              name: "Irene Beltran"
            },
            payment_period: PaymentPeriod.Weekly,
            salary: 1150_0000000,
            notice_period: 1,
            employment_start_date: 0,
            last_payment_date: 0,
            is_active: true,
          },
        }
        Object.entries(employees).forEach(([address, workContract]) => {
          //@ts-ignore
          dispatch(addEmployee({ address, workContract }));
        });
        //@ts-ignore
        // dispatch(setBalance(Number(scValToNative(result))))
      })

    })


  }, [address, activeChain])
  const [isCreateContractModalOpen, setIsCreateContractModalOpen] = useState<boolean>(false)

  if (!address) return null;
  return (
    <>
      <ProfileDrawer isOpen={isDrawerOpen} onClose={() => { setIsDrawerOpen(false) }} />
      <SearchBar handleOpenDrawer={setIsDrawerOpen} handleCreateContract={setIsCreateContractModalOpen} />
      <CreateContractModal isOpen={isCreateContractModalOpen} onClose={setIsCreateContractModalOpen} />
      <Stack>
        <Text>Business account balance:</Text>
        <Text as={'b'} fontSize={'3xl'}>$ {companyBalance / 10000000}</Text>
      </Stack>
      <TradContractsAccordion />
      <Flex justifyContent="flex-end" width="100%" mt={4}>
        <PayButton />
      </Flex>
    </>
  )
}

export default MainPage
