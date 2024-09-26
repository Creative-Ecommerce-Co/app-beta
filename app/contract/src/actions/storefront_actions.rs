use near_sdk::{env, AccountId};
use crate::models::Storefront;
use crate::storage::Storage;

pub fn add_storefront(storage: &mut Storage, user_id: AccountId, storefront: Storefront) {
    if storage.get_storefront(user_id.clone()).is_none() {
        storage.add_storefront(user_id, storefront);
    } else {
        env::panic(b"Storefront already exists for this user");
    }
}

pub fn update_storefront(storage: &mut Storage, user_id: AccountId, storefront: Storefront) {
    if storage.get_storefront(user_id.clone()).is_some() {
        storage.add_storefront(user_id, storefront);
    } else {
        env::panic(b"Storefront does not exist for this user");
    }
}

pub fn get_storefront(storage: &Storage, user_id: AccountId) -> Option<Storefront> {
    storage.get_storefront(user_id)
}

pub fn remove_storefront(storage: &mut Storage, user_id: AccountId) {
    if storage.get_storefront(user_id.clone()).is_some() {
        storage.remove_storefront(user_id);
    } else {
        env::panic(b"Storefront does not exist for this user");
    }
}

pub fn list_storefronts(storage: &Storage) -> Vec<Storefront> {
    storage.get_all_storefronts()
}