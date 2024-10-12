import { promises as fs } from 'fs';
import path from 'path';

export async function fetchPayrollAddress(network: string): Promise<string | undefined> {
    if (network !== "testnet" && network !== "mainnet") {
        throw new Error(`Invalid network: ${network}. It should be testnet or mainnet`);
    }

    const filePath = path.resolve(__dirname, `../../public/${network}.contracts.json`);

    try {
        const fileData = await fs.readFile(filePath, 'utf-8');
        const data = JSON.parse(fileData);
        const factoryAddress = data.ids.defindex_factory;
        return factoryAddress;
    } catch (error) {
        console.log("error fetching payroll address:", error)
    }
}
