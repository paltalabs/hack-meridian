#![no_std]
use models::{Employer, PaymentPeriod, WorkContract};
use soroban_sdk::{
    contract, contractimpl, token::TokenClient, Address, Env, String, Vec // Map, String,
};

mod error;
mod interface;
mod models;
mod storage;
mod test;
mod balance;
mod utils;

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

use utils::calculate_periods_since;


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
        
    fn employ(
        e: Env,
        employer: Address,
        employee: Address,
        name: String,
        payment_period: PaymentPeriod,
        salary: i128,
        notice_period: i128, // Number of payment periods before the employee can be fired
    ) -> Result<(), ContractError> {
        let mut employer_struct = get_employer(&e, &employer);
    
        employer_struct.address.require_auth();
    
        if employer_struct.employees.contains_key(employee.clone()) {
            return Err(ContractError::AlreadyEmployed);
        }
    
        let employer_balance = read_balance(&e, employer.clone());
    
        let new_employee_liability = salary.checked_mul(notice_period)
            .ok_or(ContractError::IntegerOverflow)?;
    
        let available_balance = employer_balance.checked_sub(employer_struct.total_liabilities)
            .ok_or(ContractError::InsufficientFunds)?;
    
        if available_balance < new_employee_liability {
            return Err(ContractError::InsufficientFunds);
        }
    
        employer_struct.total_liabilities = employer_struct.total_liabilities.checked_add(new_employee_liability)
            .ok_or(ContractError::IntegerOverflow)?;
    
        // Create the new work contract
        let work_contract = WorkContract {
            employee: models::Employee {
                address: employee.clone(),
                name,
            },
            payment_period,
            salary,
            notice_period,
            employed_at: e.ledger().timestamp(),
            is_active: true,
            unemployed_at: None,
            notice_period_payments_made: 0,
        };
    
        // Add the new employee to the employer's employees map
        employer_struct.employees.set(employee, work_contract);
    
        // Save the updated employer struct back to storage
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
    
        let employee_keys: Vec<Address> = employer_struct.employees.keys();
    
        for employee_address in employee_keys {
            let mut work_contract = employer_struct.employees.get(employee_address.clone()).unwrap();
    
            let salary = work_contract.salary;
            let employer_balance = read_balance(&e, employer.clone());
    
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
    
                    periods_since_fired < work_contract.notice_period
                } else {
                    false
                }
            };
    
            if should_pay {
                // Transfer salary to employee
                TokenClient::new(&e, &asset).transfer(
                    &e.current_contract_address(),
                    &employee_address,
                    &salary,
                );
    
                // Deduct salary from employer's balance
                spend_balance(&e, employer.clone(), salary);
    
                if !work_contract.is_active {
                    // Reduce total liabilities by the salary amount
                    employer_struct.total_liabilities = employer_struct.total_liabilities.checked_sub(salary)
                        .ok_or(ContractError::IntegerOverflow)?;
    
                    // Increase notice period payments made
                    work_contract.notice_period_payments_made += 1;
    
                    if work_contract.notice_period_payments_made >= work_contract.notice_period {
                        // Remove employee from the map
                        employer_struct.employees.remove(employee_address.clone());
                    } else {
                        // Update the work contract in the map
                        employer_struct.employees.set(employee_address.clone(), work_contract);
                    }
                } else {
                    // Active employee, total liabilities remain unchanged
                    employer_struct.employees.set(employee_address.clone(), work_contract);
                }
            }
        }
    
        set_employer(&e, employer, employer_struct);
        Ok(())
    }

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
    
        if !work_contract.is_active {
            return Err(ContractError::EmployeeAlreadyFired);
        }
    
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
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, employer)
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
