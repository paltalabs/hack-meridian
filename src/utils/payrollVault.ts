// Import the JSON file from src/constants/testnet.contracts.json
import contracts from '@/constants/testnet.contracts.json';

// Function to fetch the payroll vault ID
export const fetchPayrollAddress = (network: string): string => {
    if (network !== "testnet") console.log("fetchPayrollAddress: not supported network,", network)
    return contracts?.ids?.payroll_vault;
};

