use near_sdk::{env, Promise};
use crate::models::{Product, Storefront};
use crate::storage::Storage;

pub fn add_product(storage: &mut Storage, user_id: AccountId, product_id: String, product: Product) {
    if let Some(mut storefront) = storage.get_storefront(user_id.clone()) {
        storefront.add_product(product_id.clone(), product);
        storage.add_storefront(user_id, storefront); // Update the storefront in storage
    } else {
        env::panic(b"Storefront does not exist");
    }
}

pub fn update_product(storage: &mut Storage, user_id: AccountId, product_id: String, product: Product) {
    if let Some(mut storefront) = storage.get_storefront(user_id.clone()) {
        storefront.update_product(product_id.clone(), product);
        storage.add_storefront(user_id, storefront); // Update the storefront in storage
    } else {
        env::panic(b"Storefront does not exist");
    }
}

pub fn get_product(storage: &Storage, user_id: AccountId, product_id: String) -> Option<Product> {
    if let Some(storefront) = storage.get_storefront(user_id) {
        return storefront.get_product(product_id);
    }
    None
}

pub fn remove_product(storage: &mut Storage, user_id: AccountId, product_id: String) {
    if let Some(mut storefront) = storage.get_storefront(user_id.clone()) {
        storefront.remove_product(product_id.clone());
        storage.add_storefront(user_id, storefront); // Update the storefront in storage
    } else {
        env::panic(b"Storefront does not exist");
    }
}