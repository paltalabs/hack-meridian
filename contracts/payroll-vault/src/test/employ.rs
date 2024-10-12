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

    // Employ someone
    let employee_0 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_period = 1;
    let name = String::from_str(&test.env, "Alice");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);
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
    let notice_period = 1;
    let name = String::from_str(&test.env, "Alice");
    let result = test.contract.try_employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);

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
    let notice_period = 1;
    let name = String::from_str(&test.env, "Alice");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);

    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_period = 2;
    let name = String::from_str(&test.env, "Bob");
    test.contract.employ(&test.employer, &employee_1, &name, &crate::test::payroll_vault::PaymentPeriod::Weekly, &salary, &notice_period);

    // Employ someone
    let employee_2 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_period = 1;
    let name = String::from_str(&test.env, "John");
    test.contract.employ(&test.employer, &employee_2, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);
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
    let notice_period = 1;
    let name = String::from_str(&test.env, "Alice");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);

    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_period = 2;
    let name = String::from_str(&test.env, "Bob");
    test.contract.employ(&test.employer, &employee_1, &name, &crate::test::payroll_vault::PaymentPeriod::Weekly, &salary, &notice_period);

    // Employ someone
    let employee_2 = Address::generate(&test.env);
    let salary = 1_000_0_000_000;
    let notice_period = 1;
    let name = String::from_str(&test.env, "John");
    test.contract.employ(&test.employer, &employee_2, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);

    // Employ someone without enough balance
    let employee_3 = Address::generate(&test.env);
    let salary = 2_000_0_000_000;
    let notice_period = 2;
    let name = String::from_str(&test.env, "Elba");
    let result = test.contract.try_employ(&test.employer, &employee_3, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary, &notice_period);

    assert_eq!(result, Err(Ok(crate::test::payroll_vault::ContractError::InsufficientFunds)));
}