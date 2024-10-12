#![no_std]
use defindex_strategy_core::DeFindexStrategyClient;
use soroban_sdk::{
    contract, contractimpl, panic_with_error,
    token::{TokenClient, TokenInterface},
    Address, Env, Map, String, Vec,
};
use soroban_token_sdk::metadata::TokenMetadata;

mod access;
mod error;
mod funds;
mod interface;
mod models;
mod storage;
mod strategies;
mod test;
mod token;
mod utils;

use access::{AccessControl, AccessControlTrait, RolesDataKey};
use funds::{fetch_current_idle_funds, fetch_current_invested_funds, fetch_total_managed_funds};
use interface::{AdminInterfaceTrait, VaultTrait};
use models::Asset;
use storage::{
    get_assets, set_asset,
    set_defindex_receiver, set_total_assets,
};
use strategies::get_strategy_client;
use token::{internal_mint, internal_burn, write_metadata, VaultToken};
use utils::{
    calculate_deposit_amounts_and_shares_to_mint, calculate_withdrawal_amounts, check_initialized,
    check_nonnegative_amount,
};

pub use error::ContractError;

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {
    fn initialize(
        e: Env,
        assets: Vec<Asset>,
        manager: Address,
        emergency_manager: Address,
        fee_receiver: Address,
        defindex_receiver: Address,
    ) -> Result<(), ContractError> {
        let access_control = AccessControl::new(&e);
        if access_control.has_role(&RolesDataKey::Manager) {
            panic_with_error!(&e, ContractError::AlreadyInitialized);
        }

        access_control.set_role(&RolesDataKey::EmergencyManager, &emergency_manager);
        access_control.set_role(&RolesDataKey::FeeReceiver, &fee_receiver);
        access_control.set_role(&RolesDataKey::Manager, &manager);

        // Set Paltalabs Fee Receiver
        set_defindex_receiver(&e, &defindex_receiver);

        // Store Assets Objects
        let total_assets = assets.len();
        set_total_assets(&e, total_assets as u32);
        for (i, asset) in assets.iter().enumerate() {
            // for every asset, we need to check that the list of strategyes indeed support this asset
            
            // TODO Fix, currently failing
            // for strategy in asset.strategies.iter() {
            //     let strategy_client = DeFindexStrategyClient::new(&e, &strategy.address);
            //     if strategy_client.asset() != asset.address {
            //         panic_with_error!(&e, ContractError::StrategyDoesNotSupportAsset);
            //     }
            // }
            set_asset(&e, i as u32, &asset);
        }

        // Metadata for the contract's token (unchanged)
        // TODO: Name should be concatenated with some other name giving when initializing. Check how soroswap pairs  token are called.
        let decimal: u32 = 7;
        let name: String = String::from_str(&e, "dfToken");
        let symbol: String = String::from_str(&e, "DFT");

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );

        // TODO: Emit event
        Ok(())
    }

    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
    ) -> Result<(Vec<i128>, i128), ContractError> {
        check_initialized(&e)?;
        from.require_auth();

        // get assets
        let assets = get_assets(&e);
        // assets lenght should be equal to amounts_desired and amounts_min length
        let assets_length = assets.len();
        if assets_length != amounts_desired.len() || assets_length != amounts_min.len() {
            panic_with_error!(&e, ContractError::WrongAmuntsLength);
        }

        // for every amount desired, check non negative
        for amount in amounts_desired.iter() {
            check_nonnegative_amount(amount)?;
        }
        // for amount min is not necesary to check if it is negative

        let (amounts, shares_to_mint) = if assets_length == 1 {
        // If Total Assets == 1
            let shares = if VaultToken::total_supply(e.clone())==0{
                // TODO In this case we might also want to mint a MINIMUM LIQUIDITY to be locked forever in the contract
                // this might be for security and practical reasons as well
                // shares will be equal to the amount desired to deposit, just for simplicity
                amounts_desired.get(0).unwrap() // here we have already check same lenght
            } else{
                // in this case we will mint a share proportional to the total managed funds
                let total_managed_funds = fetch_total_managed_funds(&e);
                VaultToken::total_supply(e.clone()) * amounts_desired.get(0).unwrap() / total_managed_funds.get(assets.get(0).unwrap().address.clone()).unwrap()
            };
            (amounts_desired, shares)
        } else {
        // If Total Assets > 1
            calculate_deposit_amounts_and_shares_to_mint(
                &e,
                &assets,
                &amounts_desired,
                &amounts_min,
            )
        };

        // for every asset,
        for (i, amount) in amounts.iter().enumerate() {
            if amount > 0 {
                let asset = assets.get(i as u32).unwrap();
                let asset_client = TokenClient::new(&e, &asset.address);
                // send the current amount to this contract
                asset_client.transfer(&from, &e.current_contract_address(), &amount);
            }
        }

        // now we mint the corresponding dfTOkenb
        internal_mint(e, from, shares_to_mint);

        // TODO: emit events
        // TODO return amounts and shares to mint
        Ok((amounts, shares_to_mint))
    }

    fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<(), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(df_amount)?;
        from.require_auth();

        // Check if the user has enough dfTokens
        let df_user_balance = VaultToken::balance(e.clone(), from.clone());
        if df_user_balance < df_amount {
            panic_with_error!(&e, ContractError::InsufficientBalance);
        }

        // Burn the dfTokens
        internal_burn(e.clone(), from.clone(), df_amount);

        // Calculate the withdrawal amounts for each token based on the dfToken amount
        let withdrawal_amounts = calculate_withdrawal_amounts(&e, df_amount)?;

        // Get idle funds for each token
        let idle_funds = fetch_current_idle_funds(&e);

        // Loop through each token and handle the withdrawal
        for (asset, required_amount) in withdrawal_amounts.iter() {
            let mut total_amount_to_transfer = 0;

            // Get idle funds for this specific token, if it exists
            let idle_balance = idle_funds.get(asset.address.clone()).unwrap_or(0);

            // Withdraw as much as possible from idle funds
            // REVIEW: This check is not necesary
            if idle_balance > 0 { 
                if idle_balance >= required_amount {
                    // REF
                    // If idle funds cover the full amount, no need to check strategies
                    total_amount_to_transfer = required_amount;
                } else {
                    // Partial amount from idle funds
                    // REIVEW? WHAT?
                    total_amount_to_transfer = idle_balance;
                    // If we want to keep a minimum amount of idle funds we should add it here so it weithdraws the required amount for the withdrawal and some more to keep the minimum
                    // REVIEW remaining_amount is getting initialized again here... so what is the purpose?
                    let mut remaining_amount = required_amount - idle_balance;

                    // Now, withdraw the remaining amount from the supported strategies
                    // TODO: is there any preference? should we withdraw from the strategy with the most funds first?
            
                    for strategy in asset.strategies.iter() {
                        let strategy_client = get_strategy_client(&e, strategy.address);

                        // Check if the strategy supports this token via the asset method

                        let strategy_balance = strategy_client.balance(&from);
                        if strategy_balance >= remaining_amount {
                            strategy_client.withdraw(&remaining_amount, &from);
                            total_amount_to_transfer += remaining_amount;
                            break;
                        } else {
                            // Withdraw as much as possible from this strategy
                            strategy_client.withdraw(&strategy_balance, &from);
                            total_amount_to_transfer += strategy_balance;

                            // Reduce remaining amount by the amount withdrawn
                            remaining_amount -= strategy_balance;
                        }
        
                        // If no strategies can fulfill the remaining amount, throw an error
                        // REVIEW: this is dangerous
                        // this means that total_amount_to_transfer should always be equal to required_amount
                        // so whats the purpose of that variable?
                        if remaining_amount > 0 { // TODO && i == total_strategies - 1 
                            panic_with_error!(&e, ContractError::InsufficientBalance);
                        }
                    }
                }
            }

            // Perform the transfer once the total amount to transfer has been calculated
            TokenClient::new(&e, &asset.address).transfer(
                &e.current_contract_address(),
                &from,
                &total_amount_to_transfer,
            );
        }
    
        Ok(())
    }

    fn emergency_withdraw(e: Env, amount: i128, from: Address) -> Result<(), ContractError> {
        check_initialized(&e)?;
        from.require_auth();
        let assets = get_assets(&e);
        for asset in assets.iter() {
            for strategy in asset.strategies.iter() {
                let strategy_client = DeFindexStrategyClient::new(&e, &strategy.address);
                // TODO. amount cannot be defined by the user... unless the user also defines the strategy address
                strategy_client.withdraw(&amount, &from);
            }
            
        }

        Ok(())
    }

    fn get_assets(e: Env) -> Vec<Asset> {
        get_assets(&e)
    }

    fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128> {
        fetch_total_managed_funds(e)
    }

    fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128> {
        fetch_current_invested_funds(e)
    }

    fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128> {
        fetch_current_idle_funds(e)
    }

    fn user_balance(e: Env, from: Address) -> i128 {
        VaultToken::balance(e, from)
    }
}

#[contractimpl]
impl AdminInterfaceTrait for DeFindexVault {
    fn set_fee_receiver(e: Env, caller: Address, fee_receiver: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_fee_receiver(&caller, &fee_receiver)
    }

    fn get_fee_receiver(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_fee_receiver()
    }

    fn set_manager(e: Env, manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_manager(&manager)
    }

    fn get_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_manager()
    }

    fn set_emergency_manager(e: Env, emergency_manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_emergency_manager(&emergency_manager)
    }

    fn get_emergency_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_emergency_manager()
    }
}
