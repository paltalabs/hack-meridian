use crate::test::PaymentVaultTest;
use crate::test::payroll_vault::ContractError;

#[test]
fn test_initialize() {
    let test = PaymentVaultTest::setup();

    // Initialize with token address
    test.contract.initialize(&test.token.address);
    
    // Check that the token address is set
    assert_eq!(test.contract.asset(), test.token.address);

    // Cannot initialize again

    let result = test.contract.try_initialize(&test.token.address);
    assert_eq!(result, Err(Ok(ContractError::AlreadyInitialized)));
}
 