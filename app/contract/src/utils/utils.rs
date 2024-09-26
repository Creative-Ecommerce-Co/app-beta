use near_sdk::AccountId;
use crate::models::{Product, Storefront};
use crate::storage::Storage;

/// Utility function to check if a user has a storefront.
pub fn user_has_storefront(storage: &Storage, user_id: &AccountId) -> bool {
    storage.get_storefront(user_id.clone()).is_some()
}

/// Utility function to check if a product exists in a user's storefront.
pub fn product_exists(storage: &Storage, user_id: &AccountId, product_id: &String) -> bool {
    if let Some(storefront) = storage.get_storefront(user_id.clone()) {
        return storefront.get_product(product_id.clone()).is_some();
    }
    false
}

/// Utility function to get all products from all storefronts.
pub fn get_all_products(storage: &Storage) -> Vec<Product> {
    let mut all_products = Vec::new();
    let user_ids = storage.get_all_users();
    for user_id in user_ids {
        if let Some(storefront) = storage.get_storefront(user_id) {
            all_products.extend(storefront.get_all_products());
        }
    }
    all_products
}

/// Utility function to get all storefronts.
pub fn get_all_storefronts(storage: &Storage) -> Vec<Storefront> {
    storage.get_all_storefronts()
}