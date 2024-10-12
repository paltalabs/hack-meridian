import { useSorobanReact } from "@soroban-react/core";
import { useCallback, useState, useEffect } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import { fetchPayrollAddress } from "@/utils/payrollVault"; // Assuming this is where you have the fetchPayrollAddress function

export enum PayrollVaultMethod {
    INITIALIZE = "initialize",
    DEPOSIT = "deposit",
    WITHDRAW = "withdraw",
    EMPLOY = "employ",
    PAY_EMPLOYEES = "pay_employees",
    FIRE = "fire",
    EMPLOYER_BALANCE = "employer_balance",
    GET_EMPLOYER = "get_employer",
    ASSET = "asset"
}

export enum PaymentPeriod {
    WEEKLY = 0,
    MONTHLY = 1,
    ANNUALY = 2,
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export function usePayrollVault(network: string) {
    const sorobanContext = useSorobanReact();
    const [error, setError] = useState<string | null>(null);

    // Callback to interact with the payroll vault contract
    const invokePayrollVault = useCallback(
        async (vaultAddress: string, method: PayrollVaultMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
            if (!vaultAddress) {
                throw new Error("Payroll vault address not available");
            }

            const result = (await contractInvoke({
                contractAddress: vaultAddress,
                method: method,
                args: args,
                sorobanContext,
                signAndSend: signAndSend,
                reconnectAfterTx: false,
            })) as TxResponse;

            if (!signAndSend) return result;

            if (
                isObject(result) &&
                result?.status !== StellarSdk.SorobanRpc.Api.GetTransactionStatus.SUCCESS
            ) throw result;
            return result;
        },
        [sorobanContext]
    );

    return { invokePayrollVault, error };
}
