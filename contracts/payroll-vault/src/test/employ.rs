use soroban_sdk::{testutils::Address as _, Address, String};

use crate::test::PaymentVaultTest;

#[test]
fn test_employ() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
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

    // Propose some employment someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    // check that we get the information of the employee
    let employer_struct = test.contract.get_employer(&test.employer);
    assert!(employer_struct.employees.contains_key(employee_0.clone()));
    let work_contract = employer_struct.employees.get(employee_0.clone()).unwrap();
    assert_eq!(work_contract.salary, salary);
    assert_eq!(work_contract.notice_periods_required, notice_periods_required);
    assert_eq!(work_contract.employment_start_date, None);
    assert_eq!(work_contract.employment_end_date, None);
    assert_eq!(work_contract.last_payment_date, None);

    // now the employee should accept the work
    test.contract.accept_work(
        &test.employer, 
        &employee_0,
        &true);

    // check dates
    let employer_struct = test.contract.get_employer(&test.employer);
    let work_contract = employer_struct.employees.get(employee_0.clone()).unwrap();
    assert_eq!(work_contract.employment_start_date, Some(0));
    assert_eq!(work_contract.employment_end_date, None);
    assert_eq!(work_contract.last_payment_date, Some(0));

}

// test employ employee does not accetps
#[test]
fn test_employ_employee_does_not_accept() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
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

    // Propose some employment to an employee
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    // Check that the employee has not accepted the work yet
    let employer_struct = test.contract.get_employer(&test.employer);
    let work_contract = employer_struct.employees.get(employee_0.clone()).unwrap();
    assert_eq!(work_contract.employment_start_date, None);
    assert_eq!(work_contract.employment_end_date, None);
    assert_eq!(work_contract.last_payment_date, None);

    // the employee should not accept the work
    test.contract.accept_work(
        &test.employer, 
        &employee_0,
        &false);
    
    // check that the work does not ecxists anymore
    let employer_struct = test.contract.get_employer(&test.employer);
    assert!(!employer_struct.employees.contains_key(employee_0.clone()));
    
}

#[test]
fn test_employ_without_deposit() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
    test.contract.initialize(&test.token.address);
    
    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    // Employ someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    let result = test.contract.try_employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    assert_eq!(result, Err(Ok(crate::test::payroll_vault::ContractError::InsufficientFunds)));
}

#[test]
fn test_employ_multiple_employees() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
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

    // Employ someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 2;
    let name = String::from_str(&test.env, "Bob");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_1, &name, &crate::test::payroll_vault::PaymentPeriod::Weekly, &salary, &notice_periods_required, &work_contract_document_hash);

    // Employ someone
    let employee_2 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "John");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_2, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);
}

#[test]
fn test_employ_multiple_employees_insufficient_balance() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
    test.contract.initialize(&test.token.address);
    
    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 5_000_0_000_000;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    // Check new balances
    assert_eq!(test.contract.employer_balance(&test.employer), deposit_amount);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount);

    // Employ someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "Alice");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 2;
    let name = String::from_str(&test.env, "Bob");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_1, &name, &crate::test::payroll_vault::PaymentPeriod::Weekly, &salary, &notice_periods_required, &work_contract_document_hash);

    // Employ someone
    let employee_2 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_periods_required = 1;
    let name = String::from_str(&test.env, "John");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    test.contract.employ(&test.employer, &employee_2, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    // Employ someone without enough balance
    let employee_3 = Address::generate(&test.env);
    let salary = 2_000_0_000_000;
    let notice_periods_required = 2;
    let name = String::from_str(&test.env, "Elba");
    let work_contract_document_hash = String::from_str(&test.env, "hash");
    let result = test.contract.try_employ(&test.employer, &employee_3, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_periods_required, &work_contract_document_hash);

    assert_eq!(result, Err(Ok(crate::test::payroll_vault::ContractError::InsufficientFunds)));
}