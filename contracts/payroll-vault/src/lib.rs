#![no_std]
use models::{Employer, PaymentPeriod, WorkContract};
use soroban_sdk::{
    contract, contractimpl, Address, Env, Map, Vec // Map, String,
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
                address: employee.clone(),
            },
            payment_period,
            salary,
            notice_period,
        };

        employer_struct.employees.push_back(work_contract.clone());


        // /////////////// IF MAP IS USED ///////////////////////

        // let mut new_map: Map<Address, WorkContract> = Map::new(&e);

        // // This would replace existent employee or we could return AlreadyEmployed if the employee is already employed
        // new_map.set(employee.clone(), work_contract);

        // new_map.iter().for_each(|(_k, _v)| {
        //     // Here we can pay to all employees
        // }); 

        // let employees = new_map.get(employee.clone()).unwrap();

        set_employer(&e, employer, employer_struct);
        Ok(())
    }

    fn pay_employees(
        _e: Env,
        _employer: Address,
    ) -> Result<(), ContractError> {
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

    fn get_employer(e: Env, employer_address: Address) -> Employer {
        get_employer(&e, &employer_address)
    }


    fn asset(e: Env) -> Address {
        get_asset(&e)
    }
}
