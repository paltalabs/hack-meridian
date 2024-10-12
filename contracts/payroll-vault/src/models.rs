use soroban_sdk::{
    contracttype, Address, Map
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employer {
    pub address: Address,
    pub balance: i128,
    pub employees: Map<Address, WorkContract>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkContract {
    pub employee: Employee,
    pub payment_period: PaymentPeriod,
    pub salary: i128,
    pub notice_period: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    pub address: Address,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum PaymentPeriod {
    Weekly,
    Monthly,
    Annually,
}