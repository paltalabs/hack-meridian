import { useEffect, useState } from 'react';
import { useDispatch } from 'react-redux';
import { setAddress, setName, addEmployee, setTotalLiabilities } from '@/store/features/employerStore';
import { PayrollVaultMethod, usePayrollVault } from '@/hooks/usePayroll'; // Assuming the usePayrollVault is imported correctly
import { useSorobanReact } from "@soroban-react/core";
import StellarSdk from "@stellar/stellar-sdk";
import {
    Address,
    nativeToScVal,
    scValToNative,
    xdr,
} from "@stellar/stellar-sdk";

export const useEmployer = (network: string) => {
    const { address } = useSorobanReact(); // Get the user's wallet address from SorobanReact
    const { invokePayrollVault } = usePayrollVault(network); // Payroll vault hook
    const dispatch = useDispatch(); // Redux dispatch
    const [error, setError] = useState<string | null>(null);

    const vaultAddress = "CCUWKBOGIYZK7HVIMYJAA65WILSV6GEFFQHJJFACHIKOANR2556IXXDV"
    useEffect(() => {
        const fetchEmployerDetails = async () => {
            try {
                // Check if we have an employer address to fetch
                if (!address) {
                    throw new Error('No employer address available');
                }

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
