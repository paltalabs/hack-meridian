# Welcome to Payroll Genius 🧠!


Payroll Genius is a decentralized application (dapp) inspired by the [Link project](https://github.com/paltalabs/hack-meridian), designed to simplify the payroll process for businesses while leveraging DeFi solutions on the Stellar blockchain.

## Build, Compile, Deploy and Test your self.
1.- Run Docker Compose and enter to our Docker container.
This helps us be sure that we all run the same software!
```bash
cp .env.example .env
docker compose up -d
bash run
```

2.- Test and Compile the PayrollVault Smart Contract
```bash
cd /workshop/contracts
cp .env.example .env # for private keys
make build
make test
```
3.- Fill the /contracts/.env file with the deployers private key

4.- Deploy the PayrollVault Smart Contract 
```bash
cd /workshop/contracts
yarn
yarn deploy testnet # can also be mainnet if you have some XLM for gas!
```

5.- Run the ReactJS frontend
```bash
cd /workshop/
yarn
yarn dev
```
