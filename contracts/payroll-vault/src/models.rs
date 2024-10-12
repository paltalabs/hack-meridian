use soroban_sdk::{
    contracttype, Address, String, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employer {
    pub address: Address,
    pub balance: i128,
    pub employees: Vec<WorkContract>,
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
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum PaymentPeriod {
    Weekly,
    Monthly,
    Annually,
}