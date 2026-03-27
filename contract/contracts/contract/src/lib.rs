#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Symbol, String, Address, Vec
};

#[contracttype]
#[derive(Clone)]
pub struct DataItem {
    pub owner: Address,
    pub data_hash: String, // IPFS hash or metadata
    pub price: i128,
}

#[contracttype]
pub enum DataKey {
    Item(u32),
    Counter,
    Purchased(Address, u32),
}

#[contract]
pub struct DataMarketplace;

#[contractimpl]
impl DataMarketplace {

    // Add new data
    pub fn add_data(env: Env, owner: Address, data_hash: String, price: i128) -> u32 {
        owner.require_auth();

        let mut counter: u32 = env.storage().instance().get(&DataKey::Counter).unwrap_or(0);
        counter += 1;

        let item = DataItem {
            owner: owner.clone(),
            data_hash,
            price,
        };

        env.storage().instance().set(&DataKey::Item(counter), &item);
        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    // Get data details
    pub fn get_data(env: Env, id: u32) -> DataItem {
        env.storage().instance().get(&DataKey::Item(id)).unwrap()
    }

    // Buy data access
    pub fn buy_data(env: Env, buyer: Address, id: u32) {
        buyer.require_auth();

        let item: DataItem = env.storage().instance().get(&DataKey::Item(id)).unwrap();

        // Mark as purchased
        env.storage().instance().set(&DataKey::Purchased(buyer.clone(), id), &true);

        // NOTE: Payment logic (token transfer) can be added later
    }

    // Check if user purchased data
    pub fn has_access(env: Env, user: Address, id: u32) -> bool {
        env.storage().instance()
            .get(&DataKey::Purchased(user, id))
            .unwrap_or(false)
    }
}