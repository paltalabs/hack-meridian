import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { AddressBook } from "../utils/address_book.js";

export async function deployContracts(addressBook: AddressBook) {
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Payroll Vault");
  console.log("-------------------------------------------------------");
  await installContract("payroll_vault", addressBook, loadedConfig.admin);
  await deployContract(
    "payroll_vault",
    "payroll_vault",
    addressBook,
    loadedConfig.admin
  );

  const asset_address = "CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI";

  const payrollInitParams: xdr.ScVal[] = [
    new Address(asset_address).toScVal(),
  ];

  console.log("Initializing DeFindex Factory");
  await invokeContract(
    "payroll_vault",
    addressBook,
    "initialize",
    payrollInitParams,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

try {
  await deployContracts(addressBook);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
