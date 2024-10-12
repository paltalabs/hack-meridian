import { Text } from "@chakra-ui/react"
import { useSorobanReact } from "@soroban-react/core"
import { useSelector } from 'react-redux';
import {
    selectEmployerAddress,
    selectEmployerName,
    selectEmployerEmployees,
    selectTotalLiabilities
} from '@/store/features/employerStore';
import { usePayrollVault, PaymentPeriod, PayrollVaultMethod } from "@/hooks/usePayroll";
import { Address, nativeToScVal } from "@stellar/stellar-sdk";
import { useEffect, useState } from "react";
import { fetchPayrollAddress } from "@/utils/payrollVault";

export const Example = () => {

    const { address, activeChain } = useSorobanReact();

    // Use selectors to get the required data from the Redux store
    const name = useSelector(selectEmployerName);
    const employees = useSelector(selectEmployerEmployees);
    const totalLiabilities = useSelector(selectTotalLiabilities);

    const { invokePayrollVault } = usePayrollVault(activeChain?.name || 'testnet')

    const [showResult, setShowResult] = useState("")
    const [payrollAddress, setPayrollAddress] = useState("")

    useEffect(() => {
        if (!activeChain?.name) return;
        fetchPayrollAddress(activeChain?.name.toLocaleLowerCase()).then((temp) => {
            if (!temp) return;
            setPayrollAddress(temp)
        })
    }, [])

    const hire = async () => {
        // We will execute EMPLOY
        if (!address) return;
        const employer = new Address(address);
        const employee = new Address("GCWGZHN3ZVH5BSW6246DOIKPDQL6RXKKENB6ZJ2MIVPISGKRBIOHM2GO")
        const name = nativeToScVal("Joe", { type: "string" })
        // const payment_period
        // const paymentPeriod = nativeToScVal(PaymentPeriod.WEEKLY, { type: "int" }) 
        const paymentPeriod = nativeToScVal(PaymentPeriod.MONTHLY, { type: "u32" })
        const salary = nativeToScVal(1000_0000000, { type: "i128" })
        const noticePeriod = nativeToScVal(2, { type: "u64" })

        const employParams = [
            employer.toScVal(),
            employee.toScVal(),
            name,
            paymentPeriod,
            salary,
            noticePeriod
        ]

        let result: any;
        try {
            result = await invokePayrollVault(
                payrollAddress,
                PayrollVaultMethod.EMPLOY,
                employParams,
                true
            );
            setShowResult(result)
        } catch (e: any) {
            console.log("Error while employing", e)
            setShowResult(e.toString())
        }

    }
    const deposit = async () => {

    }

    return (
        <div>
            <p>
                Payroll address: {payrollAddress}
            </p>
            <p>
                <button onClick={() => hire()}>
                    Hire!
                </button>
            </p>
            <p>
                <button onClick={() => deposit()}>
                    deposit
                </button>
            </p>
            <p>
                <button onClick={() => setShowResult("")}>
                    Clean
                </button>
            </p>
            <p>
                {showResult}
            </p>
            <h2>Employer Information</h2>
            <p><strong>Address:</strong> {address}</p>
            <p><strong>Name:</strong> {name}</p>
            <p><strong>Total Liabilities:</strong> {totalLiabilities}</p>

            <h3>Employees:</h3>
            <ul>
                {Object.keys(employees).length === 0 ? (
                    <p>No employees found.</p>
                ) : (
                    Object.keys(employees).map((employeeAddress) => {
                        const employee = employees[employeeAddress];
                        return (
                            <li key={employeeAddress}>
                                <p><strong>Employee Address:</strong> {employee.employee.address}</p>
                                <p><strong>Employee Name:</strong> {employee.employee.name}</p>
                                <p><strong>Salary:</strong> {employee.salary}</p>
                                <p><strong>Payment Period:</strong> {employee.payment_period}</p>
                                <p><strong>Notice Period:</strong> {employee.notice_period}</p>
                                <p><strong>Employed At:</strong> {employee.employed_at}</p>
                                <p><strong>Is Active:</strong> {employee.is_active ? 'Yes' : 'No'}</p>
                                {employee.unemployed_at && (
                                    <p><strong>Unemployed At:</strong> {employee.unemployed_at}</p>
                                )}
                                <p><strong>Notice Period Payments Made:</strong> {employee.notice_period_payments_made}</p>
                                <hr />
                            </li>
                        );
                    })
                )}
            </ul>
        </div>
    );
}