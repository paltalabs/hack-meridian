use soroban_sdk::{
    Address, Env, String, Vec};

use crate::{
    // models::{Asset, Strategy},
    models::{Employer, PaymentPeriod}, ContractError
};

pub trait VaultTrait {

    // WRITE FUNCTIONS

    fn initialize(
        e: Env,
        asset: Address,
        // manager: Address,
        // emergency_manager: Address,
        // fee_receiver: Address,
        // defindex_receiver: Address,
    ) -> Result<(), ContractError>;

    // deposit. employer can deposit funds
    // caller deemployer
    // employerposits amount into employers balance
        // this can be called from a POS terminal
    fn deposit(
        e: Env,
        caller: Address,
        employer: Address,
        amount: i128,
    ) -> Result<(), ContractError>;

    // withdraw: employer can withdraw funds
    // can onmly withdraw amount of fundsw so there is enought funds to pay employees
    fn withdraw(
        e: Env,
        employer: Address,
        amount: i128,
    ) -> Result<i128, ContractError>;
        
    // employ: employer can employ an employee
    // This functions produces
    // <employee, employer> -> <payment_period, salary, notice_period>
    // this function should take 
    fn employ(
        e: Env,
        employer: Address,
        employee: Address,
        name: String,
        payment_period: PaymentPeriod, // enum weekly monthly or anually
        salary: i128,
        notice_period: i128, // how many payment periods before the employee can be fired
    ) -> Result<(), ContractError>;

    fn pay_employees(
        e: Env,
        employer: Address,
    ) -> Result<(), ContractError>;

    // this can only be done by the employer
    // 
    fn fire(
        e: Env,
        employer: Address,
        employee: Address,
    ) -> Result<(), ContractError>;

    // READ FUNCTION

    // get employer balance
    fn employer_balance(e: Env, employer: Address) -> i128;

    // get employee available balanbce to withdraw now
    fn employee_available_balance(e: Env, employee: Address) -> i128;

    // get employer employee information
    fn employer_employee_info(e: Env, employer: Address, employee: Address) -> i128;

    fn get_employer(e: Env, employer_address: Address) -> Employer;

    fn asset(e: Env) -> Address;

    // /// get employer health 


    // fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<(), ContractError>;

    // fn emergency_withdraw(e: Env, amount: i128, from: Address) -> Result<(), ContractError>;

    // fn get_assets(e: Env) -> Vec<Asset>;

    // fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128>;

    // fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128>;

    // fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128>;

    // fn user_balance(e: Env, from: Address) -> i128;
}

// pub trait AdminInterfaceTrait {
//     fn set_fee_receiver(e: Env, caller: Address, fee_receiver: Address);

//     fn get_fee_receiver(e: Env) -> Result<Address, ContractError>;

//     fn set_manager(e: Env, manager: Address);

//     fn get_manager(e: Env) -> Result<Address, ContractError>;

//     fn set_emergency_manager(e: Env, emergency_manager: Address);

//     fn get_emergency_manager(e: Env) -> Result<Address, ContractError>;
// }
