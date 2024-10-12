use soroban_sdk::{testutils::Address as _, Address, String};
use soroban_sdk::{testutils::{Ledger}};

use crate::test::PaymentVaultTest;

#[test]
fn test_pay_employees() {
    let test = PaymentVaultTest::setup();

    test.contract.initialize(&test.token.address);
    
    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 10_000_0_000_000;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    // Check new balances
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount);

    let init_time = 0;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    // Employ someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_000;
    let notice_periods_required = 3;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");

    test.contract.employ(
        &test.employer,
        &employee_0,
        &name,
        &crate::test::payroll_vault::PaymentPeriod::Monthly, 
        &salary, 
        &notice_periods_required,
        &work_contract_document_hash);
    
    test.contract.accept_work(
        &test.employer, 
        &employee_0,
        &true);
    
    // time has passed // 30 days has passed
    let new_time = 30 * 24 * 60 * 60 +1;
    test.env.ledger().with_mut(|li| {
        li.timestamp = new_time;
    });

    // Pay employees
    test.contract.pay_employees(&test.employer);
    
    // check new balances
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount - salary);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&employee_0), salary);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount - salary);

    // 65 days has passed
    let new_time = 65 * 24 * 60 * 60 +1;
    test.env.ledger().with_mut(|li| {
        li.timestamp = new_time;
    });

    // Pay employees
    test.contract.pay_employees(&test.employer);

    // check new balances
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount - 2 * salary);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&employee_0), 2 * salary);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount - 2 * salary);

    // 70   
    let new_time = 70 * 24 * 60 * 60 +1;
    test.env.ledger().with_mut(|li| {
        li.timestamp = new_time;
    });

    // I fire the employee now
    test.contract.fire(&test.employer, &employee_0);

    // the employee wants to get money the day 170
    // this means that it should get
    // 60 to 70 and then 2 more periods, to in total shpould receive more 3 salaries

    // 170
    let new_time = 170 * 24 * 60 * 60 +1;
    test.env.ledger().with_mut(|li| {
        li.timestamp = new_time;
    });

    // Pay employees
    test.contract.pay_employees(&test.employer);

    // check new balances
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount - 5 * salary);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&employee_0), 5 * salary);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount - 5 * salary);


}

// test with 2 employees with different salaries and periods types
#[test]
fn test_pay_employees_multiple_employees() {
    let test = PaymentVaultTest::setup();

    test.contract.initialize(&test.token.address);
    
    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 10_000_0_000_000;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    // Check new balances
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount);

    let init_time = 0;
    test.env.ledger().with_mut(|li| {
        li.timestamp = init_time;
    });

    // Employ someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_000;
    let notice_periods_required = 3;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");

    test.contract.employ(
        &test.employer,
        &employee_0,
        &name,
        &crate::test::payroll_vault::PaymentPeriod::Monthly, 
        &salary, 
        &notice_periods_required,
        &work_contract_document_hash);
    
    test.contract.accept_work(
        &test.employer, 
        &employee_0,
        &true);
    
    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary = 1_000_000;
    let notice_periods_required = 2;
    let name = String::from_str(&test.env, "Bob");
    let work_contract_document_hash = String::from_str(&test.env, "hash");

    test.contract.employ(
        &test.employer,
        &employee_1,
        &name,
        &crate::test::payroll_vault::PaymentPeriod::Weekly, 
        &salary, 
        &notice_periods_required,
        &work_contract_document_hash);

    test.contract.accept_work(
        &test.employer, 
        &employee_1,
        &true);
    
    // time has passed // 30 days has passed
    let new_time = 30 * 24 * 60 * 60 +1;
    test.env.ledger().with_mut(|li| {
        li.timestamp = new_time;
    });

    // Pay employees
    test.contract.pay_employees(&test.employer);

    // check new balances
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    
    // employee 0 gets 1 salary
    assert_eq!(test.token.balance(&employee_0), salary);
    // employee 1 gets 4 salaries (4 weeks)
    assert_eq!(test.token.balance(&employee_1), 4 * salary);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount - 5 * salary);
    
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount - 5 * salary);
    // 65 days has passed
    let new_time = 65 * 24 * 60 * 60 +1;
    test.env.ledger().with_mut(|li| {
        li.timestamp = new_time;
    });

    // Pay employees
    test.contract.pay_employees(&test.employer);
    //check new balances
    // employee 0 gets 1 salary more
    assert_eq!(test.token.balance(&employee_0), 2 * salary);
    // emplyee 1 gets 5 more salaries
    assert_eq!(test.token.balance(&employee_1), 9 * salary);

    // EMPLOYER has paid in total 6 + 5 = 11 salaries
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount - 11 * salary);
}

