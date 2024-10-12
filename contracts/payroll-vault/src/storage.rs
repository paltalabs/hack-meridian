use soroban_sdk::{
    contracttype,
    Address,
    Env,
    //Vec
};

// use crate::models::AssetAllocation;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Asset,
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