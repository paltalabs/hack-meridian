#![no_std]
use models::{PaymentPeriod, WorkContract};
use soroban_sdk::{
    contract, contractimpl, // panic_with_error,
    // token::{TokenClient, TokenInterface},
    Address, Env, Vec, // Map, String,
};

mod error;
mod interface;
mod models;
mod storage;
mod test;

use interface::VaultTrait;
pub use error::ContractError;

use storage::{
    get_asset, get_employer, has_asset, set_asset, set_employer};

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
        _e: Env,
        _employer: Address,
        _amount: Vec<i128>,
    ) -> Result<(), ContractError> {
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
        // payment period should be enum weekly monthly or anually
        payment_period: PaymentPeriod,
        salary: i128,
        notice_period: i128, // how many payment periods before the employee can be fired
    ) -> Result<(), ContractError> {
        let mut employer_struct = get_employer(&e, &employer);

        let work_contract = WorkContract {
            employee: models::Employee {
                address: employee,
            },
            payment_period,
            salary,
            notice_period,
        };

        employer_struct.employees.push_back(work_contract);

        set_employer(&e, employer, employer_struct);
        Ok(())
    }

    fn claim_salary(
        _e: Env,
        _employee: Address,
    ) -> Result<i128, ContractError> {
        Ok(0i128)
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

    // READ FUNCTION

    // get employer balance
    fn employer_balance(
        _e: Env, 
        _employee: Address) -> i128 {
        0i128
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

    fn asset(e: Env) -> Address {
        get_asset(&e)
    }
}
