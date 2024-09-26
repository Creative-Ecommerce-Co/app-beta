use near_sdk::AccountId;
use crate::models::{Product, Storefront};
use crate::storage::Storage;

pub fn migrate_product_schema(storage: &mut Storage, user_id: AccountId, product_id: String, new_product: Product) {
    if let Some(mut storefront) = storage.get_storefront(user_id.clone()) {
        if let Some(_) = storefront.get_product(product_id.clone()) {
            storefront.add_product(product_id.clone(), new_product);
            storage.add_storefront(user_id, storefront); // Update the storefront in storage
        } else {
            env::panic(b"Product does not exist");
        }
    } else {
        env::panic(b"Storefront does not exist");
    }
}

pub fn migrate_all_products(storage: &mut Storage, user_id: AccountId, new_product_schema: fn(Product) -> Product) {
    if let Some(mut storefront) = storage.get_storefront(user_id.clone()) {
        let product_ids: Vec<String> = storefront.get_all_product_ids();
        for product_id in product_ids {
            if let Some(product) = storefront.get_product(product_id.clone()) {
                let new_product = new_product_schema(product);
                storefront.add_product(product_id.clone(), new_product);
            }
        }
        storage.add_storefront(user_id, storefront); // Update the storefront in storage
    } else {
        env::panic(b"Storefront does not exist");
    }
}