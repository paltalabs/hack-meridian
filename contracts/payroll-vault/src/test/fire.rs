use soroban_sdk::{
    testutils::Address as _, Address, String
};

use crate::test::{payroll_vault::PaymentPeriod, PaymentVaultTest};
use crate::test::payroll_vault::ContractError;

#[test]
fn test_fire_employee_successful() {
    let test = PaymentVaultTest::setup();

    // Initialize the contract with token address
    test.contract.initialize(&test.token.address);

    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 10_000_0_000_000;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    // Employer employs an employee
    let employee = Address::generate(&test.env);

    let salary = 1000i128;
    let notice_period = 2u64;
    let payment_period = PaymentPeriod::Monthly;
    let employee_name = String::from_str(&test.env, "Alice");

    test.contract.employ(
        &test.employer,
        &employee,
        &employee_name,
        &payment_period,
        &salary,
        &notice_period,
    );

    // Verify that the employee is employed
    let employer_struct = test.contract.get_employer(&test.employer);
    assert!(employer_struct.employees.contains_key(employee.clone()));

    // Fire the employee
    test.contract.fire(&test.employer, &employee);

    // Verify that the employee's work contract is updated
    let updated_employer_struct = test.contract.get_employer(&test.employer);
    let work_contract = updated_employer_struct
        .employees
        .get(employee.clone())
        .unwrap();
    assert_eq!(work_contract.is_active, false);
    assert!(work_contract.employment_end_date.is_some());
    assert_eq!(work_contract.notice_period_payments_made, 0);
}

#[test]
fn test_fire_employee_not_found() {
    let test = PaymentVaultTest::setup();

    // Initialize the contract with token address
    test.contract.initialize(&test.token.address);

    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 10_000_0_000_000;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    let employee = Address::generate(&test.env);

    // Attempt to fire an employee who was never employed
    let result = test.contract.try_fire(
        &test.employer,
        &employee,
    );

    // Verify that the error is EmployeeNotFound
    assert_eq!(result, Err(Ok(ContractError::EmployeeNotFound)));
}

#[test]
fn test_fire_employee_already_fired() {
    let test = PaymentVaultTest::setup();

    // Initialize the contract with token address
    test.contract.initialize(&test.token.address);

    // Check initial balance
    assert_eq!(test.contract.employer_balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 100_000_0_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 10_000_0_000_000;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    // Employer employs an employee
    let employee = Address::generate(&test.env);
    let salary = 1000i128;
    let notice_period = 2u64;
    let payment_period = PaymentPeriod::Monthly;
    let employee_name = String::from_str(&test.env, "Bob");

    test.contract.employ(
        &test.employer,
        &employee,
        &employee_name,
        &payment_period,
        &salary,
        &notice_period,
    );

    // Fire the employee
    test.contract.fire(&test.employer, &employee);

    // Attempt to fire the employee again
    let result = test.contract.try_fire(
        &test.employer,
        &employee,
    );

    // Verify that the error is EmployeeAlreadyFired
    assert_eq!(result, Err(Ok(ContractError::EmployeeAlreadyFired)));
}