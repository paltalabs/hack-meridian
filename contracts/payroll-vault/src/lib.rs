#![no_std]
use models::{Employer, PaymentPeriod, WorkContract};
use soroban_sdk::{
    contract, contractimpl, token::TokenClient, Address, Env, String, // Vec // Map, String,
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

use utils::{calculate_periods_since, calculate_periods_amounts_in_seconds};


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
        e: Env,
        employer: Address,
        amount: i128,
    ) -> Result<i128, ContractError> {
        check_nonnegative_amount(amount.clone());
        let employer_struct = get_employer(&e, &employer);
        employer_struct.address.require_auth();

        let employer_balance = read_balance(&e, employer.clone());

        let available_balance = employer_balance.checked_sub(employer_struct.total_liabilities)
            .ok_or(ContractError::InsufficientFunds)?;

        if available_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        TokenClient::new(&e, &get_asset(&e)).transfer(
            &e.current_contract_address(),
            &employer,
            &amount,
        );

        spend_balance(&e, employer, amount);

        Ok(amount)
    }
        
    fn employ(
        e: Env,
        employer: Address,
        employee: Address,
        name: String,
        payment_period: PaymentPeriod,
        salary: i128,
        notice_periods_required: u64, // Number of payment periods before the employee can be fired
        work_contract_document_hash: String,
    ) -> Result<(), ContractError> {
        let mut employer_struct = get_employer(&e, &employer);
    
        employer_struct.address.require_auth();
    
        if employer_struct.employees.contains_key(employee.clone()) {
            return Err(ContractError::AlreadyEmployed);
        }
    
        let employer_balance = read_balance(&e, employer.clone());
    
        let new_employee_liability = salary.checked_mul(notice_periods_required as i128)
            .ok_or(ContractError::IntegerOverflow)?;
    
        let available_balance = employer_balance.checked_sub(employer_struct.total_liabilities)
            .ok_or(ContractError::InsufficientFunds)?;
    
        if available_balance < new_employee_liability {
            return Err(ContractError::InsufficientFunds);
        }
    
        employer_struct.total_liabilities = employer_struct.total_liabilities.checked_add(new_employee_liability)
            .ok_or(ContractError::IntegerOverflow)?;
    
        let now = e.ledger().timestamp();
        // Create the new work contract
        let work_contract = WorkContract {
            employee: models::Employee {
                address: employee.clone(),
                name,
            },
            payment_period,
            notice_periods_required,
            salary,

            employment_start_date: now,
            employment_end_date: None,
            last_payment_date: now,

            notice_period_payments_made: 0,
            work_contract_document_hash,
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
        let asset = get_asset(&e);
        let current_timestamp = e.ledger().timestamp();
    
        for employee_address in employer_struct.employees.keys() {
            let mut work_contract = employer_struct.employees.get(employee_address.clone()).unwrap();

             // I should pay if the periods since fired is less than the notice periods required

             match work_contract.employment_end_date {
                Some(employment_end_date) if calculate_periods_since(
                    employment_end_date, 
                    current_timestamp, 
                    work_contract.payment_period,
                ) > work_contract.notice_periods_required => {
                    employer_struct.employees.remove(employee_address.clone());
                    continue; // this person has been fired
                }
                _ => {}
            };
            
            let periods_since_last_payment = calculate_periods_since(
                work_contract.last_payment_date,
                current_timestamp,
                work_contract.payment_period,
            );

            if periods_since_last_payment == 0 { //we skip if the employer does not have to pay the employee
                continue;
            }

            let salary_to_pay = work_contract.salary * periods_since_last_payment as i128;
            let employer_balance = read_balance(&e, employer.clone());
    
            if employer_balance < salary_to_pay {
                // TODO: this will fail and we cannot payu at least some employees
                // TODO make sure we can pay some employees
                return Err(ContractError::InsufficientFunds);
                // we should break and emit an event
                // we can pay something
            }

            // Deduct salary from employer's LOCAL balance
            spend_balance(&e, employer.clone(), salary_to_pay);
            
            // Transfer salary to employee
            TokenClient::new(&e, &asset).transfer(
                &e.current_contract_address(),
                &employee_address,
                &salary_to_pay,
            );

            // TODO This is bad if the payment is made days after the end of a period
            // TODO FIX THIS
            // last payment date + periods since last payment * payment period
            work_contract.last_payment_date = work_contract.last_payment_date + calculate_periods_amounts_in_seconds(
                periods_since_last_payment,
                work_contract.payment_period,
            );

            employer_struct.employees.set(employee_address.clone(), work_contract);
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
    
        match work_contract.employment_end_date {
            Some(_) => return Err(ContractError::EmployeeAlreadyFired),
            None => {
                work_contract.employment_end_date = Some(e.ledger().timestamp());
            }
        }
    
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

    fn get_employer(e: Env, employer_address: Address) -> Employer {
        get_employer(&e, &employer_address)
    }

    fn asset(e: Env) -> Address {
        get_asset(&e)
    }
}
