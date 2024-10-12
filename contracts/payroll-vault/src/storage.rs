use soroban_sdk::{
    contracttype, Address, Env, Map
    //Vec
};

use crate::models::Employer;

// use crate::models::AssetAllocation;

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 120 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;


#[derive(Clone)]
#[contracttype] 
pub enum DataKey {
    Asset,
    Employer(Address),
    Balance(Address),
}

pub fn set_asset(e: &Env, asset: &Address) {
    e.storage().instance().set(&DataKey::Asset, asset);
}

pub fn get_asset(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Asset).unwrap()
}

pub fn has_asset(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Asset)
}

pub fn set_employer(e: &Env, employer_address: Address, employer: Employer) {
    let key = DataKey::Employer(employer_address);
    e.storage().instance().set(&key, &employer);
}

pub fn get_employer(e: &Env, employer_address: &Address) -> Employer {
    let key = DataKey::Employer(employer_address.clone());
    e.storage().instance().get(&key).unwrap_or(Employer{address: employer_address.clone(), employees: Map::new(e)})
}