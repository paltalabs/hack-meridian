use soroban_sdk::{
    contracttype,
    Address,
    Env, Vec,
    //Vec
};

use crate::models::Employer;

// use crate::models::AssetAllocation;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Asset,
    Employer(Address),
}

pub fn set_asset(e: &Env, asset: &Address) {
    e.storage().instance().set(&DataKey::Asset, asset);
}

// pub fn get_asset(e: &Env) -> Address {
//     e.storage().instance().get(&DataKey::Asset).unwrap()
// }

pub fn has_asset(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Asset)
}

pub fn set_employer(e: &Env, employer_address: Address, employer: Employer) {
    let key = DataKey::Employer(employer_address);
    e.storage().instance().set(&key, &employer);
}

pub fn get_employer(e: &Env, employer_address: &Address) -> Employer {
    let key = DataKey::Employer(employer_address.clone());
    e.storage().instance().get(&key).unwrap_or(Employer{address: employer_address.clone(), balance: 0, employees: Vec::new(e)})
}