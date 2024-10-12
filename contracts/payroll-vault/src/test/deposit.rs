use crate::test::PaymentVaultTest;
use crate::test::payroll_vault::ContractError;

#[test]
fn test_deposit_employer_is_caller() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
    test.contract.initialize(&test.token.address);
    
    // Check initial balance
    assert_eq!(test.contract.balance(&test.employer), 0);
    let initial_employer_asset_balance = test.token.balance(&test.employer);
    assert_eq!(initial_employer_asset_balance, 1_000_000_000_000);
    assert_eq!(test.token.balance(&test.contract.address), 0);

    let deposit_amount = 1234567890;
    test.contract.deposit(&test.employer, &test.employer, &deposit_amount);

    // Check new balances
    assert_eq!(test.contract.balance(&test.employer), deposit_amount);
    assert_eq!(test.token.balance(&test.employer), initial_employer_asset_balance - deposit_amount);
    assert_eq!(test.token.balance(&test.contract.address), deposit_amount);
}
 

// todo, test when caller is not employer (POS machine)