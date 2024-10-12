import fs from 'fs';
import path from 'path';

export async function fetchPayrollAddress(network: string): Promise<string | undefined> {
    console.log('🚀 ~ fetchPayrollAddress ~ network:', network);
    if (network !== "testnet" && network !== "mainnet") {
        throw new Error(`Invalid network: ${network}. It should be testnet or mainnet`);
    }

    const filePath = path.resolve(__dirname, `../../public/${network}.contracts.json`);

    try {
        const fileData = fs.readFileSync(filePath, 'utf-8');
        console.log('🚀 ~ fetchPayrollAddress ~ fileData:', fileData);
        const data = JSON.parse(fileData);
        const factoryAddress = data.ids.payroll_vault;
        return factoryAddress;
    } catch (error) {
        console.log("error fetching payroll address:", error);
    }
}
