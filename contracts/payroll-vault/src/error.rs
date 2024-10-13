use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization Errors (10x)
    NotInitialized = 100,
    AlreadyInitialized = 101,
    AlreadyEmployed = 102,
    InsufficientFunds = 103,
    EmployeeNotFound = 104,
    IntegerOverflow = 105,
    EmployeeAlreadyFired = 106,
    WorkContractNotFound = 107,
}