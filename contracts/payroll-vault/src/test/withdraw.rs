use soroban_sdk::{testutils:: Address as _, Address, String};

use crate::test::{payroll_vault::ContractError, PaymentVaultTest};
 
#[test]
fn test_withdraw_success() {
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
    let salary_0 = 1_000_0_000_000;
    let notice_periods_required_0 = 1u64;
    let name = String::from_str(&test.env, "Alice");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary_0, &notice_periods_required_0);

    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary_1 = 1_000_0_000_000;
    let notice_periods_required_1 = 2u64;
    let name = String::from_str(&test.env, "Bob");
    test.contract.employ(&test.employer, &employee_1, &name, &crate::test::payroll_vault::PaymentPeriod::Weekly, &salary_1, &notice_periods_required_1);


    // Withdraw Available Balance
    let expected_available_balance = deposit_amount - (salary_0 * notice_periods_required_0 as i128) - (salary_1 * notice_periods_required_1 as i128);

    test.contract.withdraw(&test.employer, &expected_available_balance);
    assert_eq!(test.token.balance(&test.contract.address), (salary_0 * notice_periods_required_0 as i128) + (salary_1 * notice_periods_required_1 as i128));
}
 
#[test]
fn test_withdraw_more_than_available() {
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
    let salary_0 = 1_000_0_000_000;
    let notice_periods_required_0 = 1u64;
    let name = String::from_str(&test.env, "Alice");
    test.contract.employ(&test.employer, &employee_0, &name, &crate::test::payroll_vault::PaymentPeriod::Monthly, &salary_0, &notice_periods_required_0);

    // Employ someone
    let employee_1 = Address::generate(&test.env);
    let salary_1 = 1_000_0_000_000;
    let notice_periods_required_1 = 2u64;
    let name = String::from_str(&test.env, "Bob");
    test.contract.employ(&test.employer, &employee_1, &name, &crate::test::payroll_vault::PaymentPeriod::Weekly, &salary_1, &notice_periods_required_1);


    // Withdraw Available Balance
    let expected_available_balance = deposit_amount - (salary_0 * notice_periods_required_0 as i128) - (salary_1 * notice_periods_required_1 as i128);

    let result = test.contract.try_withdraw(&test.employer, &(expected_available_balance + 1000));
    assert_eq!(result, Err(Ok(ContractError::InsufficientFunds)));
}

#[test]
fn test_withdraw_no_employees() {
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

    test.contract.withdraw(&test.employer, &deposit_amount);
    assert_eq!(test.token.balance(&test.contract.address), 0);
}