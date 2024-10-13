#![cfg(test)]
extern crate std;
use soroban_sdk::{
    Env, 
    Address, 
           testutils::{
        Address as _,
    },
};
// use crate::{PayrollVault};

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;
pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

// PayrollVault Contract

pub mod payroll_vault {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/payroll_vault.optimized.wasm");
    pub type PayrollVaultClient<'a> = Client<'a>;
}
use payroll_vault::PayrollVaultClient;


fn create_payroll_vault_contract<'a>(
    e: & Env
) -> PayrollVaultClient<'a> {
    let payroll_vault_address = &e.register_contract_wasm(None, payroll_vault::WASM);
    let payroll_vault_client = PayrollVaultClient::new(e, payroll_vault_address);
    payroll_vault_client
}



pub struct PaymentVaultTest<'a> {
    env: Env,
    // admin: Address,
    employer: Address,
    token: TokenClient<'a>,
    contract: PayrollVaultClient<'a>,
}

impl<'a> PaymentVaultTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let employer = Address::generate(&env);
        let token = create_token_contract(&env, &admin);
        let contract = create_payroll_vault_contract(&env);

        // mint to employer
        token.mint(&employer, &1_000_000_000_000);

        // let test = PhoenixTest::phoenix_setup();
        
        // let wasm_hash = test.env.deployer().upload_contract_wasm(phoenix_adapter_contract::WASM);
        // let deployer_client = create_deployer(&test.env);

        // let adapter_client_not_initialized = create_soroswap_aggregator_phoenix_adapter(&test.env);
        // // Deploy contract using deployer, and include an init function to call.
        // let salt = BytesN::from_array(&test.env, &[0; 32]);
        // let init_fn = Symbol::new(&test.env, &("initialize"));

        // let protocol_id = String::from_str(&test.env, "phoenix");
        // let protocol_address = test.multihop_client.address.clone();

        // // Convert the arguments into a Vec<Val>
        // let init_fn_args: Vec<Val> = (protocol_id.clone(), protocol_address.clone()).into_val(&test.env);

        // test.env.mock_all_auths();
        // let (contract_id, _init_result) = deployer_client.deploy(
        //     &deployer_client.address,
        //     &wasm_hash,
        //     &salt,
        //     &init_fn,
        //     &init_fn_args,
        // );

        // let adapter_client = phoenix_adapter_contract::Client::new(&test.env, &contract_id);

        PaymentVaultTest {
            env: env,
            // admin: admin,
            employer: employer,
            token: token,
            contract: contract,
            // adapter_client,
            // adapter_client_not_initialized,
            // factory_client: test.factory_client,
            // multihop_client: test.multihop_client,
            // token_1: test.token_1,
            // token_2: test.token_2,
            // token_3: test.token_3,
            // user: test.user,
        }
    }
}

pub mod pay_employees;
pub mod deposit;
pub mod employ;
pub mod fire;
pub mod initialize;
pub mod withdraw;