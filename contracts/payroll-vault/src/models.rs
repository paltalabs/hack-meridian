use soroban_sdk::{contracttype, Address, Map, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employer {
    pub employee: Map<Address, Employee>,
    pub name: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    pub name: String,
    pub paused: bool,
    pub ratio: i128,
}