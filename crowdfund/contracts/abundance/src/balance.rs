use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT};
use soroban_sdk::{Address, Env};

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::Balance(addr);
    e.storage().persistent().set(&key, &amount);
    e.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't receive when deauthorized");
    }
    write_balance(e, addr, balance + amount);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't spend when deauthorized");
    }
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, addr, balance - amount);
}

pub fn is_authorized(e: &Env, addr: Address) -> bool {
    let key = DataKey::State(addr);
    if let Some(state) = e.storage().persistent().get::<DataKey, bool>(&key) {
        state
    } else {
        true
    }
}

pub fn write_authorization(e: &Env, addr: Address, is_authorized: bool) {
    let key = DataKey::State(addr);
    e.storage().persistent().set(&key, &is_authorized);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());
    if !is_authorized(e, addr.clone()) {
        panic!("can't spend when deauthorized");
    }
    if balance < amount {
        panic!("insufficient balance");
    }
//QF
    pub fn has_balance_spender_role(e: &Env, user: &Address) -> bool {
        let key = DataKey::BalanceSpender(user.clone());
        e.storage().instance().get::<DataKey, bool>(&key).unwrap_or(false)
    }
//QF
    pub fn is_spender_authorized_for_campaign(
        e: &Env,
        spender: &Address,
        campaign_id: &Address,
    ) -> bool {
        // Get the address of the spender.
        let spender_address = spender.clone();
    
        // Get the address of the administrator of the campaign.
        let administrator = read_administrator(e);
    
        // If the spender is the administrator of the campaign, then they are authorized to spend the balance for the campaign.
        if spender_address == administrator {
            return true;
        }
    
        // Check if the spender has the BalanceSpender role.
        let has_balance_spender_role = has_balance_spender_role(e, spender);
        if has_balance_spender_role {
            return true;
        }
    
        return false;
    }
//QF
    // Check if the spender is authorized to spend the balance for the given campaign.
    let campaign_id = read_administrator(e);
    if !is_spender_authorized_for_campaign(e, &addr, &campaign_id) {
        panic!("spender is not authorized to spend the balance for the given campaign");
    }

    write_balance(e, addr, balance - amount);
}