use soroban_sdk::{Address, Env};

use crate::storage::{
    DataKey, 
    BALANCE_BUMP_AMOUNT, 
    BALANCE_LIFETIME_THRESHOLD};

pub fn read_lender_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::LenderBalance(addr);
    if let Some(lender_balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        lender_balance
    } else {
        0
    }
}

fn write_lender_balance(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::LenderBalance(addr);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn receive_lender_balance(e: &Env, addr: Address, amount: i128) {
    let lender_balance = read_lender_balance(e, addr.clone());

    let new_lender_balance = lender_balance.checked_add(amount)
        .expect("Integer overflow occurred while adding lender_balance.");

    write_lender_balance(e, addr, new_lender_balance);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let lender_balance = read_lender_balance(e, addr.clone());
    if lender_balance < amount {
        panic!("insufficient lender_balance");
    }
    write_lender_balance(e, addr, lender_balance - amount);
}
