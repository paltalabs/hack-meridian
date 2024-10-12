use soroban_sdk::{
    contracttype, Address, Map, String
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employer {
    pub address: Address,
    pub employees: Map<Address, WorkContract>,
    pub name: String,
    pub total_liabilities: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkContract {
    pub employee: Employee,
    pub payment_period: PaymentPeriod,
    pub salary: i128,
    pub notice_period: i128,
    pub employed_at: u64,
    pub is_active: bool,
    pub unemployed_at: Option<u64>,
    pub notice_period_payments_made: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    pub address: Address,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[contracttype]
pub enum PaymentPeriod {
    Weekly,
    Monthly,
    Annually,
}