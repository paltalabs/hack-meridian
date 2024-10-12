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
// employment_start_date is None until the employee accepts the contract.
// if the employee does noe accets the contracts, it gets removed

pub struct WorkContract {
    pub employee: Employee,

    pub payment_period: PaymentPeriod,
    pub notice_periods_required: u64,            // Reflects the number of payment periods required as notice    pub salary: i128,

    pub salary: i128,
    
    pub employment_start_date: Option<u64>,      // Clarified to indicate employment start date
    pub employment_end_date: Option<u64>,        // Renamed for employment end date (if applicable)
    pub last_payment_date: Option<u64>,                  // More explicit name for last payment date
    
    pub work_contract_document_hash: String,
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