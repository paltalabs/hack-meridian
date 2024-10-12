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
mod utils;

use interface::VaultTrait;
pub use error::ContractError;

use storage::{
    get_asset, get_employer, has_asset, set_asset, set_employer};
use utils::calculate_periods_since;

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
            employed_at: e.ledger().timestamp(),
            is_active: true,
            unemployed_at: None,
            notice_period_payments_made: 0,
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
    
        let current_timestamp = e.ledger().timestamp();
    
        for (employee_address, mut work_contract) in employer_struct.employees.iter() {
            let salary = work_contract.salary;
            let employer_balance = employer_struct.balance;
    
            if employer_balance < salary {
                return Err(ContractError::InsufficientFunds);
            }
    
            let should_pay = if work_contract.is_active {
                true
            } else {
                if let Some(unemployed_at) = work_contract.unemployed_at {
                    let periods_since_fired = calculate_periods_since(
                        unemployed_at,
                        current_timestamp,
                        work_contract.payment_period,
                    );
    
                    if periods_since_fired < work_contract.notice_period {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
    
            if should_pay {
                TokenClient::new(&e, &asset).transfer(
                    &e.current_contract_address(),
                    &employee_address,
                    &salary,
                );
    
                employer_struct.balance -= salary;
    
                if !work_contract.is_active {
                    work_contract.notice_period_payments_made += 1;
    
                    if work_contract.notice_period_payments_made >= work_contract.notice_period {
                        employer_struct.employees.remove(employee_address.clone());
                    } else {
                        employer_struct
                            .employees
                            .set(employee_address.clone(), work_contract);
                    }
                } else {
                    employer_struct
                        .employees
                        .set(employee_address.clone(), work_contract);
                }
            }
        }
    
        set_employer(&e, employer, employer_struct);
        Ok(())
    }

    // this can only be done by the employer
    fn fire(
        e: Env,
        employer: Address,
        employee: Address,
    ) -> Result<(), ContractError> {
        let mut employer_struct = get_employer(&e, &employer);
        employer_struct.address.require_auth();
    
        let mut work_contract = employer_struct
            .employees
            .get(employee.clone())
            .ok_or(ContractError::EmployeeNotFound)?;
    
        work_contract.is_active = false;
        work_contract.unemployed_at = Some(e.ledger().timestamp());
        work_contract.notice_period_payments_made = 0;
    
        employer_struct
            .employees
            .set(employee.clone(), work_contract);
    
        set_employer(&e, employer, employer_struct);
        Ok(())
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
