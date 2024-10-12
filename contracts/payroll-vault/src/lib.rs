#![no_std]
use models::{Employer, PaymentPeriod, WorkContract};
use soroban_sdk::{
    contract, contractimpl, token::TokenClient, Address, Env, Vec // Map, String,
};

mod error;
mod interface;
mod models;
mod storage;
mod test;
mod balance;



use interface::VaultTrait;
pub use error::ContractError;

use storage::{
    DAY_IN_LEDGERS,
INSTANCE_BUMP_AMOUNT,
INSTANCE_LIFETIME_THRESHOLD,
BALANCE_BUMP_AMOUNT,
BALANCE_LIFETIME_THRESHOLD,
    get_asset, 
    get_employer, 
    has_asset, 
    set_asset, 
    set_employer};

use balance::{
    read_balance, 
    receive_balance, 
    spend_balance};


fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}
    

#[contract]
pub struct PayrollVault;

#[contractimpl]
impl VaultTrait for PayrollVault {
    fn initialize(
        e: Env,
        asset: Address,
    ) -> Result<(), ContractError> {
        if has_asset(&e) {
            return Err(ContractError::AlreadyInitialized);
        }

        set_asset(&e, &asset);
        Ok(())
    }

    // deposit. employer can deposit funds
    // caller deemployer
    // employerposits amount into employers balance
    // this can be called from a POS terminal
    fn deposit(
        e: Env,
        caller: Address,
        employer: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        caller.require_auth();
        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        
        // caller sends amount to this contract
        TokenClient::new(
            &e, 
            &get_asset(&e)).transfer(
                &caller, 
                &e.current_contract_address(), 
            &amount);

        receive_balance(&e, employer.clone(), amount); // we record the balance of the employer

        // TODO: Create an event
        Ok(())
    }

    // withdraw: employer can withdraw funds
    // can onmly withdraw amount of fundsw so there is enought funds to pay employees
    fn withdraw(
        _e: Env,
        _employer: Address,
        _amount: i128,
    ) -> Result<i128, ContractError> {
        Ok(0i128)
    }
        
    // employ: employer can employ an employee
    // This functions produces
    // <employee, employer> -> <payment_period, salary, notice_period>
    // this function should take 
    fn employ(
        e: Env,
        employer: Address,
        employee: Address,
        payment_period: PaymentPeriod,
        salary: i128,
        notice_period: i128, // how many payment periods before the employee can be fired
    ) -> Result<(), ContractError> {
        let mut employer_struct = get_employer(&e, &employer);

        employer_struct.address.require_auth();

        if employer_struct.employees.contains_key(employee.clone()) {
            return Err(ContractError::AlreadyEmployed);
        }

        let work_contract = WorkContract {
            employee: models::Employee {
                address: employee.clone(),
            },
            payment_period,
            salary,
            notice_period,
        };

        employer_struct.employees.set(employee, work_contract.clone());

        set_employer(&e, employer, employer_struct);
        Ok(())
    }

    fn pay_employees(
        e: Env,
        employer: Address,
    ) -> Result<(), ContractError> {
        let mut employer_struct = get_employer(&e, &employer);
        employer_struct.address.require_auth();

        let asset = get_asset(&e);

        for (employee, work_contract) in employer_struct.employees.iter() {
            let salary = work_contract.salary;

            let employer_balance = employer_struct.balance;

            if employer_balance < salary {
                return Err(ContractError::InsufficientFunds);
            }

            TokenClient::new(&e, &asset).transfer(&e.current_contract_address(), &employee, &salary);

            employer_struct.balance -= salary;
        }

        set_employer(&e, employer, employer_struct);
        Ok(())
    }

    // this can only be done by the employer
    // 
    fn fire (
        _e: Env,
        _employer: Address,
        _employee: Address,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }


    // READ FUNCTION

    // get employer balance
    fn employer_balance(
        e: Env, 
        employer: Address
    ) -> i128 {
        let employer_struct = get_employer(&e, &employer);
        employer_struct.balance
    }

    // get employee available balanbce to withdraw now
    fn employee_available_balance(
        _e: Env, 
        _employee: Address) -> i128 {
        0i128
    }

    // get employer employee information
    fn employer_employee_info(
        _e: Env,
        _employer: Address,
        _employee: Address) -> i128 {
        0i128
    }

    fn get_employer(e: Env, employer_address: Address) -> Employer {
        get_employer(&e, &employer_address)
    }


    fn asset(e: Env) -> Address {
        get_asset(&e)
    }
}
