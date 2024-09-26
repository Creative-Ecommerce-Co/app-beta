use near_sdk::AccountId;
use crate::models::Storefront;
use crate::storage::Storage;

pub fn migrate_storefront_schema(storage: &mut Storage, user_id: AccountId, new_storefront: Storefront) {
    if let Some(_) = storage.get_storefront(user_id.clone()) {
        storage.add_storefront(user_id, new_storefront);
    } else {
        env::panic(b"Storefront does not exist");
    }
}

pub fn migrate_all_storefronts(storage: &mut Storage, new_storefront_schema: fn(Storefront) -> Storefront) {
    let user_ids: Vec<AccountId> = storage.get_all_users();
    for user_id in user_ids {
        if let Some(storefront) = storage.get_storefront(user_id.clone()) {
            let new_storefront = new_storefront_schema(storefront);
            storage.add_storefront(user_id, new_storefront);
        }
    }
}