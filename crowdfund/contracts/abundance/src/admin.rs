use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}
//QF
pub fn read_administrator(e: &Env) -> (Address, f32) {
    let key = DataKey::Admin;
    let admin_address = e.storage().instance().get(&key).unwrap();

    // Get the matching rate for the campaign, if it exists.
    let key = DataKey::MatchingRate(admin_address.clone());
    let matching_rate = e.storage().instance().get(&key).unwrap_or(0.0);

    return (admin_address, matching_rate);
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}
//QF
pub fn set_matching_rate(e: &Env, campaign_id: &Address, matching_rate: f32) {
        // Get the address of the caller.
        let caller_address = e.current_contract_address();

        // Check if the caller is the administrator of the campaign.
        let administrator = read_administrator(e);
        if administrator != caller_address {
            panic!("Only the administrator can set the matching rate for a campaign.");
        }
    
    // Store the matching rate for the campaign.
    let key = DataKey::MatchingRate(campaign_id.clone());
    e.storage().instance().set(&key, &matching_rate);
}
//QF
pub fn set_allowance_spender(e: &Env, user: &Address) {
    let key = DataKey::AllowanceSpender(user.clone());
    e.storage().instance().set(&key, &true);
}