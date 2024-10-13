import { act, useEffect, useState } from 'react';
import { useDispatch } from 'react-redux';
import { setAddress, setName, addEmployee, setTotalLiabilities, setBalance } from '@/store/features/employerStore';
import { PayrollVaultMethod, usePayrollVaultCallback } from '@/hooks/usePayroll'; // Assuming the usePayrollVaultCallback is imported correctly
import { useSorobanReact } from "@soroban-react/core";
import StellarSdk from "@stellar/stellar-sdk";
import {
    Address,
    nativeToScVal,
    scValToNative,
    xdr,
} from "@stellar/stellar-sdk";
import { fetchPayrollAddress } from '@/utils/payrollVault';

export const useEmployer = (network: string) => {

    const sorobanContext = useSorobanReact()
    const { address, activeChain } = sorobanContext
    const invokePayrollVault = usePayrollVaultCallback(); // Payroll vault hook
    const dispatch = useDispatch(); // Redux dispatch
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchEmployerDetails = async () => {
            try {
                // Check if we have an employer address to fetch
                if (!address) {
                    throw new Error('No employer address available');
                }
                if (!activeChain) {
                    throw new Error('No active Chain when fetching employer details')
                }

                const vaultAddress = fetchPayrollAddress(activeChain?.id)
                // Call the contract to get the employer details
                const employerData: any = await invokePayrollVault(vaultAddress, PayrollVaultMethod.GET_EMPLOYER, [
                    (new Address(address)).toScVal(), // Convert the address to ScVal type
                ]);

                // Assuming employerData contains the expected fields like name, employees, etc.
                if (employerData) {
                    const { name, employees, total_liabilities } = scValToNative(employerData.returnValue);

                    // Dispatch the data to the Redux store
                    dispatch(setAddress(address));
                    dispatch(setName(name));
                    dispatch(setTotalLiabilities(total_liabilities));

                    // Add each employee to the store
                    Object.keys(employees).forEach(employeeAddress => {
                        const employeeContract = employees[employeeAddress];
                        dispatch(addEmployee({ address: employeeAddress, workContract: employeeContract }));
                    });
                }
            } catch (err) {
                setError(`Error fetching employer details: ${err}`);
            }
        };

        fetchEmployerDetails();
    }, [address, network, invokePayrollVault, dispatch]);

    return { error };
};

export const useEmployerBalance = () => {

    const sorobanContext = useSorobanReact()
    const { address, activeChain } = sorobanContext
    const dispatch = useDispatch(); // Redux dispatch

    if (!activeChain || !address) {
        console.log("Not connected")
        return;
    }
    const invokePayrollVault = usePayrollVaultCallback(); // Payroll vault hook
    const vaultAddress = fetchPayrollAddress(activeChain.id)
    const employer = new Address(address)

    invokePayrollVault(
        vaultAddress,
        PayrollVaultMethod.EMPLOYER_BALANCE,
        [employer.toScVal()],
        false
    ).then((result) => {
        //@ts-ignore
        dispatch(setBalance(scValToNative(result)))
    })


}