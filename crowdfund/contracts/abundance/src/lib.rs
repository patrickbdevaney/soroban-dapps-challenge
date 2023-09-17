#![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod event;
mod metadata;
mod storage_types;
mod test;

pub use crate::contract::TokenClient;

use crate::pledge::{pledge, Pledge};

pub fn pledge_tokens(e: Env, user: Address, campaign_id: Address, amount: i128) -> Pledge {
    let pledge = pledge(e, user, campaign_id, amount);

    // Emit a pledge created event.
    event::pledge_created(e, pledge);

    pledge
}

use crate::campaign::{create_campaign, Campaign};

pub fn create_new_campaign(e: Env, name: String, description: String, goal: i128) -> Campaign {
    let campaign = create_campaign(e, name, description, goal);

    // Emit a campaign created event.
    event::campaign_created(e, campaign);

    campaign
}