use near_sdk::{env, AccountId};
use crate::models::{Storefront, UserSubscription};
use crate::storage::Storage;

pub fn add_admin(storage: &mut Storage, admin_id: AccountId) {
    storage.add_admin(admin_id);
}

pub fn remove_admin(storage: &mut Storage, admin_id: AccountId) {
    storage.remove_admin(admin_id);
}

pub fn is_admin(storage: &Storage, admin_id: AccountId) -> bool {
    storage.is_admin(admin_id)
}

pub fn list_all_storefronts(storage: &Storage) -> Vec<Storefront> {
    storage.get_all_storefronts()
}

pub fn list_all_users(storage: &Storage) -> Vec<AccountId> {
    storage.get_all_users()
}

pub fn list_all_subscriptions(storage: &Storage) -> Vec<UserSubscription> {
    storage.get_all_subscriptions()
}