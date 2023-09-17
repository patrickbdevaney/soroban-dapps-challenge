use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey};
use soroban_sdk::{Address, Env};

pub fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = e.storage().temporary().get::<_, AllowanceValue>(&key) {
        if allowance.expiration_ledger < e.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    if amount > 0 && expiration_ledger < e.ledger().sequence() {
        panic!("expiration_ledger is less than ledger seq when amount > 0")
    }

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &allowance);

    if amount > 0 {
        e.storage().temporary().bump(
            &key,
            expiration_ledger
                .checked_sub(e.ledger().sequence())
                .unwrap(),
        )
    }
}

pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }
    write_allowance(
        e,
        from,
        spender,
        allowance.amount - amount,
        allowance.expiration_ledger,
    );
}
//QF
pub fn has_allowance_spender_role(e: &Env, user: &Address) -> bool {
    let key = DataKey::AllowanceSpender(user.clone());
    e.storage().instance().has(&key)
}
pub fn is_spender_authorized(e: &Env, spender: &Address, campaign_id: &Address) -> bool {

    // Check if the spender is the administrator of the campaign.
    if spender == read_administrator(e) {
        return true;
    }

    // Check if the spender has the AllowanceSpender role.
    if has_allowance_spender_role(e, spender) {
        return true;
    }

}